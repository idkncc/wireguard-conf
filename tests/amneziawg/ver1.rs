use wireguard_conf::prelude::*;

#[test]
fn random() {
    let settings = AmneziaWG::random_v1();

    assert!(settings.validate().is_ok());
}

#[test]
fn validate_jc() {
    let mut settings = AmneziaWG::random_v1();

    match settings {
        AmneziaWG::V1(ref mut settings) => {
            settings.jc = 9999;
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jc"))
    );
}

#[test]
fn validate_jmin() {
    let mut settings = AmneziaWG::random_v1();

    match settings {
        AmneziaWG::V1(ref mut settings) => {
            settings.jmin = 100;
            settings.jmax = 50;
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin"))
    );
}

#[test]
fn validate_jmax() {
    let mut settings = AmneziaWG::random_v1();

    match settings {
        AmneziaWG::V1(ref mut settings) => {
            settings.jmax = 9999;
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmax"))
    );
}

#[test]
fn validate_s1() {
    let mut settings = AmneziaWG::random_v1();

    match settings {
        AmneziaWG::V1(ref mut settings) => {
            settings.s1 = 9999;
        }
        _ => unreachable!(),
    }
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S1"))
    );

    // s1 + 56 != s2
    let mut settings = AmneziaWG::random_v1();
    match settings {
        AmneziaWG::V1(ref mut settings) => {
            settings.s1 = 100;
            settings.s2 = 156;
        }
        _ => unreachable!(),
    }
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S1"))
    );
}

#[test]
fn validate_s2() {
    let mut settings = AmneziaWG::random_v1();

    match settings {
        AmneziaWG::V1(ref mut settings) => {
            settings.s2 = 9999;
        }
        _ => unreachable!(),
    }
    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S2"))
    );
}

#[test]
fn validate_h1_h2_h3_h4() {
    let mut settings = AmneziaWG::random_v1();

    match settings {
        AmneziaWG::V1(ref mut settings) => {
            settings.h1 = 1111; // same
            settings.h2 = 1111; // same
            settings.h3 = 3333;
            settings.h4 = 4444;
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H1/H2/H3/H4"))
    );
}
