

#[cfg(test)]
mod test {
    use struct_metadata::{Described, Kind};
    use struct_metadata_derive::Described;

    #[derive(Described)]
    struct EmptyA;

    #[derive(Described)]
    struct EmptyB {}

    #[test]
    fn empty_a() {
        let data = EmptyA::metadata();
        assert_eq!(data.name, "EmptyA");
        assert_eq!(data.kind, Kind::Struct{ children: vec![]});
        assert_eq!(data.docs, None);
    }

    #[test]
    fn empty_b() {
        let data = EmptyB::metadata();
        assert_eq!(data.name, "EmptyB");
        assert_eq!(data.kind, Kind::Struct{ children: vec![]});
        assert_eq!(data.docs, None);
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
        assert_eq!(data.name, "EmptyDocA");
        assert_eq!(data.kind, Kind::Struct{ children: vec![]});
        assert_eq!(data.docs, Some(vec!["Docstring"]));
    }

    #[test]
    fn empty_doc_b() {
        let data = EmptyDocB::metadata();
        assert_eq!(data.name, "EmptyDocB");
        assert_eq!(data.kind, Kind::Struct{ children: vec![]});
        assert_eq!(data.docs, Some(vec!["The", "", "Docstring"]));
    }


    #[derive(Described)]
    struct Single(u64);

    #[test]
    fn single() {
        let data = Single::metadata();
        assert_eq!(data.name, "Single");
        assert_eq!(data.kind, Kind::UnsignedInteger);
        assert_eq!(data.docs, None);
    }

}