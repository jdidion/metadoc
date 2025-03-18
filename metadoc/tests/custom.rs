#![cfg(test)]

use pretty_assertions::assert_eq;

use metadoc::{Described, Descriptor, Entry, Kind, MetadataKind, StaticStrings};

#[derive(Default, PartialEq, Eq, Debug, MetadataKind, Clone)]
struct Properties {
    pub important: bool,
    pub cats: &'static str,
}

#[derive(Described)]
#[metadata_type(Properties)]
#[metadata(important: true)]
struct SingleFeatured;

#[derive(Described)]
#[metadata_type(Properties, defaults: false)]
#[metadata(important: true, cats: "Less than 10")]
struct DoubleFeatured;

#[derive(Described)]
#[metadata_type(Properties, defaults: false)]
struct NoneFeatured;

#[test]
fn single_featured() {
    let data = SingleFeatured::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "SingleFeatured".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert_eq!(
        data.metadata,
        Properties {
            important: true,
            cats: ""
        }
    );
}

#[test]
fn dual_featured() {
    let data = DoubleFeatured::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "DoubleFeatured".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert_eq!(
        data.metadata,
        Properties {
            important: true,
            cats: "Less than 10"
        }
    );
}

#[test]
fn none_featured() {
    let data = NoneFeatured::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "NoneFeatured".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert_eq!(
        data.metadata,
        Properties {
            ..Default::default()
        }
    );
}

// This should cause a compiler error when uncommented
// #[derive(Described)]
// #[metadata_type(Properties)]
// #[metadata(dogs: true)]
// struct DogFeatured;

#[derive(Described)]
#[metadata_sequence(Vec<(&'static str, &'static str)>)]
#[metadata(important: true)]
struct SingleVecFeatured;

#[derive(Described)]
#[metadata_sequence(Vec<(&'static str, &'static str)>)]
#[metadata(important: true, cats="Less than 10")]
struct DoubleVecFeatured;

#[test]
fn single_vec_featured() {
    let data = SingleVecFeatured::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "SingleVecFeatured".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert_eq!(data.metadata, vec![("important", "true")]);
}

#[test]
fn dual_vec_featured() {
    let data = DoubleVecFeatured::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "DoubleVecFeatured".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert_eq!(
        data.metadata,
        vec![("important", "true"), ("cats", "\"Less than 10\"")]
    );
}

/// non trivial metadata structs
#[derive(Described)]
#[allow(unused)]
#[metadata_type(Properties)]
#[metadata(important: true)]
struct Fields {
    /// Name used
    label: u64,

    #[metadata(cats: "fluffy")]
    description: String,

    /// Are cats allowed here?
    #[metadata(important: true)]
    cats: bool,
}

fn expected_fields_metadata() -> Descriptor<Properties> {
    Descriptor {
        docs: StaticStrings::from(vec!["non trivial metadata structs"]),
        metadata: Properties {
            important: true,
            cats: "",
        },
        kind: Kind::Struct {
            name: "Fields".into(),
            children: vec![
                Entry {
                    label: "label".into(),
                    docs: StaticStrings::from(vec!["Name used"]),
                    has_default: false,
                    metadata: Default::default(),
                    type_info: u64::metadata(),
                    aliases: StaticStrings::from(&["label"]),
                },
                Entry {
                    label: "description".into(),
                    docs: StaticStrings::NONE,
                    has_default: false,
                    metadata: Properties {
                        cats: "fluffy",
                        ..Default::default()
                    },
                    type_info: String::metadata(),
                    aliases: StaticStrings::from(&["description"]),
                },
                Entry {
                    label: "cats".into(),
                    docs: StaticStrings::from(vec!["Are cats allowed here?"]),
                    has_default: false,
                    metadata: Properties {
                        important: true,
                        cats: "",
                    },
                    type_info: bool::metadata(),
                    aliases: StaticStrings::from(&["cats"]),
                },
            ],
        },
    }
}

#[test]
fn fields() {
    assert_eq!(Fields::metadata(), expected_fields_metadata());
}

/// nested structs
#[derive(Described)]
#[allow(unused)]
#[metadata_type(Properties)]
#[metadata(important: true)]
struct Nested {
    /// Name used
    label: u64,

    #[metadata(cats: "with stripes")]
    data: Fields,
}

#[test]
fn nested() {
    assert_eq!(
        Nested::metadata(),
        Descriptor {
            docs: StaticStrings::from(vec!["nested structs"]),
            metadata: Properties {
                important: true,
                cats: ""
            },
            kind: Kind::Struct {
                name: "Nested".into(),
                children: vec![
                    Entry {
                        label: "label".into(),
                        docs: StaticStrings::from(vec!["Name used"]),
                        has_default: false,
                        metadata: Default::default(),
                        type_info: u64::metadata(),
                        aliases: StaticStrings::from(&["label"])
                    },
                    Entry {
                        label: "data".into(),
                        docs: StaticStrings::NONE,
                        has_default: false,
                        metadata: Properties {
                            cats: "with stripes",
                            ..Default::default()
                        },
                        type_info: expected_fields_metadata(),
                        aliases: StaticStrings::from(&["data"])
                    },
                ]
            }
        }
    );
}

// #[derive(Described)]
// #[allow(unused)]
// #[metadata_type(Properties)]
// #[metadata(important: true)]
// struct Newtype(u64);

// #[derive(Described)]
// #[allow(unused)]
// #[metadata_type(Properties)]
// struct UseNewtype {
//     data: Newtype,
//     odata: Option<Newtype>,
// }

// #[test]
// fn newtype() {

//     let newtype_kind = Kind::Aliased { name: "Newtype", kind: Box::new(u64::metadata()) };

//     let newtype = Descriptor {
//         docs: None,
//         metadata: Properties { important: true, cats: "" },
//         kind: newtype_kind.clone(),
//     };

//     let newtype_option = Descriptor {
//         docs: None,
//         metadata: Properties { important: true, cats: "" },
//         kind: Kind::Option(Box::new(newtype.clone())),
//     };

//     assert_eq!(UseNewtype::metadata(), Descriptor{
//         docs: None,
//         metadata: Default::default(),
//         kind: Kind::Struct {
//             name: "UseNewtype",
//             children: vec![
//                 Entry { label: "data", docs: None, has_default: false, metadata: Properties { important: true, ..Default::default() }, type_info: newtype, aliases: &["data"] },
//                 Entry { label: "odata", docs: None, has_default: false, metadata: Properties { important: true, ..Default::default() }, type_info: newtype_option, aliases: &["odata"] },
//             ]
//         }
//     });
// }
