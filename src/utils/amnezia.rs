use rand::prelude::*;
use std::{collections::HashSet, fmt};

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
use serde::{Deserialize, Serialize};

use super::{WireguardError, WireguardResult};

macro_rules! assert_return {
    ($test:expr, $err:expr) => {
        if !($test) {
            return Err($err);
        }
    };
}

/// AmneziaWG obfuscation values.
///
/// - [Documentation](https://github.com/amnezia-vpn/amneziawg-linux-kernel-module?tab=readme-ov-file#configuration)
#[must_use]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AmneziaSettings {
    /// 1 ≤ Jc ≤ 128; recommended range is from 3 to 10 inclusive
    pub jc: usize,

    /// Jmin < Jmax; recommended value is 50
    pub jmin: usize,
    /// Jmin < Jmax ≤ 1280; recommended value is 1000
    pub jmax: usize,
    /// S1 < 1280; S1 + 56 ≠ S2; recommended range is from 15 to 150 inclusive
    pub s1: usize,
    /// S2 < 1280; recommended range is from 15 to 150 inclusive
    pub s2: usize,

    /// must be unique among each other; recommended range is from 5 to 2147483647 inclusive
    pub h1: usize,
    /// must be unique among each other; recommended range is from 5 to 2147483647 inclusive
    pub h2: usize,
    /// must be unique among each other; recommended range is from 5 to 2147483647 inclusive
    pub h3: usize,
    /// must be unique among each other; recommended range is from 5 to 2147483647 inclusive
    pub h4: usize,
}

/// Methods
impl AmneziaSettings {
    /// Generate [`AmneziaSettings`] with randomized values, based of recommended ranges or values.
    ///
    /// # Examples
    ///
    /// ```
    /// use wireguard_conf::prelude::*;
    ///
    /// let settings = AmneziaSettings::random();
    ///
    /// _ = InterfaceBuilder::new()
    ///    // <snip>
    ///    .amnezia_settings(settings)
    ///    .build();
    /// ```
    pub fn random() -> Self {
        let mut rng = rand::rng();

        let jc = rng.random_range(4..=12);
        let jmin = 8;
        let jmax = 80;
        let s1 = rng.random_range(15..=150);
        let s2 = {
            let mut value = s1 + 56;

            while s1 + 56 == value {
                value = rng.random_range(1..=150);
            }

            value
        };

        let h1 = rng.random_range(10..=2_147_483_640);
        let h2 = rng.random_range(10..=2_147_483_640);
        let h3 = rng.random_range(10..=2_147_483_640);
        let h4 = rng.random_range(10..=2_147_483_640);

        Self {
            jc,
            jmin,
            jmax,
            s1,
            s2,
            h1,
            h2,
            h3,
            h4,
        }
    }

    /// Validates [`AmneziaSettings`].
    ///
    /// # Errors
    ///
    /// If [`AmneziaSettings`] is invalid, it will throw [`WireguardError::InvalidAmneziaSetting`]
    /// with setting name
    pub fn validate(&self) -> WireguardResult<()> {
        assert_return!(
            1 <= self.jc && self.jc <= 128,
            WireguardError::InvalidAmneziaSetting("Jc".to_string())
        );

        assert_return!(
            self.jmin < self.jmax,
            WireguardError::InvalidAmneziaSetting("Jmin".to_string())
        );
        assert_return!(
            self.jmax <= 1280,
            WireguardError::InvalidAmneziaSetting("Jmax".to_string())
        );
        assert_return!(
            self.s1 < 1280 && self.s1 + 56 != self.s2,
            WireguardError::InvalidAmneziaSetting("S1".to_string())
        );
        assert_return!(
            self.s2 < 1280,
            WireguardError::InvalidAmneziaSetting("S2".to_string())
        );

        let are_h_values_unique = {
            let set = HashSet::from([self.h1, self.h2, self.h3, self.h4]);

            set.len() == 4
        };
        assert_return!(
            are_h_values_unique,
            WireguardError::InvalidAmneziaSetting("H1/H2/H3/H4".to_string())
        );

        Ok(())
    }
}

/// Implements [`fmt::Display`] for exporting AmneziaWG values.
///
/// # Note
///
/// It exports only [`Jc = ..., Jmin = ..., etc`]. To export full interface, use `Interface.to_string()`.
impl fmt::Display for AmneziaSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Jc = {}", self.jc)?;
        writeln!(f, "Jmin = {}", self.jmin)?;
        writeln!(f, "Jmax = {}", self.jmax)?;
        writeln!(f, "S1 = {}", self.s1)?;
        writeln!(f, "S2 = {}", self.s2)?;
        writeln!(f, "H1 = {}", self.h1)?;
        writeln!(f, "H2 = {}", self.h2)?;
        writeln!(f, "H3 = {}", self.h3)?;
        writeln!(f, "H4 = {}", self.h4)?;

        Ok(())
    }
}
