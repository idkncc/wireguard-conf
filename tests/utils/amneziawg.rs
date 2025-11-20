#![cfg(feature = "amneziawg")]
use wireguard_conf::prelude::*;

#[test]
fn random() {
    let settings = AmneziaSettings::random();

    assert!(settings.validate().is_ok());
}

#[test]
fn validate_jc() {
    let mut settings = AmneziaSettings::random();

    settings.jc = 9999;

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jc".to_string()))
    );
}

#[test]
fn validate_jmin() {
    let mut settings = AmneziaSettings::random();

    settings.jmin = 100;
    settings.jmax = 50;

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin".to_string()))
    );
}

#[test]
fn validate_jmax() {
    let mut settings = AmneziaSettings::random();

    settings.jmax = 9999;

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmax".to_string()))
    );
}

#[test]
fn validate_s1() {
    let mut settings = AmneziaSettings::random();

    settings.s1 = 9999;
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S1".to_string()))
    );

    // s1 + 56 != s2
    settings.s1 = 100;
    settings.s2 = 156;
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S1".to_string()))
    );
}

#[test]
fn validate_s2() {
    let mut settings = AmneziaSettings::random();

    settings.s2 = 9999;
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S2".to_string()))
    );
}

#[test]
fn validate_h1_h2_h3_h4() {
    let mut settings = AmneziaSettings::random();

    settings.h1 = 1111; // same
    settings.h2 = 1111; // same
    settings.h3 = 3333;
    settings.h4 = 4444;
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting(
            "H1/H2/H3/H4".to_string()
        ))
    );
}
