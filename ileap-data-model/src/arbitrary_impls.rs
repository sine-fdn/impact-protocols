use core::str;

use crate::*;
use chrono::Duration;
use quickcheck::Arbitrary;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

#[derive(Clone)]
pub struct LowerAToZNumDash(String);

impl LowerAToZNumDash {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Arbitrary for LowerAToZNumDash {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let maybe_string: Vec<u8> = Vec::arbitrary(g)
            .into_iter()
            .map(|v: u8| {
                let i = v % 37;

                match i {
                    0 => b'-',
                    1..=10 => i + 47,
                    _ => i + 86,
                }
            })
            .collect();

        Self(str::from_utf8(&maybe_string).unwrap().to_string())
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let s = self.0.clone();
        let range = 0..self.len();
        let shrunk: Vec<_> = range
            .into_iter()
            .map(|len| Self(s[0..len].to_string()))
            .collect();
        Box::new(shrunk.into_iter())
    }
}

fn formatted_arbitrary_string(fixed: &str, g: &mut quickcheck::Gen) -> String {
    fixed.to_string() + &LowerAToZNumDash::arbitrary(g).0
}

impl Arbitrary for ShipmentFootprint {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        ShipmentFootprint {
            // Using u16 to avoid unreadably large numbers.
            mass: format!("{}", u16::arbitrary(g)),
            shipment_id: formatted_arbitrary_string("shipment-", g),
            tces: NonEmptyVec::<Tce>::arbitrary(g),
            // Currently None for simplicity.
            volume: None,
            number_of_items: None,
            type_of_items: None,
        }
    }
}

impl Arbitrary for Hoc {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        fn gen_diff_transport_modes(
            g: &mut quickcheck::Gen,
        ) -> (Option<TransportMode>, Option<TransportMode>) {
            let inbound = Some(TransportMode::arbitrary(g));
            let mut outbound = Some(TransportMode::arbitrary(g));

            while inbound == outbound {
                outbound = Some(TransportMode::arbitrary(g));
            }

            (inbound, outbound)
        }

        let hub_type = HubType::arbitrary(g);

        let (inbound_transport_mode, outbound_transport_mode) = match hub_type {
            // TODO: verify whether Transshipment and StorageAndTransshipment require different
            // inbound and outbound transport modes.
            HubType::Transshipment => gen_diff_transport_modes(g),
            HubType::StorageAndTransshipment => gen_diff_transport_modes(g),
            HubType::Warehouse => (Some(TransportMode::Road), Some(TransportMode::Road)),
            HubType::LiquidBulkTerminal => (
                Some(TransportMode::arbitrary(g)),
                Some(TransportMode::arbitrary(g)),
            ),
            HubType::MaritimeContainerterminal => {
                let inbound = Option::<TransportMode>::arbitrary(g);
                let outbound = Option::<TransportMode>::arbitrary(g);

                let (inbound, outbound) = match (inbound.clone(), outbound.clone()) {
                    (None, None) => (inbound, outbound),
                    (Some(TransportMode::Sea), _) => (inbound, outbound),
                    (_, Some(TransportMode::Sea)) => (inbound, outbound),
                    _ => (
                        Option::<TransportMode>::arbitrary(g),
                        Option::<TransportMode>::arbitrary(g),
                    ),
                };

                (inbound, outbound)
            }
        };

        Hoc {
            hoc_id: formatted_arbitrary_string("hoc-", g),
            is_verified: bool::arbitrary(g),
            is_accredited: bool::arbitrary(g),
            hub_type,
            temperature_control: Option::<TemperatureControl>::arbitrary(g),
            inbound_transport_mode,
            outbound_transport_mode,
            packaging_or_tr_eq_type: Option::<PackagingOrTrEqType>::arbitrary(g),
            energy_carriers: NonEmptyVec::<EnergyCarrier>::arbitrary(g),
            co2e_intensity_wtw: arbitrary_wrapped_decimal(g),
            co2e_intensity_ttw: arbitrary_wrapped_decimal(g),
            co2e_intensity_throughput: HocCo2eIntensityThroughput::arbitrary(g),
            // Currently None for simplicity.
            description: None,
            hub_location: None,
            packaging_or_tr_eq_amount: None,
        }
    }
}

impl Arbitrary for HocCo2eIntensityThroughput {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let hoc_co2e_intensity_throughput = &[
            HocCo2eIntensityThroughput::TEU,
            HocCo2eIntensityThroughput::Tonnes,
        ];

        g.choose(hoc_co2e_intensity_throughput).unwrap().to_owned()
    }
}

impl Arbitrary for HubType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let hub_type = &[
            HubType::Transshipment,
            HubType::StorageAndTransshipment,
            HubType::Warehouse,
            HubType::LiquidBulkTerminal,
            HubType::MaritimeContainerterminal,
        ];

        g.choose(hub_type).unwrap().to_owned()
    }
}

impl Arbitrary for PackagingOrTrEqType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let packaging_or_tr_eq_type = &[
            PackagingOrTrEqType::Box,
            PackagingOrTrEqType::Pallet,
            PackagingOrTrEqType::Container,
        ];

        g.choose(packaging_or_tr_eq_type).unwrap().to_owned()
    }
}

fn arbitrary_option_factor(g: &mut quickcheck::Gen) -> Option<String> {
    let rand_num = u8::arbitrary(g) % 10 + 1;
    let rand_factor: Decimal = Decimal::new(rand_num as i64, 1);

    Some(rand_factor.to_string())
}

impl Arbitrary for Toc {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mode = TransportMode::arbitrary(g);

        let (air_shipping_option, flight_length) = match mode {
            TransportMode::Air => (
                Option::<AirShippingOption>::arbitrary(g),
                Option::<FlightLength>::arbitrary(g),
            ),
            _ => (None, None),
        };

        Toc {
            toc_id: formatted_arbitrary_string("toc-", g),
            is_verified: bool::arbitrary(g),
            is_accredited: bool::arbitrary(g),
            mode,
            load_factor: arbitrary_option_factor(g),
            empty_distance_factor: arbitrary_option_factor(g),
            temperature_control: Option::<TemperatureControl>::arbitrary(g),
            truck_loading_sequence: Option::<TruckLoadingSequence>::arbitrary(g),
            air_shipping_option,
            flight_length,
            energy_carriers: NonEmptyVec::<EnergyCarrier>::arbitrary(g),
            co2e_intensity_wtw: arbitrary_wrapped_decimal(g),
            co2e_intensity_ttw: arbitrary_wrapped_decimal(g),
            co2e_intensity_throughput: TocCo2eIntensityThroughput::arbitrary(g),
            // Currently None for simplicity.
            description: None,
            glec_data_quality_index: None,
        }
    }
}

impl Arbitrary for TocCo2eIntensityThroughput {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let toc_co2e_intensity_throughput = &[
            TocCo2eIntensityThroughput::Tkm,
            TocCo2eIntensityThroughput::TEUkm,
        ];

        g.choose(toc_co2e_intensity_throughput).unwrap().to_owned()
    }
}

impl<T: Arbitrary> Arbitrary for NonEmptyVec<T> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // Restricting to 1..5 elements.
        let num = u8::arbitrary(g) % 5 + 1;

        let mut vec = vec![];
        for _ in 0..num {
            vec.push(T::arbitrary(g));
        }
        NonEmptyVec(vec)
    }
}

impl Arbitrary for TransportMode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let transport_mode = &[
            TransportMode::Road,
            TransportMode::Rail,
            TransportMode::Air,
            TransportMode::Sea,
            TransportMode::InlandWaterway,
        ];

        g.choose(transport_mode).unwrap().to_owned()
    }
}

impl Arbitrary for TemperatureControl {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let temperature_control = &[
            TemperatureControl::Ambient,
            TemperatureControl::Refrigerated,
            TemperatureControl::Mixed,
        ];

        g.choose(temperature_control).unwrap().to_owned()
    }
}

impl Arbitrary for TruckLoadingSequence {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let truck_loading_sequence = &[TruckLoadingSequence::Ftl, TruckLoadingSequence::Ltl];

        g.choose(truck_loading_sequence).unwrap().to_owned()
    }
}

impl Arbitrary for AirShippingOption {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let air_shipping_option = &[
            AirShippingOption::BellyFreight,
            AirShippingOption::Freighter,
        ];

        g.choose(air_shipping_option).unwrap().to_owned()
    }
}

impl Arbitrary for FlightLength {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let flight_length = &[FlightLength::ShortHaul, FlightLength::LongHaul];

        g.choose(flight_length).unwrap().to_owned()
    }
}

impl Arbitrary for EnergyCarrier {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let energy_carrier = EnergyCarrierType::arbitrary(g);

        let feedstocks = match Option::<Vec<Feedstock>>::arbitrary(g) {
            Some(mut feedstocks) => {
                let total_percentage: Decimal = feedstocks
                    .iter()
                    .map(|f| {
                        f.feedstock_percentage
                            .as_ref()
                            .map(|p| p.0)
                            .unwrap_or(Decimal::from(0))
                    })
                    .sum();

                if total_percentage > Decimal::from(1) {
                    for feedstock in &mut feedstocks {
                        feedstock.feedstock_percentage = None
                    }
                }

                // TODO: verify which feedstocks make sense for each energy carrier.
                feedstocks = feedstocks
                    .iter()
                    .filter(|f| match energy_carrier {
                        EnergyCarrierType::Diesel => f.feedstock == FeedstockType::Fossil,
                        EnergyCarrierType::Hvo => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Petrol => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Cng => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Lng => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Lpg => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Hfo => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Mgo => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::AviationFuel => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Hydrogen => f.feedstock == FeedstockType::CookingOil,
                        EnergyCarrierType::Methanol => {
                            f.feedstock == FeedstockType::Fossil
                                || f.feedstock == FeedstockType::NaturalGas
                                || f.feedstock == FeedstockType::CookingOil
                        }
                        EnergyCarrierType::Electric => {
                            f.feedstock == FeedstockType::Grid
                                || f.feedstock == FeedstockType::RenewableElectricity
                        }
                    })
                    .cloned()
                    .collect::<Vec<Feedstock>>();

                Some(feedstocks)
            }
            None => None,
        };

        EnergyCarrier {
            energy_carrier,
            feedstocks,
            energy_consumption: arbitrary_option_wrapped_decimal(g),
            energy_consumption_unit: Option::<EnergyConsumptionUnit>::arbitrary(g),
            emission_factor_wtw: arbitrary_wrapped_decimal(g),
            emission_factor_ttw: arbitrary_wrapped_decimal(g),
        }
    }
}

impl Arbitrary for EnergyCarrierType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let energy_carrier = &[
            EnergyCarrierType::Diesel,
            EnergyCarrierType::Hvo,
            EnergyCarrierType::Petrol,
            EnergyCarrierType::Cng,
            EnergyCarrierType::Lng,
            EnergyCarrierType::Lpg,
            EnergyCarrierType::Hfo,
            EnergyCarrierType::Mgo,
            EnergyCarrierType::AviationFuel,
            EnergyCarrierType::Hydrogen,
            EnergyCarrierType::Methanol,
            EnergyCarrierType::Electric,
        ];

        g.choose(energy_carrier).unwrap().to_owned()
    }
}

impl Arbitrary for EnergyConsumptionUnit {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let energy_consumption_unit = &[
            EnergyConsumptionUnit::KWh,
            EnergyConsumptionUnit::MJ,
            EnergyConsumptionUnit::Kg,
            EnergyConsumptionUnit::L,
        ];

        g.choose(energy_consumption_unit).unwrap().to_owned()
    }
}

impl Arbitrary for Feedstock {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let feedstock_percentage = arbitrary_option_wrapped_decimal(g);

        let feedstock_percentage = match feedstock_percentage {
            None => None,
            Some(f) => {
                let decimal = (f.0 / Decimal::from(u16::MAX)).round_dp(1);
                Some(WrappedDecimal::from(decimal))
            }
        };

        Feedstock {
            feedstock: FeedstockType::arbitrary(g),
            feedstock_percentage,
            // Currently None for simplicity.
            region_provenance: None,
        }
    }
}

impl Arbitrary for FeedstockType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let feedstock = &[
            FeedstockType::Fossil,
            FeedstockType::NaturalGas,
            FeedstockType::Grid,
            FeedstockType::RenewableElectricity,
            FeedstockType::CookingOil,
        ];

        g.choose(feedstock).unwrap().to_owned()
    }
}

impl Arbitrary for GlecDataQualityIndex {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        GlecDataQualityIndex(u8::arbitrary(g) % 5)
    }
}

impl Arbitrary for Tce {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let toc_id = if bool::arbitrary(g) {
            Some(formatted_arbitrary_string("toc-", g))
        } else {
            None
        };

        let hoc_id = match toc_id {
            Some(_) => None,
            None => Some(formatted_arbitrary_string("hoc-", g)),
        };

        let mass = arbitrary_wrapped_decimal(g);
        let glec_distance = GlecDistance::arbitrary(g);

        let distance = match &glec_distance {
            GlecDistance::Actual(d) => d,
            GlecDistance::Gcd(d) => d,
            GlecDistance::Sfd(d) => d,
        };

        let transport_activity = WrappedDecimal::from(mass.0 * distance.0);

        let departure_at =
            Option::<DateTime<Utc>>::from(Utc::now() + Duration::days(u8::arbitrary(g) as i64));

        let arrival_at = match departure_at {
            None => None,
            Some(departure) => {
                // Assuming an average speed of 100 km/h, calculate the arrival time based on the
                // distance, rounded.
                let hours = (distance.0 / Decimal::from(100)).round().to_i64().unwrap();

                Some(departure + Duration::hours(hours))
            }
        };

        Tce {
            tce_id: formatted_arbitrary_string("tce-", g),
            // Empty vec by default, populated by the generator function on main.
            prev_tce_ids: Some(vec![]),
            toc_id,
            hoc_id,
            shipment_id: formatted_arbitrary_string("shipment-", g),
            consignment_id: Some(formatted_arbitrary_string("consignment-", g)),
            mass,
            packaging_or_tr_eq_type: Option::<PackagingOrTrEqType>::arbitrary(g),
            packaging_or_tr_eq_amount: Option::<usize>::arbitrary(g),
            distance: glec_distance,
            // TODO: origin and destination are currently None to avoid an inconsistencies with the
            // distance field. In order to fix this, we need to ensure that either the distance is
            // calculated from the origin and destination or that the origin and destination are set
            // based on the distance.
            origin: None,
            destination: None,
            transport_activity,
            departure_at,
            arrival_at,
            incoterms: Option::<Incoterms>::arbitrary(g),
            // co2eWTW and co2eTTW are populated by the generator function on main, based on the
            // emissions profile of the TOC/HOC.
            co2e_wtw: Decimal::from(0).into(),
            co2e_ttw: Decimal::from(0).into(),
            // Currently None for simplicity.
            flight_no: None,
            voyage_no: None,
            nox_ttw: None,
            sox_ttw: None,
            ch4_ttw: None,
            pm_ttw: None,
        }
    }
}

impl Arbitrary for GlecDistance {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let glec_distance = &[
            // Dividing u16 by 2 to avoid unreadably large data.
            GlecDistance::Actual(Decimal::from(u16::arbitrary(g) / 2).into()),
            GlecDistance::Gcd(Decimal::from(u16::arbitrary(g) / 2).into()),
            GlecDistance::Sfd(Decimal::from(u16::arbitrary(g) / 2).into()),
        ];

        g.choose(glec_distance).unwrap().to_owned()
    }
}

impl Arbitrary for Location {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Location {
            street: Option::<String>::arbitrary(g),
            zip: Option::<String>::arbitrary(g),
            city: String::arbitrary(g),
            country: GeographicScope::Country {
                geography_country: pact_data_model::ISO3166CC(String::arbitrary(g)),
            },
            iata: Option::<IataCode>::arbitrary(g),
            locode: Option::<Locode>::arbitrary(g),
            uic: Option::<UicCode>::arbitrary(g),
            lat: arbitrary_option_wrapped_decimal(g),
            lng: arbitrary_option_wrapped_decimal(g),
        }
    }
}

impl Arbitrary for IataCode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut s = String::new();

        for _ in 0..3 {
            let ascii_capital = ((u8::arbitrary(g) % 26) + 65) as char;
            s.push(ascii_capital)
        }

        IataCode::from(s)
    }
}

impl Arbitrary for Locode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut s = String::new();

        for _ in 0..5 {
            // 65..90 - ASCII A to Z
            let ascii_capital = ((u8::arbitrary(g) % 26) + 65) as char;
            s.push(ascii_capital)
        }

        Locode::from(s)
    }
}

impl Arbitrary for UicCode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut s = String::new();

        for _ in 0..2 {
            let int = (u8::arbitrary(g) % 9) + 1;
            s.push(int as char)
        }

        UicCode::from(s)
    }
}

fn arbitrary_wrapped_decimal(g: &mut quickcheck::Gen) -> WrappedDecimal {
    Decimal::from(u16::arbitrary(g)).round_dp(2).into()
}

fn arbitrary_option_wrapped_decimal(g: &mut quickcheck::Gen) -> Option<WrappedDecimal> {
    let option = &[Some(arbitrary_wrapped_decimal(g)), None];

    g.choose(option).unwrap().to_owned()
}

impl Arbitrary for Incoterms {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let incoterms = &[
            Incoterms::Exw,
            Incoterms::Fca,
            Incoterms::Cpt,
            Incoterms::Cip,
            Incoterms::Dap,
            Incoterms::Dpu,
            Incoterms::Ddp,
            Incoterms::Fas,
            Incoterms::Fob,
            Incoterms::Cfr,
            Incoterms::Cif,
        ];

        g.choose(incoterms).unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hoc, ShipmentFootprint, Tce, Toc};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn ser_and_deser_tce(tce: Tce) -> bool {
        let serialized = serde_json::to_string(&tce).unwrap();
        let deserialized = serde_json::from_str::<Tce>(&serialized).unwrap();

        println!("tce: {tce:?}");
        println!("serialized: {serialized}");
        println!("deserialized: {deserialized:?}");

        deserialized == tce
    }

    #[quickcheck]
    fn ser_and_deser_toc(toc: Toc) -> bool {
        let serialized = serde_json::to_string(&toc).unwrap();
        let deserialized = serde_json::from_str::<Toc>(&serialized).unwrap();

        if deserialized != toc {
            println!("toc: {toc:?}");
            println!("deserialized: {deserialized:?}");
        }

        deserialized == toc
        // true
    }

    #[quickcheck]
    fn ser_and_deser_hoc(hoc: Hoc) -> bool {
        let serialized = serde_json::to_string(&hoc).unwrap();
        let deserialized = serde_json::from_str::<Hoc>(&serialized).unwrap();

        if deserialized != hoc {
            println!("toc: {hoc:?}");
            println!("deserialized: {deserialized:?}");
        }

        deserialized == hoc
    }

    #[quickcheck]
    fn ser_and_deser_ship_foot(ship_foot: ShipmentFootprint) {
        let serialized = serde_json::to_string(&ship_foot).unwrap();
        let deserialized = serde_json::from_str::<ShipmentFootprint>(&serialized).unwrap();

        if deserialized != ship_foot {
            println!("ship_foot: {ship_foot:?}");
            println!("deserialized: {deserialized:?}");
        }

        assert_eq!(deserialized, ship_foot);
    }
}
