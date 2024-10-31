/*
 * Copyright (c) 2022-2024 Martin Pompéry
 * Copyright (c) 2023-2024 SINE Foundation e.V.
 *
 * This software is released under the MIT License, see LICENSE.
 */

use pact_data_model::*;
use schemars::schema::RootSchema;
use schemars::schema_for;
use serde_json::{to_string_pretty, Value};
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let mut schema = schema_for!(ProductFootprint<Value>);

    update_schema_title(&mut schema);

    let schema_json = to_string_pretty(&schema).expect("Failed to serialize schema");

    let mut file = File::create("./pact-data-model/schema/data-model-schema.json")?;

    file.write_all(schema_json.as_bytes())?;

    println!("data-model-schema.json successfully created");

    Ok(())
}

pub fn update_schema_title(schema: &mut RootSchema) {
    if let Some(metadata) = schema.schema.metadata.as_mut() {
        metadata.title = Some("ProductFootprint".to_string());
    }
}
