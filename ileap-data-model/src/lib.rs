/*
 * Copyright (c) 2024 Martin Pompéry
 * Copyright (c) 2024 SINE Foundation e.V.
 *
 * This software is released under the MIT License, see LICENSE.
 */

//! A Rust implementation of the iLEAP Data Model, a logistics-specific extension to the PACT Data
//! Model.
//!
//! See https://sine-fdn.github.io/ileap-extension for further details.

use chrono::{DateTime, Utc};

use pact_data_model::{PositiveDecimal, WrappedDecimal, ISO3166CC};
use rust_decimal::Decimal;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod pact_integration;
pub use pact_integration::*;

mod arbitrary_impls;

mod data_gen;
pub use data_gen::*;

pub mod schema_gen;
pub use schema_gen::*;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentFootprint {
    pub mass: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    pub shipment_id: String,
    pub tces: NonEmptyVec<Tce>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NonEmptyVec<T>(pub Vec<T>);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", rename = "TCE")]
pub struct Tce {
    pub tce_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_tce_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoc_id: Option<String>,
    pub shipment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignment_id: Option<String>,
    pub mass: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<PositiveDecimal>,
    pub distance: GlecDistance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Location>,
    pub transport_activity: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voyage_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoterms: Option<Incoterms>,
    #[serde(rename = "co2eWTW")]
    pub co2e_wtw: WrappedDecimal,
    #[serde(rename = "co2eTTW")]
    pub co2e_ttw: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none", rename = "noxTTW")]
    pub nox_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "soxTTW")]
    pub sox_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ch4TTW")]
    pub ch4_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pmTTW")]
    pub pm_ttw: Option<WrappedDecimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Incoterms {
    Exw,
    Fca,
    Cpt,
    Cip,
    Dap,
    Dpu,
    Ddp,
    Fas,
    Fob,
    Cfr,
    Cif,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
pub enum Certification {
    #[serde(rename = "ISO14083:2023")]
    ISO14083_2023,
    #[serde(rename = "GLECv2")]
    GlecV2,
    #[serde(rename = "GLECv3")]
    GlecV3,
    #[serde(rename = "GLECv3.1")]
    GlecV3_1,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", rename = "TOC")]
pub struct Toc {
    pub toc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certifications: Option<NonEmptyVec<Certification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub mode: TransportMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_factor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_distance_factor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_control: Option<TemperatureControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truck_loading_sequence: Option<TruckLoadingSequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_shipping_option: Option<AirShippingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_length: Option<FlightLength>,
    pub energy_carriers: NonEmptyVec<EnergyCarrier>,
    #[serde(rename = "co2eIntensityWTW")]
    pub co2e_intensity_wtw: WrappedDecimal,
    #[serde(rename = "co2eIntensityTTW")]
    pub co2e_intensity_ttw: WrappedDecimal,
    pub transport_activity_unit: TransportActivityUnit,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TransportActivityUnit {
    #[serde(rename = "TEUkm")]
    TEUkm,
    Tkm,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TemperatureControl {
    Ambient,
    Refrigerated,
    Mixed,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TadTempControl {
    Ambient,
    Refrigerated,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TruckLoadingSequence {
    Ltl,
    Ftl,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AirShippingOption {
    #[serde(rename = "belly freight")]
    BellyFreight,
    Freighter,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FlightLength {
    #[serde(rename = "short-haul")]
    ShortHaul,
    #[serde(rename = "long-haul")]
    LongHaul,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", rename = "HOC")]
pub struct Hoc {
    pub hoc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certifications: Option<NonEmptyVec<Certification>>,
    pub hub_type: HubType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_control: Option<TemperatureControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transport_mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transport_mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    pub energy_carriers: NonEmptyVec<EnergyCarrier>,
    #[serde(rename = "co2eIntensityWTW")]
    pub co2e_intensity_wtw: WrappedDecimal,
    #[serde(rename = "co2eIntensityTTW")]
    pub co2e_intensity_ttw: WrappedDecimal,
    pub hub_activity_unit: HubActivityUnit,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HubActivityUnit {
    #[serde(rename = "TEU")]
    TEU,
    Tonnes,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum HubType {
    Transshipment,
    StorageAndTransshipment,
    Warehouse,
    LiquidBulkTerminal,
    MaritimeContainerTerminal,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", rename = "TAD")]
/// Data Type "Transport Activity Data" of the iLEAP Technical Specifications
pub struct Tad {
    pub activity_id: ActivityId,              // Unique
    pub consignment_ids: Vec<ConsignementId>, // Unique
    pub distance: GlecDistance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_factor: Option<WrappedDecimal>, // TODO replace with propoer type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_distance_factor: Option<WrappedDecimal>, // TODO replace with propoer type
    pub origin: Location,
    pub destination: Location,
    pub departure_at: DateTime<Utc>,
    pub arrival_at: DateTime<Utc>,
    pub mode: TransportMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_carriers: Option<NonEmptyVec<EnergyCarrier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_control: Option<TadTempControl>,
}

pub type ActivityId = String;
pub type ConsignementId = String;
pub type ShipmentId = String;

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum GlecDistance {
    Actual {
        actual: WrappedDecimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        gcd: Option<WrappedDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sfd: Option<WrappedDecimal>,
    },
    Gcd {
        #[serde(skip_serializing_if = "Option::is_none")]
        actual: Option<WrappedDecimal>,
        gcd: WrappedDecimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        sfd: Option<WrappedDecimal>,
    },
    Sfd {
        #[serde(skip_serializing_if = "Option::is_none")]
        actual: Option<WrappedDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gcd: Option<WrappedDecimal>,
        sfd: WrappedDecimal,
    },
}

impl GlecDistance {
    /// construct a new GLEC distance with only actual distance set
    pub fn new_actual(distance: WrappedDecimal) -> Self {
        GlecDistance::Actual {
            actual: distance,
            gcd: None,
            sfd: None,
        }
    }

    /// construct a new GLEC distance with only GCD distance set
    pub fn new_gcd(distance: WrappedDecimal) -> Self {
        GlecDistance::Gcd {
            actual: None,
            gcd: distance,
            sfd: None,
        }
    }

    /// construct a new GLEC distance with only SFD distance set
    pub fn new_sfd(distance: WrappedDecimal) -> Self {
        GlecDistance::Sfd {
            actual: None,
            gcd: None,
            sfd: distance,
        }
    }

    pub(crate) fn get_distance(&self) -> Decimal {
        match self {
            GlecDistance::Actual { actual, .. } => actual.0,
            GlecDistance::Gcd { gcd, .. } => gcd.0,
            GlecDistance::Sfd { sfd, .. } => sfd.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    pub city: String,
    pub country: ISO3166CC,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata: Option<IataCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locode: Option<Locode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uic: Option<UicCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<WrappedDecimal>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum TransportMode {
    Road,
    Rail,
    Air,
    Sea,
    InlandWaterway,
    //Hub,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum PackagingOrTrEqType {
    Box,
    Pallet,
    #[serde(rename = "Container-TEU")]
    ContainerTEU,
    #[serde(rename = "Container-FEU")]
    ContainerFEU,
    Container,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnergyCarrier {
    pub energy_carrier: EnergyCarrierType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstocks: Option<Vec<Feedstock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption_unit: Option<EnergyConsumptionUnit>,
    #[serde(rename = "emissionFactorWTW")]
    pub emission_factor_wtw: WrappedDecimal,
    #[serde(rename = "emissionFactorTTW")]
    pub emission_factor_ttw: WrappedDecimal,
    pub relative_share: WrappedDecimal,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum EnergyCarrierType {
    Diesel,
    #[serde(rename = "HVO")]
    Hvo,
    Petrol,
    #[serde(rename = "CNG")]
    Cng,
    #[serde(rename = "LNG")]
    Lng,
    #[serde(rename = "LPG")]
    Lpg,
    #[serde(rename = "HFO")]
    Hfo,
    #[serde(rename = "MGO")]
    Mgo,
    #[serde(rename = "Aviation fuel")]
    AviationFuel,
    Hydrogen,
    Methanol,
    Electric,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EnergyConsumptionUnit {
    L,
    Kg,
    KWh,
    #[serde(rename = "MJ")]
    MJ,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feedstock {
    pub feedstock: FeedstockType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstock_share: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_provenance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum FeedstockType {
    Fossil,
    #[serde(rename = "Natural gas")]
    NaturalGas,
    Grid,
    #[serde(rename = "Renewable electricity")]
    RenewableElectricity,
    #[serde(rename = "Cooking oil")]
    CookingOil,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IataCode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for IataCode {
    fn from(s: String) -> Self {
        if s.len() <= 3 {
            IataCode(s)
        } else {
            panic!("IATA code must be 3 characters long")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Locode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for Locode {
    fn from(s: String) -> Self {
        if s.len() == 5 {
            Locode(s)
        } else {
            panic!("LOCODE must be 5 characters long, got '{s}'")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UicCode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for UicCode {
    fn from(s: String) -> Self {
        if s.len() == 2 {
            UicCode(s)
        } else {
            panic!("UIC code must be 2 characters long")
        }
    }
}

impl<T> From<Vec<T>> for NonEmptyVec<T> {
    fn from(v: Vec<T>) -> NonEmptyVec<T> {
        if v.is_empty() {
            panic!("Vector must not be empty")
        } else {
            NonEmptyVec(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_transportactivityunit_deser() {
        let tests = [
            ("\"TEUkm\"", TransportActivityUnit::TEUkm),
            ("\"tkm\"", TransportActivityUnit::Tkm),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: TransportActivityUnit = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_temperaturecontrol_deser() {
        let tests = [
            ("\"ambient\"", TemperatureControl::Ambient),
            ("\"refrigerated\"", TemperatureControl::Refrigerated),
            ("\"mixed\"", TemperatureControl::Mixed),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: TemperatureControl = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_tad_tempcontrol_deser() {
        let tests = [
            ("\"ambient\"", TadTempControl::Ambient),
            ("\"refrigerated\"", TadTempControl::Refrigerated),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: TadTempControl = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_tls_deser() {
        let tests = [
            ("\"LTL\"", TruckLoadingSequence::Ltl),
            ("\"FTL\"", TruckLoadingSequence::Ftl),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: TruckLoadingSequence = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_airshippingoption_deser() {
        let tests = [
            ("\"belly freight\"", AirShippingOption::BellyFreight),
            ("\"freighter\"", AirShippingOption::Freighter),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: AirShippingOption = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_flightlength_deser() {
        let tests = [
            ("\"short-haul\"", FlightLength::ShortHaul),
            ("\"long-haul\"", FlightLength::LongHaul),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: FlightLength = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_hubtype_deser() {
        let tests = [
            ("\"Transshipment\"", HubType::Transshipment),
            (
                "\"StorageAndTransshipment\"",
                HubType::StorageAndTransshipment,
            ),
            ("\"Warehouse\"", HubType::Warehouse),
            ("\"LiquidBulkTerminal\"", HubType::LiquidBulkTerminal),
            (
                "\"MaritimeContainerTerminal\"",
                HubType::MaritimeContainerTerminal,
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: HubType = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_certifications_deser() {
        let tests = [
            ("\"ISO14083:2023\"", Certification::ISO14083_2023),
            ("\"GLECv2\"", Certification::GlecV2),
            ("\"GLECv3\"", Certification::GlecV3),
            ("\"GLECv3.1\"", Certification::GlecV3_1),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: Certification = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_transportmode_deser() {
        let tests = [
            ("\"Road\"", TransportMode::Road),
            ("\"Rail\"", TransportMode::Rail),
            ("\"Air\"", TransportMode::Air),
            ("\"Sea\"", TransportMode::Sea),
            ("\"InlandWaterway\"", TransportMode::InlandWaterway),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: TransportMode = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_hubactivityunit_deser() {
        let tests = [
            ("\"TEU\"", HubActivityUnit::TEU),
            ("\"tonnes\"", HubActivityUnit::Tonnes),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: HubActivityUnit = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_energyconsumption_unit_deser() {
        let tests = [
            ("\"l\"", EnergyConsumptionUnit::L),
            ("\"kg\"", EnergyConsumptionUnit::Kg),
            ("\"kWh\"", EnergyConsumptionUnit::KWh),
            ("\"MJ\"", EnergyConsumptionUnit::MJ),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: EnergyConsumptionUnit = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_energycarriertype_deser() {
        let tests = [
            ("\"Diesel\"", EnergyCarrierType::Diesel),
            ("\"HVO\"", EnergyCarrierType::Hvo),
            ("\"Petrol\"", EnergyCarrierType::Petrol),
            ("\"CNG\"", EnergyCarrierType::Cng),
            ("\"LNG\"", EnergyCarrierType::Lng),
            ("\"LPG\"", EnergyCarrierType::Lpg),
            ("\"HFO\"", EnergyCarrierType::Hfo),
            ("\"MGO\"", EnergyCarrierType::Mgo),
            ("\"Aviation fuel\"", EnergyCarrierType::AviationFuel),
            ("\"Hydrogen\"", EnergyCarrierType::Hydrogen),
            ("\"Methanol\"", EnergyCarrierType::Methanol),
            ("\"Electric\"", EnergyCarrierType::Electric),
        ];
        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: EnergyCarrierType = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_packging_or_tr_eq_type_deser() {
        let tests = [
            ("\"Box\"", PackagingOrTrEqType::Box),
            ("\"Pallet\"", PackagingOrTrEqType::Pallet),
            ("\"Container-TEU\"", PackagingOrTrEqType::ContainerTEU),
            ("\"Container-FEU\"", PackagingOrTrEqType::ContainerFEU),
            ("\"Container\"", PackagingOrTrEqType::Container),
        ];

        for (input, expected) in tests {
            assert_eq!(input, serde_json::to_string(&expected).unwrap());

            let deserialized: PackagingOrTrEqType = serde_json::from_str(input).unwrap();
            assert_eq!(deserialized, expected);
        }
    }

    #[test]
    fn test_glecdistance_construction() {
        assert_eq!(
            GlecDistance::new_actual(WrappedDecimal(Decimal::new(100, 0))),
            GlecDistance::Actual {
                actual: WrappedDecimal(Decimal::new(100, 0)),
                gcd: None,
                sfd: None
            }
        );
        assert_eq!(
            GlecDistance::new_actual(WrappedDecimal(Decimal::new(100, 0))).get_distance(),
            Decimal::new(100, 0)
        );
        assert_eq!(
            GlecDistance::new_gcd(WrappedDecimal(Decimal::new(200, 0))),
            GlecDistance::Gcd {
                actual: None,
                gcd: WrappedDecimal(Decimal::new(200, 0)),
                sfd: None
            }
        );
        assert_eq!(
            GlecDistance::new_gcd(WrappedDecimal(Decimal::new(200, 0))).get_distance(),
            Decimal::new(200, 0)
        );
        assert_eq!(
            GlecDistance::new_sfd(WrappedDecimal(Decimal::new(300, 0))),
            GlecDistance::Sfd {
                actual: None,
                gcd: None,
                sfd: WrappedDecimal(Decimal::new(300, 0))
            }
        );
        assert_eq!(
            GlecDistance::new_sfd(WrappedDecimal(Decimal::new(300, 0))).get_distance(),
            Decimal::new(300, 0)
        );
    }
}
