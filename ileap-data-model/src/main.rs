/*
 * Copyright (c) 2024 Martin Pompéry
 * Copyright (c) 2024 SINE Foundation e.V.
 *
 * This software is released under the MIT License, see LICENSE.
 */

use ileap_data_model::*;
use pact_data_model::ProductFootprint;
use regex::Regex;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::schema::{ObjectValidation, RootSchema, Schema, SchemaObject};
use schemars::{schema_for, Map, Set};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    generate_schemas::<ShipmentFootprint>()?;
    generate_schemas::<Toc>()?;
    generate_schemas::<Tad>()?;
    generate_schemas::<Hoc>()?;

    Ok(())
}

fn generate_schemas<T: schemars::JsonSchema>() -> Result<(), Error> {
    let type_name = get_type_name::<T>()?;

    let schema = schema_for!(T);

    write_schema_file(schema, &type_name)?;

    let pcf_schema = embedded_schema::<T>()?;

    write_schema_file(pcf_schema, &format!("pcf-{type_name}"))?;

    Ok(())
}

fn embedded_schema<T: schemars::JsonSchema>() -> Result<RootSchema, Error> {
    let type_name = get_type_name::<T>()?;

    let settings = SchemaSettings::default();

    let mut schema_generator = SchemaGenerator::new(settings);

    schema_generator.subschema_for::<T>();

    let dme_schema = SchemaObject {
        instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
            schemars::schema::InstanceType::Object,
        ))),
        object: Some(Box::new(ObjectValidation {
            required: Set::from([
                "data".to_string(),
                "dataSchema".to_string(),
                "specVersion".to_string(),
            ]),
            properties: Map::from([
                (
                    "dataSchema".to_string(),
                    Schema::Object(SchemaObject {
                        instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                            schemars::schema::InstanceType::String,
                        ))),
                        ..Default::default()
                    }),
                ),
                (
                    "documentation".to_string(),
                    Schema::Object(SchemaObject {
                        instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                            schemars::schema::InstanceType::String,
                        ))),
                        ..Default::default()
                    }),
                ),
                (
                    "specVersion".to_string(),
                    Schema::Object(SchemaObject {
                        instance_type: None,
                        reference: Some("#/definitions/VersionString".to_string()),
                        ..Default::default()
                    }),
                ),
                (
                    "data".to_string(),
                    Schema::Object(SchemaObject {
                        instance_type: None,
                        reference: Some(format!("#/definitions/{}", type_name)),
                        ..Default::default()
                    }),
                ),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    };

    let mut pcf_schema = schema_generator.root_schema_for::<ProductFootprint<T>>();

    pcf_schema
        .definitions
        .insert("DataModelExtension".to_string(), Schema::Object(dme_schema));

    if let Some(ref mut metadata) = pcf_schema.schema.metadata {
        metadata.title = Some(format!("ProductFootprint_with_{}_Extension", type_name));
        metadata.description = Some(format!("PData Type \"ProductFootprint\" of PACT Tech Spec Version 2 with {} as a DataModelExtension", type_name));
    };

    Ok(pcf_schema)
}

fn get_type_name<T: schemars::JsonSchema>() -> Result<String, Error> {
    let type_name = std::any::type_name::<T>();
    let type_name = type_name.split("::").collect::<Vec<&str>>();
    let type_name = type_name.last().unwrap_or(&"Could not parse type name");

    Ok(type_name.to_string())
}

fn write_schema_file(schema: RootSchema, type_name: &str) -> Result<(), Error> {
    let schema_json = to_string_pretty(&schema)
        .unwrap_or_else(|_| panic!("Failed to serialize {type_name} schema"));

    let regex = Regex::new(r"([^A-Z])([A-Z])").unwrap();

    let schema_name = regex
        .replace_all(&type_name, "$1-$2")
        .into_owned()
        .to_lowercase();

    let mut schema_file = File::create(format!("./ileap-data-model/schemas/{schema_name}.json"))?;

    schema_file.write_all(schema_json.as_bytes())?;

    println!("{schema_name}.json successfully created");

    Ok(())
}
