use indoc::formatdoc;
use wireguard_conf::{as_ipnet, prelude::*};

#[test]
fn ipv6_and_multiple_addresses() {
    let mut server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("fd3d:1209:d994::1/48"), as_ipnet!("10.0.0.1/24")])
        .build();

    let client = PeerBuilder::new()
        .allowed_ips([as_ipnet!("fd3d:1209:d994::2/128"), as_ipnet!("10.0.0.2/32")])
        .build();
    server_interface.peers.push(client.clone());

    let client_interface = client
        .to_interface(&server_interface)
        .expect("failed to create client interface");

    let server_privkey = server_interface.private_key.clone();
    let server_publickey = PublicKey::from(&server_privkey);

    let client_privkey = client_interface.private_key.clone();
    let client_publickey = PublicKey::from(&client_privkey);

    let server_conf = server_interface.to_string();
    assert_eq!(
        server_conf,
        formatdoc! {"
            [Interface]
            Address = fd3d:1209:d994::1/48,10.0.0.1/24
            PrivateKey = {server_privkey}
            
            [Peer]
            AllowedIPs = fd3d:1209:d994::2/128,10.0.0.2/32
            PublicKey = {client_publickey}
            
        "}
    );

    let client_conf = client_interface.to_string();
    assert_eq!(
        client_conf,
        formatdoc! {"
            [Interface]
            Address = fd3d:1209:d994::2/48,10.0.0.2/24
            PrivateKey = {client_privkey}
            
            [Peer]
            AllowedIPs = fd3d:1209:d994::1/48,10.0.0.1/24
            PublicKey = {server_publickey}
            
        "}
    );
}
