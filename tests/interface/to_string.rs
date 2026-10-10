use indoc::formatdoc;
use wireguard_conf::{as_ipaddr, as_ipnet, prelude::*};

#[test]
fn empty() {
    let interface = InterfaceBuilder::new().build();

    assert_eq!(
        interface.to_string(),
        formatdoc! {
            "
            [Interface]
            Address = 0.0.0.0/0
            PrivateKey = {private_key}
            ", 
            private_key = interface.private_key
        }
    )
}

#[test]
fn multiple_addresses() {
    let interface = InterfaceBuilder::new()
        .add_network(as_ipnet!("1.2.3.4/16"))
        .add_network(as_ipnet!("fd00:dead:beef::1/48"))
        .build();

    assert_eq!(
        interface.to_string(),
        formatdoc! {
            "
            [Interface]
            Address = 1.2.3.4/16,fd00:dead:beef::1/48
            PrivateKey = {private_key}
            ", 
            private_key = interface.private_key
        }
    )
}

#[test]
fn non_cidr_addresses() {
    let interface = InterfaceBuilder::new()
        .add_address(as_ipaddr!("1.2.3.4"))
        .add_address(as_ipaddr!("fd00:dead:beef::1"))
        .build();

    assert_eq!(
        interface.to_string(),
        formatdoc! {
            "
            [Interface]
            Address = 1.2.3.4,fd00:dead:beef::1
            PrivateKey = {private_key}
            ", 
            private_key = interface.private_key
        }
    )
}

#[cfg(feature = "amneziawg")]
fn v3_settings_with_all_keys() -> (AmneziaWG, HeaderProtectionKey) {
    let header_protection_key = HeaderProtectionKey::random();

    let settings = AmneziaWG::builder_v3()
        .jc(5)
        .jmin(100)
        .jmax(500)
        .s1(15)
        .s2(16)
        .s3(17)
        .s4(18)
        .try_h1("1-2")
        .expect("valid range")
        .try_h2("3-4")
        .expect("valid range")
        .h3(5)
        .h4(6)
        .i1("<b 0x01>")
        .i2("<b 0x02>")
        .i3("<c>")
        .i4("<t>")
        .i5("<r>")
        .header_protection_key(header_protection_key.clone())
        .try_content_padding_addition("5-8")
        .expect("valid range")
        .try_rekey_after_time("10-20")
        .expect("valid range")
        .try_rekey_timeout("30-40")
        .expect("valid range")
        .try_reject_after_time("50-60")
        .expect("valid range")
        .try_keepalive_timeout("70-80")
        .expect("valid range")
        .try_max_handshake_attempts("2-5")
        .expect("valid range")
        .random_trailers(true)
        .disable_cookies(false)
        .build()
        .expect("valid settings");

    (settings, header_protection_key)
}

#[cfg(feature = "amneziawg")]
#[test]
fn exports_amnezia_obfuscation_settings() {
    let (settings, header_protection_key) = v3_settings_with_all_keys();

    let interface = InterfaceBuilder::new()
        .address([as_ipnet!("10.0.0.1/24")])
        .amnezia_settings(settings)
        .build();

    assert_eq!(
        interface.to_string(),
        formatdoc! {
            "
            [Interface]
            Address = 10.0.0.1/24
            PrivateKey = {private_key}

            Jc = 5
            Jmin = 100
            Jmax = 500
            S1 = 15
            S2 = 16
            S3 = 17
            S4 = 18
            H1 = 1-2
            H2 = 3-4
            H3 = 5
            H4 = 6
            I1 = <b 0x01>
            I2 = <b 0x02>
            I3 = <c>
            I4 = <t>
            I5 = <r>
            HeaderProtectionKey = {header_protection_key}
            ContentPaddingAddition = 5-8
            RekeyAfterTime = 10-20
            RekeyTimeout = 30-40
            RejectAfterTime = 50-60
            KeepaliveTimeout = 70-80
            MaxHandshakeAttempts = 2-5
            RandomTrailers = on
            DisableCookies = off

            ",
            private_key = interface.private_key,
            header_protection_key = header_protection_key,
        }
    );
}

#[cfg(feature = "amneziawg")]
#[test]
fn server_config_strips_client_only_settings() {
    let (settings, header_protection_key) = v3_settings_with_all_keys();

    let interface = InterfaceBuilder::new()
        .address([as_ipnet!("10.0.0.1/24")])
        .amnezia_settings(settings)
        .build();

    let server = interface.strip_client_data();

    assert_eq!(
        server.to_string(),
        formatdoc! {
            "
            [Interface]
            Address = 10.0.0.1/24
            PrivateKey = {private_key}

            S1 = 15
            S2 = 16
            S3 = 17
            S4 = 18
            H1 = 1-2
            H2 = 3-4
            H3 = 5
            H4 = 6
            HeaderProtectionKey = {header_protection_key}
            ContentPaddingAddition = 5-8
            RandomTrailers = on
            DisableCookies = off

            ",
            private_key = server.private_key,
            header_protection_key = header_protection_key,
        }
    );
}
