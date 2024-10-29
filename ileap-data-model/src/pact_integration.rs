use chrono::Utc;
use pact_data_model::{
    CarbonFootprint, CharacterizationFactors, CompanyIdSet, CrossSectoralStandard,
    CrossSectoralStandardSet, DataModelExtension, DeclaredUnit, ExemptedEmissionsPercent,
    IpccCharacterizationFactorsSource, PfId, PfStatus, PositiveDecimal, ProductFootprint,
    ProductIdSet, SpecVersionString, Urn, VersionInteger,
};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::{Hoc, HocCo2eIntensityThroughput, ShipmentFootprint, Toc};

/*pub enum HocTeuContainerSize {
    Normal,
    Light,
    Heavy,
}*/

/* fn get_teu_co2e_intensity_wtw(
    hoc_co2e_intensity_wtw: Decimal,
    hoc_container_size: &Option<HocTeuContainerSize>,
) -> Decimal {
    match hoc_container_size {
        Some(HocTeuContainerSize::Normal) => hoc_co2e_intensity_wtw * Decimal::from(10000),
        Some(HocTeuContainerSize::Light) => hoc_co2e_intensity_wtw * Decimal::from(6000),
        Some(HocTeuContainerSize::Heavy) => hoc_co2e_intensity_wtw * Decimal::from(14050),
        None => {
            println!("Warning: HOC TEU container size not specified, using normal container");
            hoc_co2e_intensity_wtw * Decimal::from(10000)
        }
    }
} */

pub struct PactMappedFields {
    product_id_type: &'static str,
    data_schema_id: &'static str,
    id: String,
    product_name_company: String,
    declared_unit: DeclaredUnit,
    unitary_product_amount: Decimal,
    p_cf_excluding_biogenic: Decimal,
}

impl From<&ShipmentFootprint> for PactMappedFields {
    fn from(shipment: &ShipmentFootprint) -> Self {
        PactMappedFields {
            product_id_type: "shipment",
            data_schema_id: "shipment-footprint",
            id: shipment.shipment_id.clone(),
            product_name_company: format!("ShipmentFootprint with id {}", shipment.shipment_id),
            declared_unit: DeclaredUnit::TonKilometer,
            unitary_product_amount: shipment
                .tces
                .0
                .iter()
                .fold(Decimal::from(0), |acc, tce| acc + tce.transport_activity.0),
            p_cf_excluding_biogenic: shipment
                .tces
                .0
                .iter()
                .fold(Decimal::from(0), |acc, tce| acc + tce.co2e_wtw.0),
        }
    }
}

impl From<&Hoc> for PactMappedFields {
    fn from(hoc: &Hoc) -> Self {
        PactMappedFields {
            product_id_type: "hoc",
            data_schema_id: "hoc",
            id: hoc.hoc_id.clone(),
            product_name_company: format!("HOC with ID {}", hoc.hoc_id),
            declared_unit: DeclaredUnit::Kilogram,
            unitary_product_amount: Decimal::from(1000),
            p_cf_excluding_biogenic: match hoc.co2e_intensity_throughput {
                HocCo2eIntensityThroughput::TEU => {
                    panic!("HOC with TEU throughput is not supported, yet")
                }
                HocCo2eIntensityThroughput::Tonnes => hoc.co2e_intensity_wtw.0,
            },
        }
    }
}

impl From<&Toc> for PactMappedFields {
    fn from(toc: &Toc) -> Self {
        PactMappedFields {
            product_id_type: "toc",
            data_schema_id: "toc",
            id: toc.toc_id.clone(),
            product_name_company: format!("TOC with ID {}", toc.toc_id),
            declared_unit: DeclaredUnit::TonKilometer,
            unitary_product_amount: Decimal::from(1),
            p_cf_excluding_biogenic: toc.co2e_intensity_wtw.0,
        }
    }
}

/**
 * Converts an iLEAP type into a PACT Data Model's ProductFootprint.
 *
 * To do so, additional propertiers are needed:
 * - company_name: the name of the company that is responsible for the product
 * - company_urn: the URN of the company that is responsible for the product
 * - characterization_factors: the optional IPCC characterization factors that were used in the calculation of the carbon footprint (TOC, HOC, ShipmentFootprint). If not defined `AR5` will be used.
 */
pub fn to_pcf<T>(
    ileap_type: T,
    company_name: &str,
    company_urn: &str,
    //    hoc_container_size: Option<HocTeuContainerSize>,
    characterization_factors: Option<Vec<CharacterizationFactors>>,
) -> ProductFootprint<T>
where
    T: JsonSchema + Serialize,
    PactMappedFields: for<'a> From<&'a T>,
{
    // Massage the optional IPCC characterization factors into a tuple of the actual factors and the
    // IPCC Characterization Factor sources
    let (characterization_factors, characterization_factors_sources) =
        to_char_factors(characterization_factors);

    // Extract the properties necessary to turn the iLEAP type into a ProductFootprint.
    // Note: this conversion at this point is "static" and does not require any additional data.
    //        However, the current implementation requires the HOC data type to declare its throughput
    //        in tonnes (i.e. /not/ in `TEU`) – otherwise the current implementation goes nuclear.
    //        We are investingating whether the iLEAP Data model needs to be updated for the `TEU` unit case.
    //        This function will be updated as we move along.
    let PactMappedFields {
        product_id_type,
        data_schema_id,
        id,
        product_name_company,
        declared_unit,
        unitary_product_amount,
        p_cf_excluding_biogenic,
    } = (&ileap_type).into();

    // Fasten your seatbelts, we are about to create a ProductFootprint...
    ProductFootprint {
        id: PfId(Uuid::new_v4()),
        spec_version: SpecVersionString("2.2.0".to_string()),
        preceding_pf_ids: None,
        version: VersionInteger(1),
        created: Utc::now(),
        updated: None,
        status: PfStatus::Active,
        status_comment: None,
        validity_period_start: None,
        validity_period_end: None,
        company_name: company_name.to_string().into(),
        company_ids: CompanyIdSet(vec![Urn::from(company_urn.to_string())]),
        product_description: "".to_string(),
        product_ids: ProductIdSet(vec![Urn::from(format!(
            "urn:pathfinder:product:customcode:vendor-assigned:{product_id_type}:{id}"
        ))]),
        product_category_cpc: String::from("83117").into(),
        product_name_company: product_name_company.into(),
        comment: "".to_string(),
        pcf: CarbonFootprint {
            declared_unit,
            unitary_product_amount: unitary_product_amount.into(),
            p_cf_excluding_biogenic: p_cf_excluding_biogenic.into(),
            p_cf_including_biogenic: None,
            fossil_ghg_emissions: p_cf_excluding_biogenic.into(),
            fossil_carbon_content: PositiveDecimal::from(Decimal::from(0)),
            biogenic_carbon_content: PositiveDecimal::from(Decimal::from(0)),
            d_luc_ghg_emissions: None,
            land_management_ghg_emissions: None,
            other_biogenic_ghg_emissions: None,
            i_luc_ghg_emissions: None,
            biogenic_carbon_withdrawal: None,
            aircraft_ghg_emissions: None,
            characterization_factors,
            ipcc_characterization_factors_sources: characterization_factors_sources.into(),
            cross_sectoral_standards_used: CrossSectoralStandardSet(vec![
                CrossSectoralStandard::ISO14083,
            ]),
            product_or_sector_specific_rules: None, // TODO: get clarity on whether GLEC should be specified
            biogenic_accounting_methodology: None,
            boundary_processes_description: "".to_string(),
            reference_period_start: Utc::now(),
            reference_period_end: (Utc::now() + chrono::Duration::days(364)),
            geographic_scope: None,
            secondary_emission_factor_sources: None,
            exempted_emissions_percent: ExemptedEmissionsPercent(0.into()),
            exempted_emissions_description: "".to_string(),
            packaging_emissions_included: false,
            packaging_ghg_emissions: None,
            allocation_rules_description: None,
            uncertainty_assessment_description: None,
            primary_data_share: None,
            dqi: None,
            assurance: None,
        },
        extensions: Some(vec![DataModelExtension {
            spec_version: SpecVersionString::from("0.2.0".to_string()),
            data_schema: format!("https://api.ileap.sine.dev/{data_schema_id}.json"),
            documentation: Some("https://sine-fdn.github.io/ileap-extension/".to_string()),
            data: ileap_type,
        }]),
    }
}

fn to_char_factors(
    characterization_factors: Option<Vec<CharacterizationFactors>>,
) -> (
    CharacterizationFactors,
    Vec<IpccCharacterizationFactorsSource>,
) {
    let (characterization_factors, characterization_factors_sources) =
        match characterization_factors {
            None => (
                CharacterizationFactors::Ar5,
                vec![IpccCharacterizationFactorsSource::from("AR5".to_string())],
            ),
            Some(cf) => {
                if cf.is_empty() {
                    (
                        CharacterizationFactors::Ar5,
                        vec![IpccCharacterizationFactorsSource::from("AR5".to_string())],
                    )
                } else {
                    let cf: Vec<IpccCharacterizationFactorsSource> = cf
                        .iter()
                        .map(|cf| match cf {
                            CharacterizationFactors::Ar5 => {
                                IpccCharacterizationFactorsSource::from("AR5".to_string())
                            }
                            CharacterizationFactors::Ar6 => {
                                IpccCharacterizationFactorsSource::from("AR6".to_string())
                            }
                        })
                        .collect();

                    let characterization_factors = if cf
                        .contains(&IpccCharacterizationFactorsSource::from("AR5".to_string()))
                    {
                        CharacterizationFactors::Ar5
                    } else {
                        CharacterizationFactors::Ar6
                    };

                    (characterization_factors, cf)
                }
            }
        };
    (characterization_factors, characterization_factors_sources)
}

#[test]
fn ship_foot_to_pfc() {
    use crate::{GlecDistance, Tce};
    use rust_decimal_macros::dec;

    let ship_foot = ShipmentFootprint {
        shipment_id: "shipment-test".to_string(),
        tces: vec![
            Tce {
                tce_id: "tce-1-toc-rail-1".to_string(),
                prev_tce_ids: Some(vec![]),
                toc_id: Some("toc-rail-1".to_string()),
                hoc_id: None,
                shipment_id: "shipment-test".to_string(),
                mass: dec!(40000).into(),
                distance: GlecDistance::Actual(dec!(423).into()),
                transport_activity: dec!(16920).into(),
                co2e_wtw: dec!(118.44).into(),
                co2e_ttw: dec!(0).into(),
                consignment_id: None,
                packaging_or_tr_eq_type: None,
                packaging_or_tr_eq_amount: None,
                origin: None,
                destination: None,
                departure_at: None,
                arrival_at: None,
                flight_no: None,
                voyage_no: None,
                incoterms: None,
                nox_ttw: None,
                sox_ttw: None,
                ch4_ttw: None,
                pm_ttw: None,
            },
            Tce {
                tce_id: "tce-2-hoc-transshipment-1".to_string(),
                prev_tce_ids: Some(vec!["tce-1-toc-rail-1".to_string()]),
                toc_id: None,
                hoc_id: Some("hoc-transshipment-1".to_string()),
                shipment_id: "shipment-test".to_string(),
                mass: dec!(40000).into(),
                distance: GlecDistance::Actual(dec!(0).into()),
                transport_activity: dec!(0).into(),
                co2e_wtw: dec!(1320).into(),
                co2e_ttw: dec!(400).into(),
                consignment_id: None,
                packaging_or_tr_eq_type: None,
                packaging_or_tr_eq_amount: None,
                origin: None,
                destination: None,
                departure_at: None,
                arrival_at: None,
                flight_no: None,
                voyage_no: None,
                incoterms: None,
                nox_ttw: None,
                sox_ttw: None,
                ch4_ttw: None,
                pm_ttw: None,
            },
            Tce {
                tce_id: "tce-3-toc-road-1".to_string(),
                prev_tce_ids: Some(vec!["tce-2-hoc-transshipment-1".to_string()]),
                toc_id: Some("toc-road-1".to_string()),
                hoc_id: None,
                shipment_id: "shipment-test".to_string(),
                mass: dec!(40000).into(),
                distance: GlecDistance::Actual(dec!(423).into()),
                transport_activity: dec!(16920).into(),
                co2e_wtw: dec!(1692.62).into(),
                co2e_ttw: dec!(1505.88).into(),
                consignment_id: None,
                packaging_or_tr_eq_type: None,
                packaging_or_tr_eq_amount: None,
                origin: None,
                destination: None,
                departure_at: None,
                arrival_at: None,
                flight_no: None,
                voyage_no: None,
                incoterms: None,
                nox_ttw: None,
                sox_ttw: None,
                ch4_ttw: None,
                pm_ttw: None,
            },
        ]
        .into(),
        mass: "40000".to_string(),
        volume: None,
        number_of_items: None,
        type_of_items: None,
    };

    let pfc = to_pcf(ship_foot, "test", "urn:test", None);

    assert_eq!(
        pfc.product_name_company.0,
        "ShipmentFootprint with id shipment-test"
    );
    assert_eq!(pfc.pcf.declared_unit, DeclaredUnit::TonKilometer);
    assert_eq!(pfc.pcf.unitary_product_amount.0, dec!(33840));
    assert_eq!(pfc.pcf.p_cf_excluding_biogenic.0, dec!(3131.06));
}

#[test]
fn toc_to_pcf() {
    use crate::{
        EnergyCarrier, EnergyCarrierType, Feedstock, FeedstockType, TemperatureControl, Toc,
        TocCo2eIntensityThroughput, TransportMode,
    };
    use rust_decimal_macros::dec;

    let toc = Toc {
        toc_id: "toc-test".to_string(),
        mode: TransportMode::Rail,
        load_factor: Some(dec!(0.6).to_string()),
        empty_distance_factor: Some(dec!(0.33).to_string()),
        temperature_control: Some(TemperatureControl::Ambient),
        truck_loading_sequence: None,
        energy_carriers: vec![EnergyCarrier {
            energy_carrier: EnergyCarrierType::Electric,
            feedstocks: Some(vec![Feedstock {
                feedstock: FeedstockType::Grid,
                feedstock_percentage: None,
                region_provenance: Some("Europe".to_string()),
            }]),
            energy_consumption: None,
            energy_consumption_unit: Some(crate::EnergyConsumptionUnit::MJ),
            emission_factor_wtw: dec!(97).into(),
            emission_factor_ttw: dec!(0).into(),
        }]
        .into(),
        co2e_intensity_wtw: dec!(0.007).into(),
        co2e_intensity_ttw: dec!(0).into(),
        co2e_intensity_throughput: TocCo2eIntensityThroughput::Tkm,
        is_verified: true,
        is_accredited: true,
        description: None,
        air_shipping_option: None,
        flight_length: None,
        glec_data_quality_index: None,
    };

    let pfc = to_pcf(toc, "test", "urn:test", None);

    assert_eq!(pfc.product_name_company.0, "TOC with ID toc-test");
    assert_eq!(pfc.pcf.declared_unit, DeclaredUnit::TonKilometer);
    assert_eq!(pfc.pcf.unitary_product_amount.0, dec!(1));
    assert_eq!(pfc.pcf.p_cf_excluding_biogenic.0, dec!(0.007));
}

#[test]
fn hoc_to_pfc() {
    use crate::{
        EnergyCarrier, EnergyCarrierType, Hoc, HubType, TemperatureControl, TransportMode,
    };
    use rust_decimal_macros::dec;

    let hoc = Hoc {
        hoc_id: "hoc-test".to_string(),
        hub_type: HubType::Transshipment,
        temperature_control: Some(TemperatureControl::Refrigerated),
        inbound_transport_mode: Some(TransportMode::Road),
        outbound_transport_mode: Some(TransportMode::Rail),
        is_verified: true,
        is_accredited: true,
        hub_location: None,
        packaging_or_tr_eq_type: None,
        packaging_or_tr_eq_amount: None,
        description: None,
        energy_carriers: vec![
            EnergyCarrier {
                energy_carrier: EnergyCarrierType::Diesel,
                feedstocks: None,
                energy_consumption: None,
                energy_consumption_unit: Some(crate::EnergyConsumptionUnit::Kg),
                emission_factor_wtw: dec!(4.13).into(),
                emission_factor_ttw: dec!(3.17).into(),
            },
            EnergyCarrier {
                energy_carrier: EnergyCarrierType::Electric,
                feedstocks: None,
                energy_consumption: None,
                energy_consumption_unit: Some(crate::EnergyConsumptionUnit::MJ),
                emission_factor_wtw: dec!(97).into(),
                emission_factor_ttw: dec!(0).into(),
            },
        ]
        .into(),
        co2e_intensity_wtw: dec!(33).into(),
        co2e_intensity_ttw: dec!(10).into(),
        co2e_intensity_throughput: HocCo2eIntensityThroughput::Tonnes,
    };

    let pfc = to_pcf(hoc, "test", "urn:test", None);

    assert_eq!(pfc.product_name_company.0, "HOC with ID hoc-test");
    assert_eq!(pfc.pcf.declared_unit, DeclaredUnit::Kilogram);
    assert_eq!(pfc.pcf.unitary_product_amount.0, dec!(1000));
    assert_eq!(pfc.pcf.p_cf_excluding_biogenic.0, dec!(33));
}
