mod range;
mod ver1;
mod ver2;

use std::fmt;

pub use range::{HeaderRange, UintRange};
pub use ver1::*;
pub use ver2::*;

#[allow(unused_imports)]
use crate::WireguardError;
use crate::WireguardResult;

/// [AmneziaWG](https://docs.amnezia.org/documentation/amnezia-wg) obfuscation values. By using this
/// enum, you can parse, generate and validate for all AmneziaWG versions.
///
/// ```
/// # use wireguard_conf::prelude::*;
/// let settings = AmneziaWG::random_v1();
/// let settings = AmneziaWG::random_v2();
///
/// assert_eq!(settings.validate(), Ok(()));
///
/// // With `serde` feature you can serialize and deserialize obfuscation values.
///
/// println!("{settings}");
/// ```
///
/// **Note**
///
/// A variant matches even when it recognized nothing, because all of its settings are
/// `Option` and unknown fields are ignored. So a typo in a setting name is silently
/// dropped instead of failing, and an object with no known settings at all is read as
/// an empty config rather than as an error.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum AmneziaWG {

    /// AmneziaWG 2.0
    V2(AmneziaWG2),

    /// AmneziaWG 1.0
    V1(AmneziaWG1),
}

impl AmneziaWG {
    /// Generate random recommended settings for **AmneziaWG 1.0**.
    ///
    /// # Examples
    ///
    /// ```
    /// use wireguard_conf::prelude::*;
    ///
    /// let settings = AmneziaWG::random_v1();
    ///
    /// _ = InterfaceBuilder::new()
    ///    // <snip>
    ///    .amnezia_settings(settings)
    ///    .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn random_v1() -> Self {
        Self::V1(AmneziaWG1::random())
    }

    /// Generate random recommended settings for **AmneziaWG 2.0**.
    ///
    /// # Examples
    ///
    /// ```
    /// use wireguard_conf::prelude::*;
    ///
    /// let settings = AmneziaWG::random_v2();
    ///
    /// _ = InterfaceBuilder::new()
    ///    // <snip>
    ///    .amnezia_settings(settings)
    ///    .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn random_v2() -> Self {
        AmneziaWG2::random()
    }

    /// Alias for `AmneziaWG::builder_v2().build()`
    #[inline]
    #[must_use]
    pub fn empty_v2() -> Self {
        Self::V2(AmneziaWG2::default())
    }

    /// Create new [`AmneziaWG2Builder`].
    ///
    /// This function is an alias for [`AmneziaWG2::builder()`].
    #[inline]
    #[must_use]
    pub fn builder_v2() -> AmneziaWG2Builder {
        AmneziaWG2::builder()
    }

    /// Validates [`AmneziaWG`] values.
    ///
    /// ```
    /// # use wireguard_conf::prelude::*;
    /// let settings = AmneziaWG::random_v2();
    ///
    /// assert_eq!(settings.validate(), Ok(()));
    /// ```
    ///
    /// # Errors
    ///
    /// If [`AmneziaWG`] is invalid, it will throw [`WireguardError::InvalidAmneziaSetting`]
    /// with setting name
    pub fn validate(&self) -> WireguardResult<()> {
        match self {
            AmneziaWG::V1(settings) => settings.validate(),
            AmneziaWG::V2(settings) => settings.validate(),
        }
    }

    /// Removes AmneziaWG settings that are not required for the server.
    ///
    /// This is intended for generating server configuration, where
    /// some fields designed to confuse DPI may not be included (e.g., I1-I5).
    pub fn strip_client_data(&mut self) {
        match self {
            AmneziaWG::V1(_) => {}
            AmneziaWG::V2(settings) => settings.strip_client_data(),
        }
    }
}

/// Implementation of [`fmt::Display`] for exporting [`AmneziaWG`] values.
///
/// # Note
///
/// It exports only `Jc`, `Jmin`, etc.
///
/// To export full interface, use `Interface::to_string()`.
impl fmt::Display for AmneziaWG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AmneziaWG::V1(settings) => settings.fmt(f),
            AmneziaWG::V2(settings) => settings.fmt(f),
        }
    }
}
