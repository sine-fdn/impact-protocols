/*
 * Copyright (c) 2024 Martin Pompéry
 * Copyright (c) 2024 SINE Foundation e.V.
 *
 * This software is released under the MIT License, see LICENSE.
 */

use ileap_data_model::{Hoc, ShipmentFootprint, Tad, Toc};
use std::io::Error;

use ileap_data_model::schema_gen::generate_schemas;

fn main() -> Result<(), Error> {
    generate_schemas::<ShipmentFootprint>()?;
    generate_schemas::<Toc>()?;
    generate_schemas::<Tad>()?;
    generate_schemas::<Hoc>()?;

    Ok(())
}
