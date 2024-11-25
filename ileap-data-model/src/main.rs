/*
 * Copyright (c) 2024 Martin Pompéry
 * Copyright (c) 2024 SINE Foundation e.V.
 *
 * This software is released under the MIT License, see LICENSE.
 */

use ileap_data_model::{Hoc, ShipmentFootprint, Tad, Toc};
use std::io::Error;

use ileap_data_model::schema_gen::write_schemas;

fn main() -> Result<(), Error> {
    write_schemas::<ShipmentFootprint>(
        "ShipmentFootprint",
        "shipment-footprint",
        "pcf-shipment-footprint",
    )?;
    write_schemas::<Toc>("Toc", "toc", "pcf-toc")?;
    write_schemas::<Tad>("Tad", "tad", "pcf-tad")?;
    write_schemas::<Hoc>("Hoc", "hoc", "pcf-hoc")?;

    Ok(())
}
