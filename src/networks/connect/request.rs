use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

#[derive(Default)]
pub struct RequestBuilder {
    id: String,

    container: String
}

impl RequestBuilder {

    pub fn with_name<T>(id: T) -> Self
        where T: Into<String>
    {
        let mut builder = RequestBuilder::default();
        builder.id = id.into();
        builder
    }

    pub fn container<T>(&mut self, name: T) -> &mut Self
        where T: Into<String>
    {
        self.container = name.into();

        self
    }

    pub fn build(&self) -> Request {
        Request {
            id: self.id.clone(),
            container: self.container.clone()
        }
    }

}

pub struct Request {
    id: String,

    container: String
}

impl Request {

    pub fn get_path(&self) -> String {
        format!("/networks/{}/connect", self.id)
    }

}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut map = serializer.serialize_map(Some(1)).unwrap();
        map.serialize_entry("Container", self.container.as_str()).unwrap();
        map.end()
    }
}