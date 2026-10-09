//! Module for AmneziaWG 3.1

use super::{AmneziaWG, HeaderRange, UintRange};
use crate::{HeaderProtectionKey, WireguardError, WireguardResult};
use derive_builder::Builder;
use rand::prelude::*;
use std::{convert::Infallible, fmt};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! assert_return {
    ($test:expr, $err:expr) => {
        if !($test) {
            return Err($err);
        }
    };
}

/// **AmneziaWG 3.1** obfuscation values.
///
/// If you need to generate, build, validate or use these settings in interface, **use [`AmneziaWG`]
/// enum instead**.
///
/// ```
/// # use wireguard_conf::prelude::*;
/// let settings = AmneziaWG::random_v3();
/// let settings = AmneziaWG::builder_v3()
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
pub struct AmneziaWG3 {
    /// Recomended: 4 ≤ Jc ≤ 12
    /// General recommendation is to use it on the client side only.
    // WARN: Модуль ядра примет от 0 до 65535,
    // go-версия судя по коду до 2^32-1,
    // но Android-приложение упадет на значениях выше 126.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub jc: Option<u8>,

    /// 0 <= Jmin <= Jmax
    /// General recommendation is to use it on the client side only.
    // WARN: Модуль ядра примет от 0 до 65535,
    // go-версия судя по коду до 2^32-1,
    // Android-приложение принимает больше чем 429496729 (ему очень плохо, но принимает).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub jmin: Option<u16>,

    /// 0 <= Jmin <= Jmax
    /// Recomended: Jmax <= system MTU
    /// General recommendation is to use it on the client side only.
    // WARN: Модуль ядра примет от 0 до 65535,
    // go-версия судя по коду до 2^32-1,
    // Android-приложение принимает больше чем 429496729 (ему очень плохо, но принимает).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub jmax: Option<u16>,

    /// Init packet length randomization.
    ///
    /// 0 <= S1 <= 64 (if HeaderProtectionKey is not specified)
    /// 12 <= S1 <= 64 (if HeaderProtectionKey is specified)
    /// WARN: Скорее всего верхняя граница не изменилась с AWG2.0,
    /// но по актуальной документации не совсем понятно,
    /// у какого конкретно поля (S1-S4) верхняя граница — 32.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s1: Option<u8>,

    /// Response packet length randomization.
    ///
    /// 0 <= S2 <= 64 (if HeaderProtectionKey is not specified)
    /// 12 <= S2 <= 64 (if HeaderProtectionKey is specified)
    /// WARN: Скорее всего верхняя граница не изменилась с AWG2.0,
    /// но по актуальной документации не совсем понятно,
    /// у какого конкретно поля (S1-S4) верхняя граница — 32.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s2: Option<u8>,

    /// Cookie packet length randomization.
    ///
    /// 0 <= S3 <= 64 (if HeaderProtectionKey is not specified)
    /// 12 <= S3 <= 64 (if HeaderProtectionKey is specified)
    /// WARN: Скорее всего верхняя граница не изменилась с AWG2.0,
    /// но по актуальной документации не совсем понятно,
    /// у какого конкретно поля (S1-S4) верхняя граница — 32.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s3: Option<u8>,

    /// Transport packet length randomization.
    ///
    /// 0 <= S4 <= 32 (if HeaderProtectionKey is not specified)
    /// 12 <= S4 <= 32 (if HeaderProtectionKey is specified)
    /// WARN: Скорее всего верхняя граница не изменилась с AWG2.0,
    /// но по актуальной документации не совсем понятно,
    /// у какого конкретно поля (S1-S4) верхняя граница — 32.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(strip_option), default)]
    pub s4: Option<u8>,

    /// Random header range for Init packets; H1-H4 ranges must not overlap.
    /// If HeaderProtectionKey is specified, it is recommended to use the value 1.
    /// In this mode, the custom header mechanism is disabled (equivalent to the absence of the field).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub h1: Option<HeaderRange>,

    /// Random header range for Response packets; H1-H4 ranges must not overlap.
    /// If HeaderProtectionKey is specified, it is recommended to use the value 2.
    /// In this mode, the custom header mechanism is disabled (equivalent to the absence of the field).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub h2: Option<HeaderRange>,

    /// Random header range for Cookie packets; H1-H4 ranges must not overlap.
    /// If HeaderProtectionKey is specified, it is recommended to use the value 3.
    /// In this mode, the custom header mechanism is disabled (equivalent to the absence of the field).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub h3: Option<HeaderRange>,

    /// Random header range for Transport packets; H1-H4 ranges must not overlap.
    /// If HeaderProtectionKey is specified, it is recommended to use the value 4.
    /// In this mode, the custom header mechanism is disabled (equivalent to the absence of the field).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub h4: Option<HeaderRange>,

    /// Custom Protocol Signature template for primary packet.
    /// General recommendation is to use it on the client side only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i1: Option<String>,

    /// Custom Protocol Signature template.
    /// General recommendation is to use it on the client side only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i2: Option<String>,

    /// Custom Protocol Signature template.
    /// General recommendation is to use it on the client side only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i3: Option<String>,

    /// Custom Protocol Signature template.
    /// General recommendation is to use it on the client side only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i4: Option<String>,

    /// Custom Protocol Signature template.
    /// General recommendation is to use it on the client side only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub i5: Option<String>,

    /// Key for Header Protection.
    /// If specified, S1-S4 must be at least 12.
    /// It is also recommended to leave H1-H4 at the standard
    /// compatibility values: H1 = 1, H2 = 2, H3 = 3, H4 = 4.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub header_protection_key: Option<HeaderProtectionKey>,

    /// Random addition to the transport payload.
    /// Adds a random number of bytes from a specified range
    /// to transport packets. The padding uses
    /// the available space up to the internal MTU and does not exceed it.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub content_padding_addition: Option<UintRange>,

    /// Interval before re-handshake.
    // PERF: Поле указано как `client-side`, но рекомендации по использованиию
    // только на клиентской стороне в документации нет.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub rekey_after_time: Option<UintRange>,

    /// Handshake timeout.
    // PERF: Поле указано как `client-side`, но рекомендации по использованиию
    // только на клиентской стороне в документации нет.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub rekey_timeout: Option<UintRange>,

    /// Interval after which the connection initiates a new handshake if no data is received.
    // PERF: Поле указано как `client-side`, но рекомендации по использованиию
    // только на клиентской стороне в документации нет.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub reject_after_time: Option<UintRange>,

    /// Interval before sending keepalive.
    // PERF: Поле указано как `client-side`, но рекомендации по использованиию
    // только на клиентской стороне в документации нет.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub keepalive_timeout: Option<UintRange>,

    /// Maximum number of handshake retries.
    // PERF: Поле указано как `client-side`, но рекомендации по использованиию
    // только на клиентской стороне в документации нет.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), try_setter, default)]
    pub max_handshake_attempts: Option<UintRange>,

    /// Adds random trailers to packets.
    /// When using RandomTrailers, it is recommended to set the same values
    /// for S1, S2, S3, and S4. This reduces the risk of incorrect packet type detection.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub random_trailers: Option<bool>,

    /// Disables sending Cookie Reply.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(setter(into, strip_option), default)]
    pub disable_cookies: Option<bool>,
}

/// Methods
impl AmneziaWG3 {
    pub(super) fn random() -> AmneziaWG {
        const DNS_PS: &str = "<r 2><b 0x8580000100010000000004796162730679616e6465780272750000010001c00c000100010000026d000457fa27d1>";
        let mut rng = rand::rng();

        AmneziaWG3::builder()
            .jc(rng.random_range(4..=12))
            .jmin(rng.random_range(64..256))
            .jmax(rng.random_range(768..=1024))
            .s1(rng.random_range(15..=64))
            .s2(rng.random_range(15..=64))
            .s3(rng.random_range(15..=64))
            .s4(rng.random_range(12..=32))
            .h1(1)
            .h2(2)
            .h3(3)
            .h4(4)
            .i1(DNS_PS)
            .header_protection_key(HeaderProtectionKey::random())
            .build()
            .unwrap_or_else(|_| unreachable!())
    }

    /// Validates [`AmneziaWG3`].
    ///
    /// **Alternative and recommended way to validate is [`AmneziaWG::validate()`].**
    ///
    /// # Errors
    ///
    /// If [`AmneziaWG3`] is invalid, it will throw [`WireguardError::InvalidAmneziaSetting`]
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

        validate_range!(jc in 0..=126, "Jc");
        validate_range!(jmin in 64..=1024, "Jmin"); // FIX: старые границы.
        validate_range!(jmax in 64..=1024, "Jmax"); // FIX: старые границы.
        if let (Some(jmin), Some(jmax)) = (self.jmin, self.jmax) {
            assert_return!(
                jmin < jmax,
                WireguardError::InvalidAmneziaSetting("Jmin >= Jmax")
            );
        }

        let s_min: u8 = if self.header_protection_key.is_some() {
            12
        } else {
            0
        };
        validate_range!(s1 in s_min..=64, "S1");
        validate_range!(s2 in s_min..=64, "S2");
        validate_range!(s3 in s_min..=64, "S3");
        validate_range!(s4 in s_min..=32, "S4");

        // FIX: Возможно стоит запретить задавать H1-H4 при наличии HPK.
        // FIX: Возможно стоит проверять что S1=S2=S3=S4 при RandomTrailers = on.
        let mut ranges: Vec<_> = [&self.h1, &self.h2, &self.h3, &self.h4]
            .into_iter()
            .flatten()
            .collect();
        ranges.sort();
        assert_return!(
            ranges.windows(2).all(|p| p[0].hi() < p[1].lo()),
            WireguardError::InvalidAmneziaSetting("H ranges overlap")
        );

        Ok(())
    }

    /// Removes AmneziaWG settings that are not required for the server.
    ///
    /// This is intended for generating server configuration, where
    /// some fields designed to confuse DPI may not be included (e.g., I1-I5).
    ///
    /// Returns a new instance with client-only settings removed.
    ///
    /// **Note**
    ///
    /// The AmneziaWG 3.1 timers (`RekeyAfterTime`, `RekeyTimeout`, `RejectAfterTime`,
    /// `KeepaliveTimeout` and `MaxHandshakeAttempts`) are removed as well, because they are
    /// marked as client-side. Whether they really have to be stripped from the server's
    /// config is not decided yet, so this may change.
    pub fn strip_client_data(&mut self) {
        macro_rules! set_none {
            ($($field:ident),+) => {
                $(self.$field = None;)+
            };
        }
        set_none!(
            jc,
            jmin,
            jmax,
            i1,
            i2,
            i3,
            i4,
            i5,
            // Решение по таймерам отложено: они помечены как client-side,
            // но нужно ли вычищать их на сервере, пока не решено.
            rekey_after_time,
            rekey_timeout,
            reject_after_time,
            keepalive_timeout,
            max_handshake_attempts
        );
    }

    /// Create new [`AmneziaWG3Builder`].
    ///
    /// ```rust
    /// # use wireguard_conf::prelude::*;
    /// let amnezia_settings = AmneziaWG3::builder() // same as AmneziaWG3Builder::new()
    ///     .jc(5)
    ///     .jmin(65)
    ///     // <snip>
    ///     .build();
    /// # assert!(amnezia_settings.is_ok())
    /// ```
    #[must_use]
    pub fn builder() -> AmneziaWG3Builder {
        AmneziaWG3Builder::default()
    }
}

impl AmneziaWG3Builder {
    /// Create new builder for `AmneziaWG3`.
    ///
    /// ```rust
    /// # use wireguard_conf::prelude::*;
    /// let amnezia_settings = AmneziaWG3::builder() // same as AmneziaWG3Builder::new()
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

    /// Finishes builder and builds [`AmneziaWG`] 3.1
    ///
    /// # Note
    ///
    /// This will return general enum [`AmneziaWG`], NOT inner struct [`AmneziaWG3`]
    ///
    /// # Errors
    ///
    /// If [`AmneziaWG3`] is invalid, it will throw [`WireguardError::InvalidAmneziaSetting`]
    /// with setting name
    pub fn build(&self) -> WireguardResult<AmneziaWG> {
        let settings = self.fallible_build().unwrap_or_else(|_| unreachable!());
        settings.validate()?;
        Ok(AmneziaWG::V3(settings))
    }
}

/// Implements [`fmt::Display`] for exporting AmneziaWG values.
///
/// # Note
///
/// It exports only Amnezia's obfuscation values (`Jc = ...`, `Jmax = ...`, etc.).
///
/// To export full interface, use `Interface::to_string()`.
impl fmt::Display for AmneziaWG3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! write_option {
            ($name:literal, $value:ident) => {
                if let Some(value) = &self.$value {
                    writeln!(f, "{} = {}", $name, value)?;
                }
            };
        }

        write_option!("Jc", jc);
        write_option!("Jmin", jmin);
        write_option!("Jmax", jmax);

        write_option!("S1", s1);
        write_option!("S2", s2);
        write_option!("S3", s3);
        write_option!("S4", s4);

        write_option!("H1", h1);
        write_option!("H2", h2);
        write_option!("H3", h3);
        write_option!("H4", h4);

        write_option!("I1", i1);
        write_option!("I2", i2);
        write_option!("I3", i3);
        write_option!("I4", i4);
        write_option!("I5", i5);

        write_option!("HeaderProtectionKey", header_protection_key);

        write_option!("ContentPaddingAddition", content_padding_addition);
        write_option!("RekeyAfterTime", rekey_after_time);
        write_option!("RekeyTimeout", rekey_timeout);
        write_option!("RejectAfterTime", reject_after_time);
        write_option!("KeepaliveTimeout", keepalive_timeout);
        write_option!("MaxHandshakeAttempts", max_handshake_attempts);

        if let Some(flag) = self.random_trailers {
            if flag {
                writeln!(f, "RandomTrailers = on")?;
            } else {
                writeln!(f, "RandomTrailers = off")?;
            }
        }

        if let Some(flag) = self.disable_cookies {
            if flag {
                writeln!(f, "DisableCookies = on")?;
            } else {
                writeln!(f, "DisableCookies = off")?;
            }
        }

        Ok(())
    }
}
