//! Module for AmneziaWG 2.0
// TODO: переписать.

use derive_builder::Builder;
use rand::prelude::*;
use std::{convert::Infallible, fmt};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[allow(unused_imports)]
use crate::Interface;
use crate::{WireguardError, WireguardResult};

use super::AmneziaWG;

macro_rules! assert_return {
    ($test:expr, $err:expr) => {
        if !($test) {
            return Err($err);
        }
    };
}

/// Inclusive range of H values used by AmneziaWG 2.0.
///
/// # Note
///
/// With `serde` an `HRange` is serialized as a string in **every** format, not
/// only in the human-readable ones: it is never a pair of numbers.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HRange {
    min: u32,
    max: u32,
}

impl HRange {
    /// Create new [`HRange`].
    #[must_use]
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

impl fmt::Display for HRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

/// **AmneziaWG 2.0** obfuscation values.
///
/// If you need to generate, build, validate or use these settings in interface, **use [`AmneziaWG`]
/// enum instead**.
///
/// ```
/// # use wireguard_conf::prelude::*;
/// let settings = AmneziaWG::random_v2();
/// let settings = AmneziaWG::builder_v2()
///     .jc(9)
///     // <snip>
///     .build()
///     .expect("invalid settings");
///
/// _ = Interface::builder()
///     .amnezia_settings(settings)
///     .build();
/// ```
///
/// - [Documentation](https://docs.amnezia.org/documentation/amnezia-wg)
#[must_use]
#[derive(Clone, Debug, PartialEq, Default, Builder)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[builder(build_fn(private, name = "fallible_build", error = "Infallible"))]
pub struct AmneziaWG2 {
    /// 0 ≤ Jc ≤ 10
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub jc: Option<u8>,

    /// 64 <= Jmin <= Jmax <= 1024
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub jmin: Option<u16>,

    /// 64 <= Jmin <= Jmax <= 1024
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub jmax: Option<u16>,

    /// Init packet length randomization.
    ///
    /// 0 <= S1 <= 64
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s1: Option<u8>,

    /// Response packet length randomization.
    ///
    /// 0 <= S2 <= 64
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s2: Option<u8>,

    /// Cookie packet length randomization.
    ///
    /// 0 <= S3 <= 64
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s3: Option<u8>,

    /// Data packet length randomization.
    ///
    /// 0 <= S4 <= 32
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s4: Option<u8>,

    /// Random header range for Init packets; H1-H4 ranges must not overlap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub h1: Option<HRange>,

    /// Random header range for Response packets; H1-H4 ranges must not overlap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub h2: Option<HRange>,

    /// Random header range for Cookie packets; H1-H4 ranges must not overlap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub h3: Option<HRange>,

    /// Random header range for Data packets; H1-H4 ranges must not overlap
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub h4: Option<HRange>,

    /// Custom Protocol Signature template for primary packet.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i1: Option<String>,

    /// Custom Protocol Signature template.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i2: Option<String>,

    /// Custom Protocol Signature template.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i3: Option<String>,

    /// Custom Protocol Signature template.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i4: Option<String>,

    /// Custom Protocol Signature template.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i5: Option<String>,
}

/// Methods
impl AmneziaWG2 {
    pub(super) fn random() -> AmneziaWG {
        let mut rng = rand::rng();

        let mut min = rng.random_range(0..=3_000_000);
        let width = rng.random_range(30_000..=65_535);
        let h1 = HRange::new(min, min + width);

        min += width + rng.random_range(1..=1000);
        let width = rng.random_range(30_000..=65_535);
        let h2 = HRange::new(min, min + width);

        min += width + rng.random_range(1..=1000);
        let width = rng.random_range(30_000..=65_535);
        let h3 = HRange::new(min, min + width);

        min += width + rng.random_range(1..=1000);
        let width = rng.random_range(30_000..=65_535);
        let h4 = HRange::new(min, min + width);
        AmneziaWG2::builder()
            .jc(rng.random_range(4..=10))
            .jmin(rng.random_range(64..256))
            .jmax(rng.random_range(768..=1024))
            .s1(rng.random_range(15..=64))
            .s2(rng.random_range(15..=64))
            .s3(rng.random_range(15..=64))
            .s4(rng.random_range(8..=32))
            .h1(h1)
            .h2(h2)
            .h3(h3)
            .h4(h4)
            .i1("<r 2><b 0x8580000100010000000004796162730679616e6465780272750000010001c00c000100010000026d000457fa27d1>")
            .build()
            .unwrap_or_else(|_| unreachable!())
    }

    /// Validates [`AmneziaWG2`].
    ///
    /// **Alternative and recommended way to validate is [`AmneziaWG::validate()`].**
    ///
    /// # Errors
    ///
    /// If [`AmneziaWG2`] is invalid, it will throw [`WireguardError::InvalidAmneziaSetting`]
    /// with setting name
    pub fn validate(&self) -> WireguardResult<()> {
        macro_rules! validate_range {
            ($field:ident in $range:expr, $name:literal) => {
                if let Some(value) = self.$field {
                    assert_return!(
                        $range.contains(&value),
                        WireguardError::InvalidAmneziaSetting($name)
                    );
                }
            };
        }
        macro_rules! validate {
            ($field:ident, $name:literal, $expr:expr) => {
                if let Some($field) = &self.$field {
                    assert_return!($expr, WireguardError::InvalidAmneziaSetting($name));
                }
            };
        }

        validate_range!(jc in 0..=10, "Jc");
        validate_range!(jmin in 64..=1024, "Jmin");
        validate_range!(jmax in 64..=1024, "Jmax");
        if let (Some(jmin), Some(jmax)) = (self.jmin, self.jmax) {
            assert_return!(
                jmin < jmax,
                WireguardError::InvalidAmneziaSetting("Jmin >= Jmax")
            );
        }
        validate_range!(s1 in 0..=64, "S1");
        validate_range!(s2 in 0..=64, "S2");
        validate_range!(s3 in 0..=64, "S3");
        validate_range!(s4 in 0..=32, "S4");
        validate!(h1, "H1", h1.min <= h1.max);
        validate!(h2, "H2", h2.min <= h2.max);
        validate!(h3, "H3", h3.min <= h3.max);
        validate!(h4, "H4", h4.min <= h4.max);

        let mut ranges: Vec<_> = [&self.h1, &self.h2, &self.h3, &self.h4]
            .into_iter()
            .flatten()
            .collect();
        ranges.sort();
        assert_return!(
            ranges.windows(2).all(|p| p[0].max < p[1].min),
            WireguardError::InvalidAmneziaSetting("H ranges overlap")
        );

        Ok(())
    }

    /// Removes AmneziaWG settings that are not required for the server.
    ///
    /// This is intended for generating server configuration, where
    /// some fields designed to confuse DPI may not be included (e.g., I1-I5).
    pub fn strip_client_data(&mut self) {
        macro_rules! set_none {
            ($($field:ident),+) => {
                $(self.$field = None;)+
            };
        }
        set_none!(jc, jmin, jmax, i1, i2, i3, i4, i5);
    }

    /// Create new [`AmneziaWG2Builder`].
    ///
    /// ```rust
    /// # use wireguard_conf::prelude::*;
    /// let amnezia_settings = AmneziaWG2::builder() // same as AmneziaWG2Builder::new()
    ///     .jc(5)
    ///     .jmin(65)
    ///     // <snip>
    ///     .build();
    /// # assert!(amnezia_settings.is_ok())
    /// ```
    #[must_use]
    pub fn builder() -> AmneziaWG2Builder {
        AmneziaWG2Builder::default()
    }
}

impl AmneziaWG2Builder {
    /// Create new builder for `AmneziaWG2`.
    ///
    /// ```rust
    /// # use wireguard_conf::prelude::*;
    /// let amnezia_settings = AmneziaWG2::builder() // same as AmneziaWG2Builder::new()
    ///     .jc(5)
    ///     .jmin(65)
    ///     // <snip>
    ///     .build();
    /// # assert!(amnezia_settings.is_ok())
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Finishes builder and builds [`AmneziaWG`] 2.0
    ///
    /// # Note
    ///
    /// This will return general enum [`AmneziaWG`], NOT inner struct [`AmneziaWG2`]
    ///
    /// # Errors
    ///
    /// If [`AmneziaWG2`] is invalid, it will throw [`WireguardError::InvalidAmneziaSetting`]
    /// with setting name
    pub fn build(&self) -> WireguardResult<AmneziaWG> {
        let settings = self.fallible_build().unwrap_or_else(|_| unreachable!());
        settings.validate()?;
        Ok(AmneziaWG::V2(settings))
    }
}

/// Implements [`fmt::Display`] for exporting AmneziaWG values.
///
/// # Note
///
/// It exports only Amnezia's obfuscation values (`Jc = ...`, `Jmax = ...`, etc.).
///
/// To export full interface, use `Interface::to_string()`.
impl fmt::Display for AmneziaWG2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! write_option {
            ($name:literal, $value:expr) => {
                if let Some(value) = &$value {
                    writeln!(f, "{} = {}", $name, value)?;
                }
            };
        }

        write_option!("Jc", self.jc);
        write_option!("Jmin", self.jmin);
        write_option!("Jmax", self.jmax);

        write_option!("S1", self.s1);
        write_option!("S2", self.s2);
        write_option!("S3", self.s3);
        write_option!("S4", self.s4);

        write_option!("H1", self.h1);
        write_option!("H2", self.h2);
        write_option!("H3", self.h3);
        write_option!("H4", self.h4);

        write_option!("I1", self.i1);
        write_option!("I2", self.i2);
        write_option!("I3", self.i3);
        write_option!("I4", self.i4);
        write_option!("I5", self.i5);

        Ok(())
    }
}

#[cfg(feature = "serde")]
impl Serialize for HRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for HRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let (min, max) = s
            .split_once('-')
            .ok_or_else(|| serde::de::Error::custom("expected \"min-max\""))?;

        Ok(Self {
            min: min.parse().map_err(serde::de::Error::custom)?,
            max: max.parse().map_err(serde::de::Error::custom)?,
        })
    }
}
