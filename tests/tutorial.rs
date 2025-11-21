use indoc::formatdoc;
use wireguard_conf::{as_ipnet, prelude::*};

#[test]
fn tutorial() {
    let server_private_key = PrivateKey::random();
    let client_private_key = PrivateKey::random();

    // Server's output interface.
    let output_interface = "ens0";

    let mut server_interface = InterfaceBuilder::new()
        .address([as_ipnet!("10.0.0.1/24")])
        .listen_port(51820)
        .private_key(server_private_key.clone())
        .dns(["1.1.1.1".to_string(), "1.0.0.1".to_string()])
        .endpoint("network.office.com")
        // Firewall configuration
        //  Sets up firewall forwards and NAT
        .post_up([
            "iptables -A FORWARD -i %i -j ACCEPT".to_string(),
            "iptables -A FORWARD -o %i -j ACCEPT".to_string(),
            format!(
                "iptables -t nat -A POSTROUTING -o {} -j MASQUERADE",
                output_interface
            ),
        ])
        .post_down([
            "iptables -D FORWARD -i %i -j ACCEPT".to_string(),
            "iptables -D FORWARD -o %i -j ACCEPT".to_string(),
            format!(
                "iptables -t nat -D POSTROUTING -o {} -j MASQUERADE",
                output_interface
            ),
        ])
        .build();

    let client_peer = PeerBuilder::new()
        .allowed_ips([as_ipnet!("10.0.0.2/32")])
        .private_key(client_private_key.clone())
        .build();

    server_interface.peers.push(client_peer.clone());

    let client_interface = client_peer
        .to_interface(
            &server_interface,
            ToInterfaceOptions::new()
                .default_gateway(true)
                .persistent_keepalive(25),
        )
        .expect("failed to get interface from peer");

    let server_conf = server_interface.to_string();
    let client_conf = client_interface.to_string();

    println!("=== SERVER CONFIG ===");
    println!("{server_conf}");
    println!("=== END SERVER CONFIG ===");

    println!();

    println!("=== CLIENT CONFIG ===");
    println!("{client_conf}");
    println!("=== END CLIENT CONFIG ===");

    // tests part
    let server_public_key = PublicKey::from(&server_private_key);
    let client_public_key = PublicKey::from(&client_private_key);
    assert_eq!(
        server_conf,
        formatdoc! {"
            [Interface]
            # Name = network.office.com
            Address = 10.0.0.1/24
            ListenPort = 51820
            PrivateKey = {server_private_key}
            DNS = 1.1.1.1,1.0.0.1
            
            PostUp = iptables -A FORWARD -i %i -j ACCEPT
            PostUp = iptables -A FORWARD -o %i -j ACCEPT
            PostUp = iptables -t nat -A POSTROUTING -o ens0 -j MASQUERADE
            
            PostDown = iptables -D FORWARD -i %i -j ACCEPT
            PostDown = iptables -D FORWARD -o %i -j ACCEPT
            PostDown = iptables -t nat -D POSTROUTING -o ens0 -j MASQUERADE
            
            [Peer]
            AllowedIPs = 10.0.0.2/32
            PublicKey = {client_public_key}
            
        "}
    );

    assert_eq!(
        client_conf,
        formatdoc! {"
            [Interface]
            Address = 10.0.0.2/24
            PrivateKey = {client_private_key}
            DNS = 1.1.1.1,1.0.0.1
            
            [Peer]
            Endpoint = network.office.com
            AllowedIPs = 0.0.0.0/0
            PublicKey = {server_public_key}
            PersistentKeepalive = 25
            
        "}
    );
}
