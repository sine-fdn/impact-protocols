use ileap_data_model::*;
use rust_decimal_macros::dec;

#[test]
fn test_transport_mode_deser() {
    let tests = vec![
        (r#""Road""#, TransportMode::Road),
        (r#""Rail""#, TransportMode::Rail),
        (r#""Air""#, TransportMode::Air),
        (r#""Sea""#, TransportMode::Sea),
        (r#""InlandWaterway""#, TransportMode::InlandWaterway),
        //(r#""Hub""#, TransportMode::Hub),
    ];

    for (json, expected) in tests {
        let mode: TransportMode = serde_json::from_str(json).unwrap();
        assert_eq!(mode, expected);
    }
}

#[test]
fn test_truck_loading_sequence_deser() {
    let tests = vec![
        (r#""LTL""#, TruckLoadingSequence::Ltl),
        (r#""FTL""#, TruckLoadingSequence::Ftl),
    ];

    for (json, expected) in tests {
        let truck_loading_sequence: TruckLoadingSequence = serde_json::from_str(json).unwrap();
        assert_eq!(truck_loading_sequence, expected);
    }
}

#[test]
fn test_temperature_control_deser() {
    let tests = vec![
        (r#""ambient""#, TemperatureControl::Ambient),
        (r#""refrigerated""#, TemperatureControl::Refrigerated),
        (r#""mixed""#, TemperatureControl::Mixed),
    ];

    for (json, expected) in tests {
        let truck_loading_sequence: TemperatureControl = serde_json::from_str(json).unwrap();
        assert_eq!(truck_loading_sequence, expected);
    }
}

#[test]
fn test_toc_deser() {
    let (json, expected) = (
        r#"{"tocId":"4561230","isVerified":true,"isAccredited":true,"mode":"Road","temperatureControl":"refrigerated","truckLoadingSequence":"FTL","energyCarriers":[{"energyCarrier":"Diesel","emissionFactorWTW":"3.6801","emissionFactorTTW":"3.2801", "relativeShare": "1"}],"co2eIntensityWTW":"3.6801","co2eIntensityTTW":"3.2801","transportActivityUnit":"tkm"}"#,
        Toc {
            toc_id: "4561230".to_string(),
            certifications: None,
            mode: TransportMode::Road,
            temperature_control: Some(TemperatureControl::Refrigerated),
            truck_loading_sequence: Some(TruckLoadingSequence::Ftl),
            energy_carriers: vec![EnergyCarrier {
                energy_carrier: EnergyCarrierType::Diesel,
                emission_factor_wtw: dec!(3.6801).into(),
                emission_factor_ttw: dec!(3.2801).into(),
                feedstocks: None,
                energy_consumption: None,
                energy_consumption_unit: None,
                relative_share: dec!(1).into(),
            }]
            .into(),
            co2e_intensity_wtw: dec!(3.6801).into(),
            co2e_intensity_ttw: dec!(3.2801).into(),
            transport_activity_unit: TransportActivityUnit::Tkm,
            description: None,
            load_factor: None,
            empty_distance_factor: None,
            air_shipping_option: None,
            flight_length: None,
        },
    );

    let serialized = serde_json::to_string(&expected).unwrap();
    println!("seria: {serialized}");
    println!("input: {json}");

    let toc: Toc = serde_json::from_str(json).unwrap();
    assert_eq!(toc, expected)
}

#[test]
fn test_ship_foot_deser() {
    let (json, expected) = (
        r#"{"mass":"87","shipmentId":"1237890","tces":[{"tceId":"abcdef", "prevTceIds": [], "tocId":"truck-40t-euro5-de","shipmentId":"1237890","mass":"87","distance":{"actual":"423"},"transportActivity":"36.801","co2eWTW":"36.801","co2eTTW":"3.2801"}]}"#,
        ShipmentFootprint {
            mass: "87".to_string(),
            volume: None,
            shipment_id: "1237890".to_string(),
            tces: NonEmptyVec::<Tce>::from(vec![Tce {
                tce_id: "abcdef".to_string(),
                prev_tce_ids: Some(vec![]),
                toc_id: Some("truck-40t-euro5-de".to_string()),
                hoc_id: None,
                shipment_id: "1237890".to_string(),
                consignment_id: None,
                mass: dec!(87).into(),
                packaging_or_tr_eq_type: None,
                packaging_or_tr_eq_amount: None,
                distance: GlecDistance::Actual(dec!(423).into()),
                origin: None,
                destination: None,
                transport_activity: dec!(36.801).into(),
                departure_at: None,
                arrival_at: None,
                flight_no: None,
                voyage_no: None,
                incoterms: None,
                co2e_wtw: dec!(36.801).into(),
                co2e_ttw: dec!(3.2801).into(),
                nox_ttw: None,
                sox_ttw: None,
                ch4_ttw: None,
                pm_ttw: None,
            }]),
        },
    );

    let ship_foot: ShipmentFootprint = serde_json::from_str(json).unwrap();
    assert_eq!(ship_foot, expected);

    let json = r#"{"mass":"87","shipmentId":"1237890","tces":[{"tceId":"abcdef", "prevTceIds": [], "tocId":"truck-40t-euro5-de","shipmentId":"1237890","mass":87,"distance":{"actual":423},"transportActivity":"36.801","co2eWTW":"36.801","co2eTTW":"3.2801"}]}"#;

    let nok_ship_foot: Result<ShipmentFootprint, serde_json::Error> = serde_json::from_str(json);

    assert!(nok_ship_foot.is_err());
}

#[test]
fn test_energyconsumptionunit_deser() {
    use EnergyConsumptionUnit::*;
    let test_vectors = vec![
        ("\"l\"", L),
        ("\"kg\"", Kg),
        ("\"kWh\"", KWh),
        ("\"MJ\"", MJ),
    ];

    for (expect, input) in &test_vectors {
        assert_eq!(expect, &serde_json::to_string(input).unwrap());
    }
}
