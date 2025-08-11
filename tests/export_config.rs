use wireguard_conf::{as_ipnet, prelude::*};

fn get_example_data() -> (Interface, Peer) {
    let address = as_ipnet!("10.0.0.1/24");

    let peer = PeerBuilder::new()
        .add_allowed_ip(as_ipnet!("10.0.0.2/32"))
        .private_key(PrivateKey::random())
        .build();
    let interface = InterfaceBuilder::new()
        .address(address)
        .listen_port(55870)
        .set_dns(vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()])
        .endpoint("vpn.example.com".to_string())
        .add_peer(peer.clone())
        .build();

    (interface, peer)
}

#[test]
pub fn export_interface() {
    let (interface, _peer) = get_example_data();

    println!("{interface}");
    // TODO: asserts
}

#[test]
pub fn export_client_interface() {
    let (server_interface, peer) = get_example_data();

    let mut client_interface = peer.to_interface(&server_interface).unwrap();
    client_interface.peers[0]
        .allowed_ips
        .push(as_ipnet!("0.0.0.0/0"));

    let config = client_interface.to_string();
    let lines: Vec<&str> = config.split("\n").collect();

    println!("{client_interface}");

    assert!(lines[0] == "[Interface]");
    assert!(lines[1].starts_with("Address = "));
    assert!(lines[2].starts_with("PrivateKey = "));
    assert!(lines[3].starts_with("DNS = "));
    // TODO: asserts
}
