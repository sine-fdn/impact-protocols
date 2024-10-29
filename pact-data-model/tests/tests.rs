use chrono::{TimeZone, Utc};
use pact_data_model::*;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[test]
fn test_deser_geography() {
    let cf = CarbonFootprint {
        declared_unit: DeclaredUnit::Kilogram,
        unitary_product_amount: StrictlyPositiveDecimal(Decimal::from_f64(1.0).unwrap()),
        p_cf_excluding_biogenic: PositiveDecimal(Decimal::from_f64(1.0).unwrap()),
        p_cf_including_biogenic: None,
        fossil_ghg_emissions: PositiveDecimal(Decimal::from_f64(1.0).unwrap()),
        fossil_carbon_content: PositiveDecimal(Decimal::from_f64(0.0).unwrap()),
        biogenic_carbon_content: PositiveDecimal(Decimal::from_f64(0.0).unwrap()),
        d_luc_ghg_emissions: None,
        land_management_ghg_emissions: None,
        other_biogenic_ghg_emissions: None,
        i_luc_ghg_emissions: None,
        biogenic_carbon_withdrawal: None,
        aircraft_ghg_emissions: None,
        characterization_factors: CharacterizationFactors::Ar6,
        ipcc_characterization_factors_sources: IpccCharacterizationFactorsSources(vec![
            IpccCharacterizationFactorsSource::from("AR6".to_string()),
        ]),
        cross_sectoral_standards_used: CrossSectoralStandardSet(vec![
            CrossSectoralStandard::Ghgp,
            CrossSectoralStandard::ISO14044,
        ]),
        product_or_sector_specific_rules: None,
        biogenic_accounting_methodology: None,
        boundary_processes_description: String::from(""),
        reference_period_start: Utc.with_ymd_and_hms(2021, 1, 1, 00, 00, 00).unwrap(),
        reference_period_end: Utc.with_ymd_and_hms(2022, 1, 1, 00, 00, 00).unwrap(),
        geographic_scope: None,
        secondary_emission_factor_sources: None,
        exempted_emissions_percent: ExemptedEmissionsPercent(1.0),
        exempted_emissions_description: String::from(""),
        packaging_emissions_included: true,
        packaging_ghg_emissions: None,
        allocation_rules_description: None,
        uncertainty_assessment_description: None,
        primary_data_share: None,
        dqi: None,
        assurance: None,
    };

    // test case with geography "global" ; i.e. no further geography defined
    {
        const GEOGRAPHY_SCOPE_NONE: &str = r#"
            {
                "declaredUnit": "kilogram",
                "unitaryProductAmount": "1",
                "pCfExcludingBiogenic": "1",
                "fossilGhgEmissions": "1",
                "fossilCarbonContent": "0",
                "biogenicCarbonContent": "0",
                "characterizationFactors": "AR6",
                "ipccCharacterizationFactorsSources": [
                "AR6"
                ],
                "crossSectoralStandardsUsed": [
                "GHGP Product",
                "ISO14044"
                ],
                "boundaryProcessesDescription": "",
                "referencePeriodStart": "2021-01-01T00:00:00Z",
                "referencePeriodEnd": "2022-01-01T00:00:00Z",
                "exemptedEmissionsPercent": 1,
                "exemptedEmissionsDescription": "",
                "packagingEmissionsIncluded": true
            }
    "#;

        assert_eq!(cf, serde_json::from_str(GEOGRAPHY_SCOPE_NONE).unwrap());
    }

    // test case with "`geographyCountry` being set
    {
        const GEOGRAPHY_SCOPE_FR: &str = r#"
            {
                "declaredUnit": "kilogram",
                "unitaryProductAmount": "1",
                "pCfExcludingBiogenic": "1",
                "fossilGhgEmissions": "1",
                "fossilCarbonContent": "0",
                "biogenicCarbonContent": "0",
                "characterizationFactors": "AR6",
                "ipccCharacterizationFactorsSources": [
                "AR6"
                ],
                "crossSectoralStandardsUsed": [
                "GHGP Product",
                "ISO14044"
                ],
                "boundaryProcessesDescription": "",
                "referencePeriodStart": "2021-01-01T00:00:00Z",
                "referencePeriodEnd": "2022-01-01T00:00:00Z",
                "exemptedEmissionsPercent": 1,
                "exemptedEmissionsDescription": "",
                "packagingEmissionsIncluded": true,
                "geographyCountry": "FR"
            }
    "#;

        let cf_test = CarbonFootprint {
            geographic_scope: Some(GeographicScope::Country {
                geography_country: ISO3166CC("FR".to_string()),
            }),
            ..(cf.clone())
        };

        assert_eq!(cf_test, serde_json::from_str(GEOGRAPHY_SCOPE_FR).unwrap());
    }

    // test case with "`geographyRegionOrSubregion` being set
    {
        const GEOGRAPHY_SCOPE_REGION: &str = r#"
            {
                "declaredUnit": "kilogram",
                "unitaryProductAmount": "1",
                "pCfExcludingBiogenic": "1",
                "fossilGhgEmissions": "1",
                "fossilCarbonContent": "0",
                "biogenicCarbonContent": "0",
                "characterizationFactors": "AR6",
                "ipccCharacterizationFactorsSources": [
                "AR6"
                ],
                "crossSectoralStandardsUsed": [
                "GHGP Product",
                "ISO14044"
                ],
                "boundaryProcessesDescription": "",
                "referencePeriodStart": "2021-01-01T00:00:00Z",
                "referencePeriodEnd": "2022-01-01T00:00:00Z",
                "exemptedEmissionsPercent": 1,
                "exemptedEmissionsDescription": "",
                "packagingEmissionsIncluded": true,
                "geographyRegionOrSubregion": "Americas"
            }
    "#;

        let cf_test = CarbonFootprint {
            geographic_scope: Some(GeographicScope::Regional {
                geography_region_or_subregion: UNRegionOrSubregion::Americas,
            }),
            ..(cf.clone())
        };

        assert_eq!(
            cf_test,
            serde_json::from_str(GEOGRAPHY_SCOPE_REGION).unwrap()
        );
    }

    // test case with "`geographyCountrySubdivision` being set
    {
        const GEOGRAPHY_SCOPE_REGION: &str = r#"
            {
                "declaredUnit": "kilogram",
                "unitaryProductAmount": "1",
                "pCfExcludingBiogenic": "1",
                "fossilGhgEmissions": "1",
                "fossilCarbonContent": "0",
                "biogenicCarbonContent": "0",
                "characterizationFactors": "AR6",
                "ipccCharacterizationFactorsSources": [
                "AR6"
                ],
                "crossSectoralStandardsUsed": [
                "GHGP Product",
                "ISO14044"
                ],
                "boundaryProcessesDescription": "",
                "referencePeriodStart": "2021-01-01T00:00:00Z",
                "referencePeriodEnd": "2022-01-01T00:00:00Z",
                "exemptedEmissionsPercent": 1,
                "exemptedEmissionsDescription": "",
                "packagingEmissionsIncluded": true,
                "geographyCountrySubdivision": "FR-48"
            }
    "#;

        let cf_test = CarbonFootprint {
            geographic_scope: Some(GeographicScope::Subdivision {
                geography_country_subdivision: NonEmptyString("FR-48".to_string()),
            }),
            ..(cf.clone())
        };

        assert_eq!(
            cf_test,
            serde_json::from_str(GEOGRAPHY_SCOPE_REGION).unwrap()
        );
    }
}
