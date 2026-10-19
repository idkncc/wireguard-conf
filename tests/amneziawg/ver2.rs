use wireguard_conf::prelude::*;

#[test]
fn random() {
    let settings = AmneziaWG::random_v2();

    assert!(settings.validate().is_ok());
}

#[test]
fn validate_jc() {
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.jc = Some(11);
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
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.jmin = Some(63);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmin"))
    );

    match settings {
        AmneziaWG::V2(ref mut settings) => {
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
fn validate_jmax() {
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.jmax = Some(63);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("Jmax"))
    );

    match settings {
        AmneziaWG::V2(ref mut settings) => {
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
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
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
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.s1 = Some(65);
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
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.s2 = Some(65);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S2"))
    );
}

#[test]
fn validate_s3() {
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.s3 = Some(65);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S3"))
    );
}

#[test]
fn validate_s4() {
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.s4 = Some(33);
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("S4"))
    );
}

#[test]
fn validate_h1() {
    let mut settings = AmneziaWG::empty_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.h1 = Some(HRange::new(200, 100));
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H1"))
    );
}

#[test]
fn validate_h2() {
    let mut settings = AmneziaWG::empty_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.h2 = Some(HRange::new(200, 100));
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H2"))
    );
}

#[test]
fn validate_h3() {
    let mut settings = AmneziaWG::empty_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.h3 = Some(HRange::new(200, 100));
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H3"))
    );
}

#[test]
fn validate_h4() {
    let mut settings = AmneziaWG::empty_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.h4 = Some(HRange::new(200, 100));
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H4"))
    );
}

#[test]
fn validate_h_overlap() {
    let mut settings = AmneziaWG::random_v2();

    match settings {
        AmneziaWG::V2(ref mut settings) => {
            settings.h1 = Some(HRange::new(1000, 2000));
            settings.h2 = Some(HRange::new(1500, 5000));
            settings.h3 = Some(HRange::new(5000, 7000));
            settings.h4 = Some(HRange::new(8000, 9000));
        }
        _ => unreachable!(),
    }

    assert_eq!(
        settings.validate(),
        Err(WireguardError::InvalidAmneziaSetting("H ranges overlap"))
    );
}
