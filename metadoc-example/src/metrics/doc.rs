use metadoc::{Described, Descriptor, Kind};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fmt;
    use std::sync::LazyLock;

    pub static METRIC_META: LazyLock<Vec<Metadata>> =
        LazyLock::new(|| vec![Metadata::from(&super::ExampleMetric::metadata())]);

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Metadata {
        name: String,
        description: String,
        fields: Vec<Field>,
    }

    impl From<&Descriptor<HashMap<&'static str, &'static str>>> for Metadata {
        fn from(struct_meta: &Descriptor<HashMap<&'static str, &'static str>>) -> Self {
            let description = struct_meta.docs.to_string("\n");
            match &struct_meta.kind {
                Kind::Struct { name, children } => {
                    let fields = children
                        .iter()
                        .map(|field| {
                            let name = field.label.to_string();
                            let description = field.docs.to_string("\n");
                            let data_type = DataType::from(&field.type_info);
                            Field {
                                name,
                                description,
                                data_type,
                            }
                        })
                        .collect::<Vec<_>>();
                    Metadata {
                        name: name.to_string(),
                        description,
                        fields,
                    }
                }
                _ => panic!("Expected metric to be a struct"),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Field {
        name: String,
        description: String,
        data_type: DataType,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum DataType {
        Integer,
        Float,
        String,
        Boolean,
        DateTime,
        List {
            item_type: Box<DataType>,
            delim: String,
        },
        Enum {
            choices: Vec<String>,
        },
        Optional {
            inner_type: Box<DataType>,
        },
    }

    impl fmt::Display for DataType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Integer => write!(f, "integer"),
                Self::Float => write!(f, "float"),
                Self::String => write!(f, "string"),
                Self::Boolean => write!(f, "boolean"),
                Self::DateTime => write!(f, "datetime"),
                Self::List { item_type, delim } => {
                    write!(f, "List of {} separated by {}", item_type, delim)
                }
                Self::Enum { choices } => {
                    let choices = choices.join(", ");
                    write!(f, "One of: {}", choices)
                }
                Self::Optional { inner_type } => {
                    write!(f, "{} (optional)", inner_type)
                }
            }
        }
    }

    impl From<&Descriptor<HashMap<&'static str, &'static str>>> for DataType {
        fn from(value: &Descriptor<HashMap<&'static str, &'static str>>) -> Self {
            match &value.kind {
                Kind::U8
                | Kind::U16
                | Kind::U32
                | Kind::U64
                | Kind::U128
                | Kind::Usize
                | Kind::I8
                | Kind::I16
                | Kind::I32
                | Kind::I64
                | Kind::I128
                | Kind::Isize => Self::Integer,
                Kind::F32 | Kind::F64 => Self::Float,
                Kind::String => Self::String,
                Kind::Bool => Self::Boolean,
                Kind::DateTime => Self::DateTime,
                Kind::Sequence(inner) => {
                    let delim = if let Some(delim) = &value.metadata.get("delim") {
                        delim.to_string()
                    } else {
                        ",".into()
                    };
                    Self::List {
                        item_type: Box::new(Self::from(inner.as_ref())),
                        delim,
                    }
                }
                Kind::Enum { variants, .. } => {
                    let choices = variants
                        .iter()
                        .map(|variant| variant.label.to_string())
                        .collect::<Vec<_>>();
                    Self::Enum { choices }
                }
                Kind::Option(inner) => Self::Optional {
                    inner_type: Box::new(Self::from(inner.as_ref())),
                },
                _ => panic!("Unsupported data type"),
            }
        }
    }