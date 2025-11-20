use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::IpNet;
use wireguard_conf::{as_ipnet, prelude::*};

#[test]
fn expect_no_private_key_provided() {
    let server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("10.0.0.1/24")])
        .build();
    let client_peer = PeerBuilder::new()
        .public_key(PublicKey::from(&PrivateKey::random()))
        // ^-- error is here, public key (instead of private) is provided.
        .allowed_ips([as_ipnet!("10.0.0.2/32")])
        .build();

    let client_interface_result =
        client_peer.to_interface(&server_interface, ToInterfaceOptions::new());

    assert_eq!(
        client_interface_result,
        Err(WireguardError::NoPrivateKeyProvided)
    )
}

#[test]
fn expect_no_assigned_ip_v4() {
    let server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("10.0.0.1/24")])
        .build();
    let client_peer = PeerBuilder::new()
        .allowed_ips([as_ipnet!("1.3.3.7/32")]) // error is here -- 10.0.0.1/24 doesn't contain 1.3.3.7
        .build();

    let client_interface_result =
        client_peer.to_interface(&server_interface, ToInterfaceOptions::new());

    assert_eq!(client_interface_result, Err(WireguardError::NoAssignedIP))
}

#[test]
fn expect_no_assigned_ip_v6() {
    let server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("fd00:1111:1111::1/48")])
        .build();
    let client_peer = PeerBuilder::new()
        .allowed_ips([as_ipnet!("fd00:2222:2222::1/128")]) // error is here -- fd00:1111:1111::1/48 doesn't contain fd00:2222:2222::1/128
        .build();

    let client_interface_result =
        client_peer.to_interface(&server_interface, ToInterfaceOptions::new());

    assert_eq!(client_interface_result, Err(WireguardError::NoAssignedIP))
}

#[test]
fn default_gateway_ipv4() {
    let server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("10.0.0.1/24")])
        .build();
    let client_peer = PeerBuilder::new()
        .allowed_ips([as_ipnet!("10.0.0.2/32")])
        .build();

    let client_interface = client_peer
        .to_interface(
            &server_interface,
            ToInterfaceOptions::new().default_gateway(true),
        )
        .expect("failed to generate interface");

    assert_eq!(
        client_interface.peers[0].allowed_ips,
        vec![IpNet::new_assert(Ipv4Addr::UNSPECIFIED.into(), 0)]
    )
}

#[test]
fn default_gateway_ipv6() {
    let server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("fd00::1/48")])
        .build();
    let client_peer = PeerBuilder::new()
        .allowed_ips([as_ipnet!("fd00::2/128")])
        .build();

    let client_interface = client_peer
        .to_interface(
            &server_interface,
            ToInterfaceOptions::new().default_gateway(true),
        )
        .expect("failed to generate interface");

    assert_eq!(
        client_interface.peers[0].allowed_ips,
        vec![IpNet::new_assert(Ipv6Addr::UNSPECIFIED.into(), 0)]
    )
}

#[test]
fn default_gateway_ipv4_and_ipv6() {
    let server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("10.0.0.1/24"), as_ipnet!("fd00::1/48")])
        .build();
    let client_peer = PeerBuilder::new()
        .allowed_ips([as_ipnet!("10.0.0.2/24"), as_ipnet!("fd00::2/128")])
        .build();

    let client_interface = client_peer
        .to_interface(
            &server_interface,
            ToInterfaceOptions::new().default_gateway(true),
        )
        .expect("failed to generate interface");

    assert_eq!(
        client_interface.peers[0].allowed_ips,
        vec![
            IpNet::new_assert(Ipv4Addr::UNSPECIFIED.into(), 0),
            IpNet::new_assert(Ipv6Addr::UNSPECIFIED.into(), 0)
        ]
    )
}
