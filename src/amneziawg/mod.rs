mod range;
mod ver1;
mod ver2;
mod ver3;

use std::fmt;

pub use range::{HeaderRange, UintRange, UintRangeOf};
pub use ver1::*;
pub use ver2::*;
pub use ver3::*;

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
/// let settings = AmneziaWG::random_v3();
///
/// assert_eq!(settings.validate(), Ok(()));
///
/// // With `serde` feature you can serialize and deserialize obfuscation values.
///
/// println!("{settings}");
/// ```
///
/// # Deserialization
///
/// [`AmneziaWG`] is `#[serde(untagged)]`, so there is no version tag in the format
/// and the variant is picked by trying the variants in order, from the newest to the
/// oldest: `V3`, then `V2`, then `V1`, and an error if none of them matches. In other
/// words, a config is read as the newest AmneziaWG version that can parse it:
///
/// - a **AmneziaWG 3.1** config is read as `V3`;
/// - **AmneziaWG 2.0** and **1.0** configs are read as `V3` too, because every `V2` and
///   `V1` setting has the same name in `V3`, and `H1`-`H4` accept both numbers and
///   ranges. The exported config doesn't change, only the variant does. The `V1` and
///   `V2` variants are produced only when a value doesn't fit the `V3` types, or when
///   the settings are built in code;
/// - an empty object is read as `V3` with everything unset.
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
    /// AmneziaWG 3.1
    V3(AmneziaWG3),

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

    /// Generate random recommended settings for **AmneziaWG 3.1**.
    ///
    /// # Examples
    ///
    /// ```
    /// use wireguard_conf::prelude::*;
    ///
    /// let settings = AmneziaWG::random_v3();
    ///
    /// _ = InterfaceBuilder::new()
    ///    // <snip>
    ///    .amnezia_settings(settings)
    ///    .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn random_v3() -> Self {
        AmneziaWG3::random()
    }

    /// Alias for `AmneziaWG::builder_v3().build()`
    #[inline]
    #[must_use]
    pub fn empty_v3() -> Self {
        Self::V3(AmneziaWG3::default())
    }

    /// Create new [`AmneziaWG3Builder`].
    ///
    /// This function is an alias for [`AmneziaWG3::builder()`].
    #[inline]
    #[must_use]
    pub fn builder_v3() -> AmneziaWG3Builder {
        AmneziaWG3::builder()
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
            AmneziaWG::V3(settings) => settings.validate(),
        }
    }

    /// Removes AmneziaWG settings that are not required for the server.
    ///
    /// This is intended for generating server configuration, where
    /// some fields designed to confuse DPI may not be included (e.g., I1-I5).
    ///
    /// For the [`V1`](AmneziaWG::V1) variant it is a no-op: AmneziaWG 1.0 has no
    /// client-only settings.
    pub fn strip_client_data(&mut self) {
        match self {
            AmneziaWG::V1(_) => {}
            AmneziaWG::V2(settings) => settings.strip_client_data(),
            AmneziaWG::V3(settings) => settings.strip_client_data(),
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
            AmneziaWG::V3(settings) => settings.fmt(f),
        }
    }
}
