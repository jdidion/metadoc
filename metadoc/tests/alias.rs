use metadoc::{Described, Kind, StaticStrings};

#[allow(dead_code)]
#[derive(serde::Deserialize, Described)]
struct HasAlias {
    single_name: u64,
    #[serde(alias = "other_name")]
    two_names: u64,
    #[serde(alias = "other_name1", alias = "other_name2")]
    three_names: u64,
}

#[test]
fn default_defined() {
    let data = HasAlias::metadata();
    let Kind::Struct { name, children } = data.kind else {
        panic!()
    };
    assert_eq!(name, "HasAlias");

    assert_eq!(children[0].label, "single_name");
    assert_eq!(children[0].aliases, StaticStrings::from(&["single_name"]));

    assert_eq!(children[1].label, "two_names");
    assert_eq!(
        children[1].aliases,
        StaticStrings::from(&["two_names", "other_name"])
    );

    assert_eq!(children[2].label, "three_names");
    assert_eq!(
        children[2].aliases,
        StaticStrings::from(&["three_names", "other_name1", "other_name2"])
    );
}
