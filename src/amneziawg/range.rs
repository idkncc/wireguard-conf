use crate::WireguardError;
use std::{fmt, num::ParseIntError, str::FromStr};

/// Unsigned integer type, that can be stored inside [`UintRangeOf`].
///
/// Implemented for `u16` and `u32`, because `amneziawg-tools` reads some of the
/// AmneziaWG settings as 16-bit values and the rest as 32-bit ones.
///
/// This trait is an implementation detail of the range types and is not re-exported:
/// use [`UintRange`] or [`HeaderRange`] instead.
pub trait Num: Copy + Ord + fmt::Display + FromStr<Err = ParseIntError> + Default {
    const ZERO: Self;
}

impl Num for u16 {
    const ZERO: Self = 0;
}

impl Num for u32 {
    const ZERO: Self = 0;
}

/// Range of `u16` values, used by the AmneziaWG settings, that are limited to 16 bits.
///
/// These are `ContentPaddingAddition` and the AmneziaWG 3.1 timers
/// (`RekeyAfterTime`, `RekeyTimeout`, `RejectAfterTime`, `KeepaliveTimeout`,
/// `MaxHandshakeAttempts`).
pub type UintRange = UintRangeOf<u16>;

/// Range of `u32` values, used by the AmneziaWG 3.1 random header ranges `H1`-`H4`.
pub type HeaderRange = UintRangeOf<u32>;

/// Inclusive range of unsigned integer values used by AmneziaWG.
///
/// Both bounds are inclusive, so `1-4` contains `1`, `2`, `3` and `4`.
/// A range with equal bounds is exported as a single value: `4-4` is displayed as `4`.
///
/// A range can be created by parsing a string (`"1"` or `"1-4"`) or from a single
/// value, so the lower bound is never greater than the upper one:
///
/// ```
/// # use wireguard_conf::prelude::*;
/// let single: UintRange = "1".parse().expect("valid range");
/// let range: UintRange = "1-4".parse().expect("valid range");
///
/// assert_eq!((single.lo(), single.hi()), (1, 1));
/// assert_eq!((range.lo(), range.hi()), (1, 4));
/// ```
///
/// Bounds, parsed from a string, need a fallible conversion, so they can only be
/// set with the `try_`-prefixed setters. A plain number is a range with equal bounds,
/// and goes through the infallible setters:
///
/// ```
/// # use wireguard_conf::prelude::*;
/// # fn main() -> WireguardResult<()> {
/// let settings = AmneziaWG::builder_v3()
///     .h1(7) // still a number
///     .try_h2("1-2")? // now a range
///     // <snip>
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// A range can also be built from a pair of bounds with [`TryFrom`]:
///
/// ```
/// # use wireguard_conf::prelude::*;
/// # fn main() -> WireguardResult<()> {
/// let range = HeaderRange::try_from((2, 4))?;
///
/// assert_eq!((range.lo(), range.hi()), (2, 4));
/// # Ok(())
/// # }
/// ```
///
/// The default value is a zero range (`0-0`), which `amneziawg-tools`
/// treats the same way as an unset setting.
///
/// [`UintRange`] is used by the settings limited to 16 bits,
/// [`HeaderRange`] — by the ones limited to 32 bits.
///
/// # Note
///
/// With `serde` a single-value range (`4-4`) is **serialized** as a plain number,
/// while a multi-value one is a string (`"1-4"`) in **every** format, not only in
/// the human-readable ones: it is never a pair of numbers.
///
/// **Deserialization** accepts both a string (`"1-4"`) and a plain unsigned number
/// (`4` as the range `4-4`), so it needs a self-describing format (JSON, TOML,
/// YAML, ...). Formats without self-description (f.e. `bincode`) can't be used to
/// deserialize a range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UintRangeOf<T: Num> {
    lo: T,
    hi: T,
}

impl<T: Num> UintRangeOf<T> {
    fn new(lo: T, hi: T) -> Result<Self, WireguardError> {
        if hi < lo {
            Err(WireguardError::InvalidAmneziaSetting(
                "wrong UintRange specified",
            ))
        } else {
            Ok(Self { lo, hi })
        }
    }

    const fn single(value: T) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    /// Returns the lower bound of the range (inclusive).
    pub const fn lo(&self) -> T {
        self.lo
    }

    /// Returns the upper bound of the range (inclusive).
    pub const fn hi(&self) -> T {
        self.hi
    }

    /// Returns `true` if the range is a zero range (`0-0`).
    ///
    /// `amneziawg-tools` treats it the same way as an unset setting:
    /// e.g. `PersistentKeepalive = 0-0` is not exported at all.
    pub fn is_zero(&self) -> bool {
        self.lo == T::ZERO && self.hi == T::ZERO
    }
}

impl<T: Num> Default for UintRangeOf<T> {
    fn default() -> Self {
        Self::single(T::ZERO)
    }
}

impl<T: Num> From<T> for UintRangeOf<T> {
    fn from(value: T) -> Self {
        Self::single(value)
    }
}

impl<T: Num> TryFrom<(T, T)> for UintRangeOf<T> {
    type Error = WireguardError;

    /// Builds a range from a pair of bounds, both inclusive.
    ///
    /// # Errors
    ///
    /// If `hi` is lower than `lo`, because a range always has `lo <= hi`.
    fn try_from((lo, hi): (T, T)) -> Result<Self, Self::Error> {
        Self::new(lo, hi)
    }
}

impl<T: Num> FromStr for UintRangeOf<T> {
    type Err = WireguardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const WRONG_FORMAT: WireguardError =
            WireguardError::InvalidAmneziaSetting("wrong UintRange format");
        let mut parts = s.split('-');

        let lo = parts
            .next()
            .and_then(|n| n.parse().ok())
            .ok_or(WRONG_FORMAT)?;

        let hi = match parts.next() {
            None => lo,
            Some(n) => n.parse().map_err(|_| WRONG_FORMAT)?,
        };

        if parts.next().is_some() {
            return Err(WRONG_FORMAT);
        }

        Self::new(lo, hi)
    }
}

impl<T: Num> TryFrom<&str> for UintRangeOf<T> {
    type Error = WireguardError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<T: Num> fmt::Display for UintRangeOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lo = self.lo;
        let hi = self.hi;

        if lo == hi {
            write!(f, "{lo}")
        } else {
            write!(f, "{lo}-{hi}")
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::{Num, UintRangeOf};
    use serde::de::{self, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;
    use std::marker::PhantomData;

    impl<T: Num + Serialize> Serialize for UintRangeOf<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if self.lo == self.hi {
                self.lo.serialize(serializer)
            } else {
                self.to_string().serialize(serializer)
            }
        }
    }

    impl<'de, T: Num> Deserialize<'de> for UintRangeOf<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(RangeVisitor(PhantomData))
        }
    }

    /// Accepts a range as a string (`"4"`, `"1-4"`) and, since `V3` accepts numeric
    /// `H1`-`H4`, a plain unsigned number (`4`) as the range `4-4`.
    struct RangeVisitor<T: Num>(PhantomData<T>);

    impl<T: Num> Visitor<'_> for RangeVisitor<T> {
        type Value = UintRangeOf<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an unsigned integer or a range like \"1-4\"")
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            let value: T = v.to_string().parse().map_err(de::Error::custom)?;
            Ok(UintRangeOf::from(value))
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
            let value =
                u64::try_from(v).map_err(|_| de::Error::custom("range must not be negative"))?;
            self.visit_u64(value)
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse().map_err(de::Error::custom)
        }
    }
}
