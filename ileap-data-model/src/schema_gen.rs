/*
 * Copyright (c) 2024 Martin Pompéry
 * Copyright (c) 2024 SINE Foundation e.V.
 *
 * This software is released under the MIT License, see LICENSE.
 */

use pact_data_model::ProductFootprint;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::schema::{ObjectValidation, RootSchema, Schema, SchemaObject};
use schemars::{schema_for, Map, Set};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};

pub fn write_schemas<T: schemars::JsonSchema>(
    type_name: &str,
    schema_file_name: &str,
    pcf_schema_file_name: &str,
) -> Result<(), Error> {
    let schema = schema_for!(T);

    write_schema_file(schema, schema_file_name)?;

    let pcf_schema = gen_pcf_with_extension::<T>(type_name);

    write_schema_file(pcf_schema, pcf_schema_file_name)?;

    Ok(())
}

pub fn gen_pcf_with_extension<T: schemars::JsonSchema>(type_name: &str) -> RootSchema {
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
        metadata.description = Some(format!("Data Type \"ProductFootprint\" of PACT Tech Spec Version 2 with {} as a DataModelExtension", type_name));
    };

    pcf_schema
}

fn write_schema_file(schema: RootSchema, schema_name: &str) -> Result<(), Error> {
    let schema_json = to_string_pretty(&schema)
        .unwrap_or_else(|_| panic!("Failed to serialize schema: {schema:?}"));

    let mut schema_file = File::create(format!("./ileap-data-model/schemas/{schema_name}.json"))?;

    schema_file.write_all(schema_json.as_bytes())?;

    println!("{schema_name}.json successfully created");

    Ok(())
}

#[test]
fn compare_schemas() {
    use crate::{Hoc, ShipmentFootprint, Tad, Toc};
    use schemars::schema_for;
    use serde_json::to_string_pretty;
    use serde_json::Value;
    use std::{fs::File, io::Read};

    fn read_schema(schema_name: &str) -> String {
        let schema_dir = std::path::Path::new("schemas");
        let mut file = File::open(schema_dir.join(schema_name)).unwrap();

        let mut schema = String::new();
        file.read_to_string(&mut schema).unwrap();

        schema
    }

    fn normalize_json(json_str: &str) -> Value {
        serde_json::from_str(json_str).unwrap()
    }

    assert_eq!(
        normalize_json(&read_schema("shipment-footprint.json")),
        normalize_json(&to_string_pretty(&schema_for!(ShipmentFootprint)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-shipment-footprint.json")),
        normalize_json(
            &to_string_pretty(&gen_pcf_with_extension::<ShipmentFootprint>(
                "ShipmentFootprint"
            ))
            .unwrap()
        )
    );

    assert_eq!(
        normalize_json(&read_schema("toc.json")),
        normalize_json(&to_string_pretty(&schema_for!(Toc)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-toc.json")),
        normalize_json(&to_string_pretty(&gen_pcf_with_extension::<Toc>("Toc")).unwrap())
    );

    assert_eq!(
        normalize_json(&read_schema("hoc.json")),
        normalize_json(&to_string_pretty(&schema_for!(Hoc)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-hoc.json")),
        normalize_json(&to_string_pretty(&gen_pcf_with_extension::<Hoc>("Hoc")).unwrap())
    );

    assert_eq!(
        normalize_json(&read_schema("tad.json")),
        normalize_json(&to_string_pretty(&schema_for!(Tad)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-tad.json")),
        normalize_json(&to_string_pretty(&gen_pcf_with_extension::<Tad>("Tad")).unwrap())
    );
}
