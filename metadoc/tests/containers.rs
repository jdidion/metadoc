#![cfg(test)]

use metadoc::{Described, Descriptor, Entry, Kind, StaticStrings};

/// non trivial metadata structs
#[derive(Described)]
#[allow(unused)]
struct OptionVec {
    /// Name used
    label: Option<String>,

    #[metadata(active = true)]
    score: Option<u64>,

    #[metadata(active = false)]
    attached: Vec<u64>,
}

#[test]
fn option_vec() {
    assert_eq!(
        OptionVec::metadata(),
        Descriptor {
            docs: StaticStrings::from(vec!["non trivial metadata structs"]),
            metadata: Default::default(),
            kind: Kind::Struct {
                name: "OptionVec".into(),
                children: vec![
                    Entry {
                        label: "label".into(),
                        docs: StaticStrings::from(vec!["Name used"]),
                        metadata: Default::default(),
                        has_default: false,
                        type_info: Descriptor {
                            docs: StaticStrings::NONE,
                            metadata: Default::default(),
                            kind: Kind::Option(Box::new(String::metadata()))
                        },
                        aliases: StaticStrings::from(&["label"])
                    },
                    Entry {
                        label: "score".into(),
                        docs: StaticStrings::NONE,
                        metadata: [("active", "true")].into_iter().collect(),
                        has_default: false,
                        type_info: Descriptor {
                            docs: StaticStrings::NONE,
                            metadata: Default::default(),
                            kind: Kind::Option(Box::new(u64::metadata()))
                        },
                        aliases: StaticStrings::from(&["score"])
                    },
                    Entry {
                        label: "attached".into(),
                        docs: StaticStrings::NONE,
                        metadata: [("active", "false")].into_iter().collect(),
                        has_default: false,
                        type_info: Descriptor {
                            docs: StaticStrings::NONE,
                            metadata: Default::default(),
                            kind: Kind::Sequence(Box::new(u64::metadata()))
                        },
                        aliases: StaticStrings::from(&["attached"])
                    },
                ]
            }
        }
    );
}
