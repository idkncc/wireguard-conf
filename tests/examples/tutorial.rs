use indoc::formatdoc;
use wireguard_conf::{as_ipnet, prelude::*};

#[test]
fn all_fields() {
    // You can import/export key using `PrivateKey::try_from()` and `PrivateKey.to_string()`
    // or using `serde::Serialize` and `serde::Deserialize` traits (requires `serde` feature)
    //
    // for the sake of example, we'll generate new key:
    let server_private_key = PrivateKey::random();

    // *same comment as about server's private key.*
    let peer1_private_key = PrivateKey::random();

    // Server's output interface.
    let output_interface = "ens0";

    let mut server_interface = InterfaceBuilder::new()
        // also you can use:
        //   .add_address(as_ipnet!("10.0.0.1/24"))
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

    let peer1 = PeerBuilder::new()
        .allowed_ips([as_ipnet!("10.0.0.2/24")])
        // you can pass either private key or public key.
        //  - if you pass public key, you can _only add `Peer` into `Interface`.
        //  - if you pass private key, you can also generate `Interface` from `Peer` and
        //    server `Interface` (using `.to_interface()`)!
        .private_key(peer1_private_key.clone())
        .build();
    server_interface.peers.push(peer1.clone());

    // cool shortcut: `.to_interface()`:
    //  it constructs peer's interface from Peer and server's interface.
    //  technically, `Peer` is just a config section, mean while `Interface` is a full config.
    let mut peer1_interface = peer1
        .to_interface(&server_interface)
        .expect("failed to get interface from peer1");

    // modify peer's full config to add PersistentKeepalive
    peer1_interface.peers[0].persistent_keepalive = 25;

    let server_conf = server_interface.to_string();
    let peer1_conf = peer1_interface.to_string();

    // tests part
    let server_public_key = PublicKey::from(&server_private_key);
    let peer1_public_key = PublicKey::from(&peer1_private_key);
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
            AllowedIPs = 10.0.0.2/24
            PublicKey = {peer1_public_key}
            
        "}
    );

    assert_eq!(
        peer1_conf,
        formatdoc! {"
            [Interface]
            Address = 10.0.0.2/24
            PrivateKey = {peer1_private_key}
            DNS = 1.1.1.1,1.0.0.1
            
            [Peer]
            Endpoint = network.office.com
            AllowedIPs = 10.0.0.1/24
            PublicKey = {server_public_key}
            PersistentKeepalive = 25
            
        "}
    );
}
