/*
 * Copyright (c) 2024 Martin Pompéry
 * Copyright (c) 2024 SINE Foundation e.V.
 *
 * This software is released under the MIT License, see LICENSE.
 */

use ileap_data_model::{AggregatedReport, Hoc, ShipmentFootprint, Tad, Toc};
use std::io::Error;

use ileap_data_model::schema_gen::{write_pcf_schema, write_standalone_schema};

fn main() -> Result<(), Error> {
    write_standalone_schema::<ShipmentFootprint>("shipment-footprint")?;
    write_pcf_schema::<ShipmentFootprint>("ShipmentFootprint", "pcf-shipment-footprint")?;

    write_standalone_schema::<Toc>("toc")?;
    write_pcf_schema::<Toc>("Toc", "pcf-toc")?;

    write_standalone_schema::<Hoc>("hoc")?;
    write_pcf_schema::<Hoc>("Hoc", "pcf-hoc")?;

    write_standalone_schema::<Tad>("tad")?;

    write_standalone_schema::<AggregatedReport>("ar")?;

    Ok(())
}
