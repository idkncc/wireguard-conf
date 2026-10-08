#[cfg(feature = "amneziawg-3")]
mod header;
mod preshared;
mod private;
mod public;

#[cfg(feature = "amneziawg-3")]
pub use header::HeaderProtectionKey;
pub use preshared::PresharedKey;
pub use private::PrivateKey;
pub use public::PublicKey;
