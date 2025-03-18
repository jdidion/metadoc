#![cfg(test)]

use metadoc::{Described, Entry, Kind, StaticStrings};

#[derive(Described)]
struct EmptyA;

#[derive(Described)]
struct EmptyB {}

#[test]
fn empty_a() {
    let data = EmptyA::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "EmptyA".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert!(data.metadata.is_empty());
}

#[test]
fn empty_b() {
    let data = EmptyB::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "EmptyB".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert!(data.metadata.is_empty());
}

#[derive(Described)]
/// Docstring
struct EmptyDocA;

#[derive(Described)]
/// The
///
/// Docstring
struct EmptyDocB {}

#[test]
fn empty_doc_a() {
    let data = EmptyDocA::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "EmptyDocA".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::from(vec!["Docstring"]));
    assert!(data.metadata.is_empty());
}

#[test]
fn empty_doc_b() {
    let data = EmptyDocB::metadata();
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "EmptyDocB".into(),
            children: vec![]
        }
    );
    assert_eq!(data.docs, StaticStrings::from(vec!["The", "", "Docstring"]));
    assert!(data.metadata.is_empty());
}

#[derive(Described)]
#[allow(dead_code)]
struct Single(u64);

#[test]
fn single() {
    let data = Single::metadata();
    assert_eq!(
        data.kind,
        Kind::Aliased {
            name: "Single".into(),
            kind: Box::new(u64::metadata())
        }
    );
    assert_eq!(data.docs, StaticStrings::NONE);
    assert!(data.metadata.is_empty());
}

#[derive(Described)]
#[metadata(important: true)]
struct SingleFeatured;

#[derive(Described)]
#[metadata(important: true, cats: "Less than 10")]
struct DoubleFeatured;

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
    assert_eq!(data.metadata, [("important", "true")].into_iter().collect());
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
        [("important", "true"), ("cats", "\"Less than 10\"")]
            .into_iter()
            .collect()
    );
}

#[derive(Described)]
#[metadata(important: true)]
#[allow(dead_code)]
struct SimpleFields {
    /// Name used
    label: u64,

    #[metadata(text: true)]
    description: String,

    /// Are cats allowed here?
    #[metadata(important: true)]
    cats: bool,
}

#[test]
fn simple_fields() {
    let data = SimpleFields::metadata();
    assert_eq!(data.docs, StaticStrings::NONE);
    assert_eq!(data.metadata, [("important", "true")].into_iter().collect());
    assert_eq!(
        data.kind,
        Kind::Struct {
            name: "SimpleFields".into(),
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
                    label: "description".into(),
                    docs: StaticStrings::NONE,
                    has_default: false,
                    metadata: [("text", "true")].into_iter().collect(),
                    type_info: String::metadata(),
                    aliases: StaticStrings::from(&["description"])
                },
                Entry {
                    label: "cats".into(),
                    docs: StaticStrings::from(vec!["Are cats allowed here?"]),
                    has_default: false,
                    metadata: [("important", "true")].into_iter().collect(),
                    type_info: bool::metadata(),
                    aliases: StaticStrings::from(&["cats"])
                },
            ]
        }
    );
}
