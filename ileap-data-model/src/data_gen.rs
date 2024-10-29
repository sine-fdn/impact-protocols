use pact_data_model::{CharacterizationFactors, ProductFootprint, WrappedDecimal};
use quickcheck::*;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::pact_integration::to_pcf;
use crate::{
    GlecDistance, Hoc, HocCo2eIntensityThroughput, NonEmptyVec, PactMappedFields,
    ShipmentFootprint, Tce, Toc,
};

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ILeapType {
    ShipmentFootprint(ShipmentFootprint),
    Toc(Toc),
    Hoc(Hoc),
}

impl From<&ILeapType> for PactMappedFields {
    fn from(ileap_type: &ILeapType) -> Self {
        match ileap_type {
            ILeapType::ShipmentFootprint(shipment_footprint) => shipment_footprint.into(),
            ILeapType::Toc(toc) => toc.into(),
            ILeapType::Hoc(hoc) => hoc.into(),
        }
    }
}

// TODO: invert logic to generate a list of HOCs and TOCs and only then generate TCEs, improving
// readability and demo data quality, as suggested by Martin.
pub fn gen_rnd_demo_data(size: u8) -> Vec<ProductFootprint<ILeapType>> {
    let mut og = Gen::new(size as usize);

    let mut shipment_footprints = vec![];
    let mut tocs = vec![];
    let mut hocs = vec![];

    let num_of_shipments = u8::arbitrary(&mut og) % size + 1;
    for _ in 0..num_of_shipments {
        let mut ship_foot = ShipmentFootprint::arbitrary(&mut og);

        let mut tces: Vec<Tce> = vec![];
        let mut prev_tces: Vec<String> = vec![];

        let mut i = 0;
        let limit = u8::arbitrary(&mut og) % size + 1;
        // TODO: improve code through pair programming with Martin.
        loop {
            let mut tce = Tce::arbitrary(&mut og);

            if let Some(prev_tce) = tces.last() {
                // Updates prevTceIds for the current TCE
                prev_tces.push(prev_tce.tce_id.clone());
                tce.prev_tce_ids = Some(prev_tces.clone());

                // Avoids having two HOCs follow one another
                if prev_tce.hoc_id.is_some() && tce.hoc_id.is_some() {
                    tce = Tce::arbitrary(&mut og);
                }
            };

            if i == 0 || i == limit - 1 && tce.hoc_id.is_some() {
                tce = Tce::arbitrary(&mut og);
            }

            if tce.hoc_id.is_some() {
                // Avoids having an HOC as the first or the last TCE

                let mut hoc = Hoc::arbitrary(&mut og);
                hoc.co2e_intensity_throughput = HocCo2eIntensityThroughput::Tonnes;

                hoc.hoc_id = tce.hoc_id.clone().unwrap();

                tce.hoc_id = Some(hoc.hoc_id.clone());

                tce.distance = GlecDistance::Actual(Decimal::from(0).into());
                tce.transport_activity = Decimal::from(0).into();

                tce.co2e_wtw =
                    WrappedDecimal::from((hoc.co2e_intensity_wtw.0 * tce.mass.0).round_dp(2));
                tce.co2e_ttw =
                    WrappedDecimal::from((hoc.co2e_intensity_ttw.0 * tce.mass.0).round_dp(2));

                let hoc = to_pcf(
                    ILeapType::Hoc(hoc),
                    "SINE Foundation",
                    "urn:sine:example",
                    Some(vec![CharacterizationFactors::Ar6]),
                );

                hocs.push(hoc);
            }

            if tce.toc_id.is_some() {
                let mut toc = Toc::arbitrary(&mut og);
                toc.toc_id = tce.toc_id.clone().unwrap();

                tce.transport_activity = (tce.mass.0 * tce.distance.get_distance())
                    .round_dp(2)
                    .into();

                tce.toc_id = Some(toc.toc_id.clone());

                tce.co2e_wtw = WrappedDecimal::from(
                    (toc.co2e_intensity_wtw.0 * tce.transport_activity.0).round_dp(2),
                );
                tce.co2e_ttw = WrappedDecimal::from(
                    (toc.co2e_intensity_ttw.0 * tce.transport_activity.0).round_dp(2),
                );

                let toc = to_pcf(
                    ILeapType::Toc(toc),
                    "SINE Foundation",
                    "urn:sine:example",
                    Some(vec![CharacterizationFactors::Ar6]),
                );

                tocs.push(toc.clone());
            }

            tce.shipment_id.clone_from(&ship_foot.shipment_id);

            tces.push(tce);

            i += 1;
            if i == limit {
                break;
            }
        }

        ship_foot.tces = NonEmptyVec::from(tces);

        let ship_foot = to_pcf(
            ILeapType::ShipmentFootprint(ship_foot),
            "SINE Foundation",
            "urn:sine:example",
            Some(vec![CharacterizationFactors::Ar6]),
        );

        shipment_footprints.push(ship_foot);
    }

    vec![shipment_footprints, tocs, hocs]
        .into_iter()
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_rnd_demo_data() {
        let footprints = gen_rnd_demo_data(10);

        for footprint in footprints.iter() {
            if let Some(extensions) = &footprint.extensions {
                for extension in extensions.iter() {
                    if let ILeapType::ShipmentFootprint(ship_foot) = &extension.data {
                        for tce in ship_foot.tces.0.iter() {
                            assert!(
                                tce.toc_id.is_some() ^ tce.hoc_id.is_some(),
                                "Either tocId or hocId, but not both, must be provided."
                            );
                        }
                    }
                }
            }
        }

        println!("{footprints:#?}");
    }
}
