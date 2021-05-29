use std::collections::HashMap;
use serde::ser::{SerializeSeq};
use serde::{Serialize, Serializer};

#[derive(Default)]
pub struct FiltersBuilder {

    label: HashMap<String, Option<String>>

}

impl FiltersBuilder {

    pub fn new() -> Self {
        FiltersBuilder::default()
    }

    pub fn label<T>(&mut self, key: T, value: Option<String>) -> &mut Self
        where T: Into<String>
    {
        self.label.insert(key.into(), value);
        
        self
    }
    
    pub fn build(&self) -> Filters {
        Filters {
            label: self.label.clone()
        }
    }

}

#[derive(Serialize, Default, Clone, Debug)]
pub struct Filters {

    #[serde(serialize_with = "serialize_label")]
    label: HashMap<String, Option<String>>

}

impl Filters {

    pub fn new() -> FiltersBuilder {
        FiltersBuilder::default()
    }

    pub fn label(&self) -> HashMap<String, Option<String>> {
        self.label.clone()
    }

}

fn serialize_label<S>(label: &HashMap<String, Option<String>>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    let mut label_seq = s.serialize_seq(Some(label.len())).unwrap();
    for (key, value) in label {
        match value {
            Some(v) => {
                label_seq.serialize_element(format!("{}={}", key, v).as_str()).unwrap();
            },
            None => { label_seq.serialize_element(key.as_str()).unwrap(); }
        }
    }
    label_seq.end()
}