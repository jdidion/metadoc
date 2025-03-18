use metadoc::{Described, Descriptor, Entry, Kind, StaticStrings, Variant};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Described, Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[serde(rename = "RenameFieldTestType")]
struct RenameField {
    #[serde(rename = "type")]
    field_type: String,
}

#[test]
fn rename_field() {
    assert_eq!(
        serde_json::to_string(&RenameField {
            field_type: "abc".to_owned()
        })
        .unwrap(),
        "{\"type\":\"abc\"}"
    );
    assert_eq!(
        serde_json::from_str::<RenameField>("{\"type\": \"abc\"}").unwrap(),
        RenameField {
            field_type: "abc".to_owned()
        }
    );
    assert_eq!(
        RenameField::metadata(),
        Descriptor {
            docs: StaticStrings::NONE,
            metadata: Default::default(),
            kind: Kind::Struct {
                name: "RenameFieldTestType".into(),
                children: vec![Entry {
                    label: "type".into(),
                    docs: StaticStrings::NONE,
                    has_default: false,
                    metadata: Default::default(),
                    type_info: String::metadata(),
                    aliases: StaticStrings::from(&["type"])
                }]
            }
        }
    );
}

#[derive(Serialize, Deserialize, Described, Debug)]
#[allow(dead_code)]
enum RenameVarient {
    #[serde(rename = "type", alias = "kind")]
    Type,
}

#[test]
fn rename_variant() {
    assert_eq!(
        RenameVarient::metadata(),
        Descriptor {
            docs: StaticStrings::NONE,
            metadata: Default::default(),
            kind: metadoc::Kind::Enum {
                name: "RenameVarient".into(),
                variants: vec![Variant {
                    label: "type".into(),
                    docs: StaticStrings::NONE,
                    metadata: Default::default(),
                    aliases: StaticStrings::from(&["type", "kind"])
                },]
            }
        }
    )
}

#[derive(Serialize, Deserialize, Described, Debug)]
#[allow(dead_code)]
#[serde(rename = "OuterName", rename_all = "UPPERCASE")]
enum RenameAllVarient {
    Type,
}

#[test]
fn rename_all_varients() {
    assert_eq!(
        RenameAllVarient::metadata(),
        Descriptor {
            docs: StaticStrings::NONE,
            metadata: Default::default(),
            kind: metadoc::Kind::Enum {
                name: "OuterName".into(),
                variants: vec![Variant {
                    label: "TYPE".into(),
                    docs: StaticStrings::NONE,
                    metadata: Default::default(),
                    aliases: StaticStrings::from(&["TYPE"])
                },]
            }
        }
    )
}

#[derive(Serialize, Deserialize, Described, Debug)]
#[allow(dead_code)]
struct RenameAllField {
    inner: u8,
}

#[test]
fn rename_all_fields() {
    assert_eq!(
        RenameAllField::metadata(),
        Descriptor {
            docs: StaticStrings::NONE,
            metadata: Default::default(),
            kind: Kind::Struct {
                name: "RenameAllField".into(),
                children: vec![Entry {
                    label: "inner".into(),
                    docs: StaticStrings::NONE,
                    has_default: false,
                    metadata: Default::default(),
                    type_info: u8::metadata(),
                    aliases: StaticStrings::from(&["inner"])
                }]
            }
        }
    );
}

// #[test]
// fn rename_struct() {
//     panic!();
// }

#[derive(Serialize, Deserialize, Described)]
struct InlineInner {
    a: u32,
}

#[derive(Serialize, Deserialize, Described)]
#[allow(dead_code)]
struct InlineOuter {
    #[serde(flatten)]
    inner: InlineInner,
}

#[test]
fn inline() {
    let Kind::Struct { name, children } = InlineInner::metadata().kind else {
        panic!()
    };
    assert_eq!(name, "InlineInner");
    let inner_children = children;
    let Kind::Struct { name, children } = InlineOuter::metadata().kind else {
        panic!()
    };
    assert_eq!(name, "InlineOuter");
    assert_eq!(inner_children, children);
}
