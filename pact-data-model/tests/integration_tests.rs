use pact_data_model::ProductFootprint;
use schemars::{schema::RootSchema, schema_for};
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
#[test]

fn compare_schemas() {
    fn update_schema_title(schema: &mut RootSchema) {
        if let Some(metadata) = schema.schema.metadata.as_mut() {
            metadata.title = Some("ProductFootprint".to_string());
        }
    }

    let mut gen_schema = schema_for!(ProductFootprint<Value>);
    update_schema_title(&mut gen_schema);

    assert_eq!(
        normalize_json(&read_schema("data-model-schema.json")),
        normalize_json(&to_string_pretty(&gen_schema).unwrap())
    );
}
