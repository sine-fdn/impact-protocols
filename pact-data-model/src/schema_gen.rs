use std::{
    collections::BTreeSet,
    fs::File,
    io::{Error, Write},
};

use schemars::{
    schema::{
        InstanceType, ObjectValidation, RootSchema, Schema, SchemaObject, SubschemaValidation,
    },
    schema_for, Map,
};
use serde_json::{to_string_pretty, Value};

use crate::{NonEmptyString, ProductFootprint, UNRegionOrSubregion, ISO3166CC};

pub fn generate_schema() -> Result<(), Error> {
    let mut schema = schema_for!(ProductFootprint<Value>);

    update_schema_title(&mut schema);
    fix_geographic_scope(&mut schema);

    let schema_json = to_string_pretty(&schema).expect("Failed to serialize schema");

    let mut file = File::create("./pact-data-model/schema/data-model-schema.json")?;

    file.write_all(schema_json.as_bytes())?;

    println!("data-model-schema.json successfully created");

    Ok(())
}

fn fix_geographic_scope(schema: &mut RootSchema) {
    let definitions = schema.clone().definitions;
    let carbon_footprint_schema = definitions.get("CarbonFootprint").unwrap().to_owned();

    // Create the oneOf variants for geographic scope
    let mut one_of = Vec::new();

    // Regional variant
    one_of.push(Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        object: Some(Box::new(ObjectValidation {
            required: BTreeSet::from(["geographyRegionOrSubregion".to_string()]),
            properties: Map::from([
                (
                    "geographyRegionOrSubregion".to_string(),
                    Schema::Object(SchemaObject {
                        instance_type: None,
                        reference: Some(format!("#/definitions/UNRegionOrSubregion",)),
                        ..Default::default()
                    }),
                ),
                ("geographyCountry".to_string(), Schema::Bool(false)),
                (
                    "geographyCountrySubdivision".to_string(),
                    Schema::Bool(false),
                ),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    }));

    // Country variant
    one_of.push(Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        object: Some(Box::new(ObjectValidation {
            required: BTreeSet::from(["geographyCountry".to_string()]),
            properties: Map::from([
                (
                    "geographyCountry".to_string(),
                    Schema::Object(SchemaObject {
                        instance_type: None,
                        reference: Some(format!("#/definitions/ISO3166CC",)),
                        ..Default::default()
                    }),
                ),
                (
                    "geographyRegionOrSubregion".to_string(),
                    Schema::Bool(false),
                ),
                (
                    "geographyCountrySubdivision".to_string(),
                    Schema::Bool(false),
                ),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    }));

    // Subdivision variant
    one_of.push(Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        object: Some(Box::new(ObjectValidation {
            required: BTreeSet::from(["geographyCountrySubdivision".to_string()]),
            properties: Map::from([
                (
                    "geographyCountrySubdivision".to_string(),
                    Schema::Object(SchemaObject {
                        instance_type: None,
                        reference: Some(format!("#/definitions/NonEmptyString",)),
                        ..Default::default()
                    }),
                ),
                (
                    "geographyRegionOrSubregion".to_string(),
                    Schema::Bool(false),
                ),
                ("geographyCountry".to_string(), Schema::Bool(false)),
            ]),
            ..Default::default()
        })),
        ..Default::default()
    }));

    // Global variant (no geographic properties)
    one_of.push(Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        object: Some(Box::new(ObjectValidation {
            properties: Map::from([
                (
                    "geographyRegionOrSubregion".to_string(),
                    Schema::Bool(false),
                ),
                ("geographyCountry".to_string(), Schema::Bool(false)),
                (
                    "geographyCountrySubdivision".to_string(),
                    Schema::Bool(false),
                ),
            ]),

            ..Default::default()
        })),
        ..Default::default()
    }));

    let mut carbon_footprint_schema = carbon_footprint_schema.clone().into_object();
    carbon_footprint_schema.subschemas = Some(Box::new(SubschemaValidation {
        one_of: Some(one_of),
        ..Default::default()
    }));

    schema.definitions.insert(
        "CarbonFootprint".to_string(),
        Schema::Object(carbon_footprint_schema),
    );
}

pub fn update_schema_title(schema: &mut RootSchema) {
    if let Some(metadata) = schema.schema.metadata.as_mut() {
        metadata.title = Some("ProductFootprint".to_string());
    }
}
