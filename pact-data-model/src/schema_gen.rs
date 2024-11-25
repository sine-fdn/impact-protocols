use std::{
    fs::File,
    io::{Error, Write},
};

use schemars::{
    gen::SchemaGenerator,
    schema::{RootSchema, Schema},
    schema_for,
};
use serde_json::{to_string_pretty, Value};

use crate::{NonEmptyString, ProductFootprint, UNRegionOrSubregion, ISO3166CC};

pub fn generate_schema() -> Result<(), Error> {
    let mut schema = schema_for!(ProductFootprint<Value>);

    update_schema_title(&mut schema);
    include_geo_scope_props(&mut schema);

    let schema_json = to_string_pretty(&schema).expect("Failed to serialize schema");

    let mut file = File::create("./pact-data-model/schema/data-model-schema.json")?;

    file.write_all(schema_json.as_bytes())?;

    println!("data-model-schema.json successfully created");

    Ok(())
}

fn update_schema_title(schema: &mut RootSchema) {
    if let Some(metadata) = schema.schema.metadata.as_mut() {
        metadata.title = Some("ProductFootprint".to_string());
    }
}

fn include_geo_scope_props(schema: &mut RootSchema) {
    let carbon_footprint = schema
        .definitions
        .get("CarbonFootprint")
        .unwrap()
        .to_owned();

    let mut carbon_footprint = carbon_footprint.into_object();
    if let Some(object) = carbon_footprint.object.as_mut() {
        let mut gen = SchemaGenerator::default();
        object.properties.insert(
            "geographyCountry".to_string(),
            gen.subschema_for::<ISO3166CC>(),
        );
        object.properties.insert(
            "geographyRegionOrSubregion".to_string(),
            gen.subschema_for::<UNRegionOrSubregion>(),
        );
        object.properties.insert(
            "geographyCountrySubdivision".to_string(),
            gen.subschema_for::<NonEmptyString>(),
        );
    }

    schema.definitions.insert(
        "CarbonFootprint".to_string(),
        Schema::Object(carbon_footprint),
    );
}

#[test]

fn compare_schemas() {
    use crate::ProductFootprint;
    use schemars::schema_for;
    use serde_json::{to_string_pretty, Value};
    use std::{fs::File, io::Read};

    fn read_schema(schema_name: &str) -> String {
        let schema_dir = std::path::Path::new("schema");
        let mut file = File::open(schema_dir.join(schema_name)).unwrap();

        let mut schema = String::new();
        file.read_to_string(&mut schema).unwrap();

        schema
    }

    fn normalize_json(json_str: &str) -> Value {
        serde_json::from_str(json_str).unwrap()
    }

    let mut gen_schema = schema_for!(ProductFootprint<Value>);
    update_schema_title(&mut gen_schema);
    include_geo_scope_props(&mut gen_schema);

    assert_eq!(
        normalize_json(&read_schema("data-model-schema.json")),
        normalize_json(&to_string_pretty(&gen_schema).unwrap())
    );
}
