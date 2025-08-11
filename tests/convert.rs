use wireguard_conf::{as_ipnet, prelude::*};

#[test]
fn interface_to_peer() {
    let address = as_ipnet!("10.3.2.1/24");
    let endpoint = "server.example.com".to_string();
    let interface = InterfaceBuilder::new()
        .address(address)
        .listen_port(55870)
        .set_dns(vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()])
        .add_dns("1.1.1.1".to_string())
        .endpoint(endpoint.clone())
        .build();

    let result_peer = interface.to_peer();
    let expected_peer = PeerBuilder::new()
        .add_allowed_ip(as_ipnet!("10.3.2.1/24"))
        .endpoint(endpoint.clone())
        .private_key(interface.private_key.clone())
        .build();

    assert_eq!(result_peer.endpoint, expected_peer.endpoint);
    assert_eq!(result_peer.allowed_ips, expected_peer.allowed_ips);
    assert_eq!(result_peer.key, expected_peer.key)
}

// TODO: `Peer::to_interface` test
// #[test]
// fn peer_to_interface() {
// }
