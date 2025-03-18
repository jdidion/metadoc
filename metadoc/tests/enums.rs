use metadoc::{Described, Descriptor, StaticStrings, Variant};
use metadoc_derive::DescribedEnumString;

#[derive(strum::Display, strum::EnumString, DescribedEnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ExtendedScanValues {
    Submitted,
    Skipped,
    Incomplete,
    #[metadata(ideal = true)]
    Complete,
}

#[test]
fn enum_display() {
    assert_eq!(
        ExtendedScanValues::metadata(),
        Descriptor {
            docs: StaticStrings::NONE,
            metadata: Default::default(),
            kind: metadoc::Kind::Enum {
                name: "ExtendedScanValues".into(),
                variants: vec![
                    Variant {
                        label: "submitted".into(),
                        docs: StaticStrings::NONE,
                        metadata: Default::default(),
                        aliases: StaticStrings::from(&["submitted"])
                    },
                    Variant {
                        label: "skipped".into(),
                        docs: StaticStrings::NONE,
                        metadata: Default::default(),
                        aliases: StaticStrings::from(&["skipped"])
                    },
                    Variant {
                        label: "incomplete".into(),
                        docs: StaticStrings::NONE,
                        metadata: Default::default(),
                        aliases: StaticStrings::from(&["incomplete"])
                    },
                    Variant {
                        label: "complete".into(),
                        docs: StaticStrings::NONE,
                        metadata: [("ideal", "true")].into_iter().collect(),
                        aliases: StaticStrings::from(&["complete"])
                    },
                ]
            }
        }
    )
}
