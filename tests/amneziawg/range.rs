use wireguard_conf::prelude::*;

macro_rules! width_tests {
    ($name:ident, $range:ty) => {
        mod $name {
            use super::*;

            fn parse(value: &str) -> $range {
                value.parse().expect("valid range")
            }

            #[test]
            fn parse_range() {
                let range = parse("1-4");

                assert_eq!((range.lo(), range.hi()), (1, 4));
                assert_eq!(range.to_string(), "1-4");
            }

            #[test]
            fn parse_single_value() {
                let range = parse("1");

                assert_eq!((range.lo(), range.hi()), (1, 1));
                assert_eq!(range.to_string(), "1");
            }

            #[test]
            fn parse_inverted_range_is_rejected() {
                assert_eq!(
                    "5-1".parse::<$range>(),
                    Err(WireguardError::InvalidAmneziaSetting(
                        "wrong UintRange specified"
                    ))
                );
            }

            #[test]
            fn parse_malformed_range_is_rejected() {
                for value in ["", " ", "a", "-", "--1", "-1", "1-", "1-2-3", "1 - 4"] {
                    assert!(
                        value.parse::<$range>().is_err(),
                        "{value:?} must not be parsed"
                    );
                }
            }

            #[test]
            fn default_is_a_zero_range() {
                let range = <$range>::default();

                assert!(range.is_zero());
                assert_eq!(range, parse("0"));
            }

            #[test]
            fn from_number_is_a_single_value() {
                let range = <$range>::from(7);

                assert_eq!(range, parse("7"));
            }

            #[test]
            fn try_from_pair_of_bounds() {
                let range = <$range>::try_from((2, 4)).expect("valid range");
                let single = <$range>::try_from((3, 3)).expect("valid range");

                assert_eq!(range, parse("2-4"));
                assert_eq!(single, parse("3"));
            }

            #[test]
            fn try_from_inverted_pair_of_bounds_is_rejected() {
                assert_eq!(
                    <$range>::try_from((5, 1)),
                    Err(WireguardError::InvalidAmneziaSetting(
                        "wrong UintRange specified"
                    ))
                );
            }

            #[test]
            fn try_from_str() {
                assert_eq!(
                    <$range>::try_from("1-4").expect("valid range"),
                    parse("1-4")
                );
                assert!(<$range>::try_from("5-1").is_err());
            }

            #[test]
            fn comparison_is_by_bounds() {
                assert!(parse("1-4") < parse("1-5"));
                assert!(parse("1-9") < parse("2-3"));
            }

            #[test]
            #[cfg(feature = "serde")]
            fn serde() {
                let range = parse("1-4");
                let json = serde_json::to_string(&range).expect("range is serializable");
                assert_eq!(json, r#""1-4""#);
                assert_eq!(
                    serde_json::from_str::<$range>(&json).expect("range is deserializable"),
                    range
                );

                let single = parse("4");
                let json = serde_json::to_string(&single).expect("range is serializable");
                assert_eq!(json, r#""4""#);
                assert_eq!(
                    serde_json::from_str::<$range>(&json).expect("range is deserializable"),
                    single
                );

                assert_eq!(
                    serde_json::from_str::<$range>("4").expect("number is deserializable"),
                    parse("4")
                );

                for value in [r#""abc""#, "true", "[1, 4]", "-5"] {
                    assert!(
                        serde_json::from_str::<$range>(value).is_err(),
                        "{value:?} must not be deserialized"
                    );
                }
            }
        }
    };
}

width_tests!(uint_range, UintRange);
width_tests!(header_range, HeaderRange);
