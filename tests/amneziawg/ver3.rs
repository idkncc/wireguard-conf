use wireguard_conf::prelude::*;

#[test]
fn random() {
    let settings = AmneziaWG::random_v3();

    assert!(settings.validate().is_ok());
}

#[test]
fn validate_jc() {
    let mut settings = AmneziaWG::random_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.jc = Some(127);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jc"))
    );
}

#[test]
// FIX: Старые границы
fn validate_jmin() {
    let mut settings = AmneziaWG::random_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.jmin = Some(63);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.jmin = Some(1025);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin"))
    );
}

#[test]
// FIX: Старые границы
fn validate_jmax() {
    let mut settings = AmneziaWG::random_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.jmax = Some(63);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmax"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.jmax = Some(1025);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmax"))
    );
}

#[test]
fn validate_jmin_jmax() {
    let mut settings = AmneziaWG::random_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.jmin = Some(100);
            settings.jmax = Some(64);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin >= Jmax"))
    );
}

#[test]
fn validate_s1() {
    let mut settings = AmneziaWG::empty_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.s1 = Some(65);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S1"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = Some(HeaderProtectionKey::random());
            settings.s1 = Some(11);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S1"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = None;
        }
        _ => unreachable!(),
    }

    assert_eq!(settings.validate(), Ok(()));
}

#[test]
fn validate_s2() {
    let mut settings = AmneziaWG::empty_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.s2 = Some(65);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S2"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = Some(HeaderProtectionKey::random());
            settings.s2 = Some(11);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S2"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = None;
        }
        _ => unreachable!(),
    }

    assert_eq!(settings.validate(), Ok(()));
}

#[test]
fn validate_s3() {
    let mut settings = AmneziaWG::empty_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.s3 = Some(65);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S3"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = Some(HeaderProtectionKey::random());
            settings.s3 = Some(11);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S3"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = None;
        }
        _ => unreachable!(),
    }

    assert_eq!(settings.validate(), Ok(()));
}

#[test]
fn validate_s4() {
    let mut settings = AmneziaWG::empty_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.s4 = Some(33);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S4"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = Some(HeaderProtectionKey::random());
            settings.s4 = Some(11);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S4"))
    );

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.header_protection_key = None;
        }
        _ => unreachable!(),
    }

    assert_eq!(settings.validate(), Ok(()));
}

#[test]
fn validate_h_overlap() {
    let mut settings = AmneziaWG::empty_v3();

    match settings {
        AmneziaWG::V3(ref mut settings) => {
            settings.h1 = Some("100-200".parse().expect("valid uint range"));
            settings.h2 = Some("150-250".parse().expect("valid uint range"));
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H ranges overlap"))
    );
}
