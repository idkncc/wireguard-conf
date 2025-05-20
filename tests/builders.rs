use ipnet::Ipv4Net;
use wireguard_conf::{as_ipnet, prelude::*};

#[test]
fn interface_builder() {
    let address = as_ipnet!("10.3.2.1/24");

    let interface = InterfaceBuilder::new()
        .address(address)
        .listen_port(55870)
        .set_dns(vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()])
        .add_dns("1.1.1.1".to_string())
        .set_table(Table::RoutingTable(12345))
        .set_mtu(1450)
        .add_pre_up("echo PreUp".to_string())
        .add_pre_down("echo PreDown".to_string())
        .add_pre_down("echo \"There's can be multiple commands\"".to_string())
        .add_post_up("echo PostUp".to_string())
        .add_post_down("echo PostDown".to_string())
        .add_peer(
            PeerBuilder::new()
                .set_allowed_ips(vec!["0.0.0.0/0".parse().unwrap()])
                .add_allowed_ip("10.3.2.2/24".parse().unwrap())
                .public_key(PublicKey::from(&PrivateKey::random()))
                .build(),
        )
        .build();

    assert_eq!(interface.address, address);
    assert_eq!(interface.listen_port, Some(55870));
    assert_eq!(interface.dns.len(), 3);
    assert_eq!(interface.table, Some(Table::RoutingTable(12345)));
    assert_eq!(interface.mtu, Some(1450));

    assert_eq!(interface.pre_up.len(), 1);
    assert_eq!(interface.pre_up[0], "echo PreUp".to_string());
    assert_eq!(interface.pre_down.len(), 2);
    assert_eq!(interface.pre_down[0], "echo PreDown".to_string());
    assert_eq!(
        interface.pre_down[1],
        "echo \"There's can be multiple commands\"".to_string()
    );
    assert_eq!(interface.post_up.len(), 1);
    assert_eq!(interface.post_up[0], "echo PostUp".to_string());
    assert_eq!(interface.post_down.len(), 1);
    assert_eq!(interface.post_down[0], "echo PostDown".to_string());

    assert_eq!(interface.peers.len(), 1);

    println!("InterfaceBuilder complete config:");
    println!("{interface}");
}

#[test]
fn peer_builder() {
    let allowed_ip = as_ipnet!("10.3.2.1/32");
    let endpoint = "peer.example.com".to_string();

    let peer = PeerBuilder::new()
        .set_allowed_ips(vec![allowed_ip])
        .endpoint(endpoint.clone())
        .build();

    assert_eq!(peer.allowed_ips, vec![allowed_ip]);
    assert_eq!(peer.endpoint, Some(endpoint));
}
