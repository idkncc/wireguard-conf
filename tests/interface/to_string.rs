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
