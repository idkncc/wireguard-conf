use wireguard_conf::as_ipnet;
use wireguard_conf::prelude::*;

use either::Either;

#[test]
fn empty_peer() {
    let peer = PeerBuilder::new().build();

    let key = peer.key.clone(); // only (private) key is generated

    assert_eq!(
        peer,
        Peer {
            endpoint: None,
            allowed_ips: vec![],
            persistent_keepalive: 0,
            key
        }
    );
}

#[test]
fn endpoint() {
    let endpoint = "peer.example.com";

    let peer = PeerBuilder::new().endpoint(endpoint).build();

    assert_eq!(peer.endpoint, Some(endpoint.to_string()));
}

#[test]
fn allowed_ips() {
    let allowed_ips = [as_ipnet!("10.0.0.1/24"), as_ipnet!("0.0.0.0/0")];

    let peer = PeerBuilder::new().allowed_ips(&allowed_ips).build();

    assert_eq!(peer.allowed_ips, allowed_ips.to_vec());
}

#[test]
fn persistent_keepalive() {
    let persistent_keepalive = 25;

    let peer = PeerBuilder::new().persistent_keepalive(25).build();

    assert_eq!(peer.persistent_keepalive, persistent_keepalive);
}

#[test]
fn key() {
    let private_key = PrivateKey::random();
    let public_key = PublicKey::from(&private_key);

    let peer_private_key = PeerBuilder::new().private_key(private_key.clone()).build();
    let peer_public_key = PeerBuilder::new().public_key(public_key.clone()).build();

    assert_eq!(peer_private_key.key, Either::Left(private_key));
    assert_eq!(peer_public_key.key, Either::Right(public_key));
}
