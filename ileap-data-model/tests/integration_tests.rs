use std::{fs::File, io::Read};

use ileap_data_model::{gen_pcf_with_extension, Hoc, ShipmentFootprint, Tad, Toc};
use schemars::schema_for;
use serde_json::{to_string_pretty, Value};

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
#[test]
fn compare_schemas() {
    assert_eq!(
        normalize_json(&read_schema("shipment-footprint.json")),
        normalize_json(&to_string_pretty(&schema_for!(ShipmentFootprint)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-shipment-footprint.json")),
        normalize_json(
            &to_string_pretty(&gen_pcf_with_extension::<ShipmentFootprint>().unwrap()).unwrap()
        )
    );

    assert_eq!(
        normalize_json(&read_schema("toc.json")),
        normalize_json(&to_string_pretty(&schema_for!(Toc)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-toc.json")),
        normalize_json(&to_string_pretty(&gen_pcf_with_extension::<Toc>().unwrap()).unwrap())
    );

    assert_eq!(
        normalize_json(&read_schema("hoc.json")),
        normalize_json(&to_string_pretty(&schema_for!(Hoc)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-hoc.json")),
        normalize_json(&to_string_pretty(&gen_pcf_with_extension::<Hoc>().unwrap()).unwrap())
    );

    assert_eq!(
        normalize_json(&read_schema("tad.json")),
        normalize_json(&to_string_pretty(&schema_for!(Tad)).unwrap())
    );
    assert_eq!(
        normalize_json(&read_schema("pcf-tad.json")),
        normalize_json(&to_string_pretty(&gen_pcf_with_extension::<Tad>().unwrap()).unwrap())
    );
}
