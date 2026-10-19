use wireguard_conf::prelude::*;

fn roundtrip(settings: &AmneziaWG) -> AmneziaWG {
    let json = serde_json::to_string(settings).expect("settings are serializable");
    serde_json::from_str(&json).expect("settings are deserializable")
}

fn kind(settings: &AmneziaWG) -> u8 {
    match settings {
        AmneziaWG::V1(_) => 1,
        AmneziaWG::V2(_) => 2,
        AmneziaWG::V3(_) => 3,
    }
}

#[test]
fn v1() {
    let settings = AmneziaWG::random_v1();
    let decoded = roundtrip(&settings);

    assert_eq!(kind(&decoded), 3);
    assert_eq!(decoded.to_string(), settings.to_string());
}

#[test]
fn v2() {
    let settings = AmneziaWG::random_v2();
    let decoded = roundtrip(&settings);

    assert_eq!(kind(&decoded), 3);
    assert_eq!(decoded.to_string(), settings.to_string());
}

#[test]
fn v3() {
    let settings = AmneziaWG::random_v3();

    assert_eq!(roundtrip(&settings), settings);
}

/// A config is read as the newest AmneziaWG version, that it is valid for.
#[test]
fn newest_version_wins() {
    for json in [
        r#"{}"#,
        r#"{"jc":5}"#,
        r#"{"h1":"1"}"#,
        r#"{"h1":"1-4"}"#,
        r#"{"h1":5}"#,
        r#"{"s1":15,"s3":15,"i1":"<b 0x01>"}"#,
        r#"{"random_trailers":true,"disable_cookies":false}"#,
        r#"{"header_protection_key":"W4Jf2G+i7RjcdNJCvJaf1pQ+3EWWyzj99Z1o6W/k7zU="}"#,
    ] {
        let settings: AmneziaWG =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{json} is valid: {e}"));

        assert_eq!(kind(&settings), 3, "{json}");
    }
}

#[test]
fn invalid_settings_fail() {
    for json in [
        r#"{"jc":-1}"#,     // not an unsigned value
        r#"{"jc":300}"#,    // `Jc` doesn't fit `u8`
        r#"{"jc":"five"}"#, // not a number
        r#"{"h1":-1}"#,     // not an unsigned value
        r#"{"h1":""}"#,     // not a range
    ] {
        let settings = serde_json::from_str::<AmneziaWG>(json);
        assert!(settings.is_err(), "{json} must not be deserializable");
    }
}

#[test]
fn unrecognized_field_names_become_empty_v3() {
    for json in [
        r#"{"header_protection_kye":"typo"}"#,
        r#"{"not_a_setting":1}"#,
    ] {
        let settings: AmneziaWG =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{json} is valid: {e}"));

        assert_eq!(kind(&settings), 3, "{json}");
        assert_eq!(settings.to_string(), "", "{json}");
    }
}

#[test]
fn invalid_3_1_value_becomes_empty_v2() {
    let settings: AmneziaWG =
        serde_json::from_str(r#"{"random_trailers":"yes"}"#).expect("the fallback is valid");

    assert_eq!(kind(&settings), 2);
    assert_eq!(settings.to_string(), "");
}

#[test]
fn inverted_header_range_falls_back_to_v2() {
    let settings: AmneziaWG =
        serde_json::from_str(r#"{"h1":"5-1"}"#).expect("the fallback is valid");

    assert_eq!(kind(&settings), 2);
    assert_eq!(settings.to_string(), "H1 = 5-1\n");
}
