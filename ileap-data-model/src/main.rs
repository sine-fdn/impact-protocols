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
use schemars::schema::{ObjectValidation, Schema, SchemaObject};
use schemars::{schema_for, Map, Set};
use serde::de::DeserializeOwned;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    generate_schema::<ShipmentFootprint>()?;
    generate_schema::<Toc>()?;
    generate_schema::<Tad>()?;
    generate_schema::<Hoc>()?;

    Ok(())
}

fn generate_schema<T: schemars::JsonSchema + DeserializeOwned>() -> Result<(), Error> {
    let type_name = std::any::type_name::<T>();
    let type_name = type_name.split("::").collect::<Vec<&str>>();
    let type_name = type_name.last().unwrap_or(&"Could not parse type name");

    let regex = Regex::new(r"([^A-Z])([A-Z])").unwrap();

    let schema_name = regex
        .replace_all(type_name, "$1-$2")
        .into_owned()
        .to_lowercase();

    let schema = schema_for!(T);

    let schema_json = to_string_pretty(&schema)
        .unwrap_or_else(|_| panic!("Failed to serialize {type_name} schema"));

    let mut schema_file = File::create(format!("./ileap-data-model/schemas/{schema_name}.json"))?;

    schema_file.write_all(schema_json.as_bytes())?;

    println!("{schema_name}.json successfully created");

    // let mut pcf_schema = schema_for!(ProductFootprint<T>);

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

    let pcf_schema_json = to_string_pretty(&pcf_schema)
        .unwrap_or_else(|_| panic!("Failed to serialize pcf-{type_name} schema"));

    let mut pcf_schema_file =
        File::create(format!("./ileap-data-model/schemas/pcf-{schema_name}.json"))?;

    pcf_schema_file.write_all(pcf_schema_json.as_bytes())?;

    Ok(())
}
