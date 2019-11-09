use std::collections::HashMap;
use std::borrow::Borrow;

#[derive(Debug, Default)]
pub struct URIBuilder {
    url: String,
    params: HashMap<String, String>,
}

#[derive(Debug, Default, Clone)]
pub struct URI {
    url: String,
    params: HashMap<String, String>,
}

impl URIBuilder {
    pub fn new() -> Self {
        URIBuilder::default()
    }

    pub fn url<T>(&mut self, url: T) -> &mut Self
        where T: Into<String> {
        self.url = url.into();

        self
    }

    pub fn parameter<T, P>(&mut self, key: T, value: P) -> &mut Self
        where
            T: Into<String>,
            P: Into<String>
    {

        self.params.insert(key.into(), value.into());

        self
    }

    pub fn build(&self) -> URI {
        URI {
            url: self.url.clone(),
            params: self.params.clone(),
        }
    }
}

impl URI {

    pub fn new() -> URIBuilder {
        URIBuilder::default()
    }

    pub fn with_path<T>(url: T) -> URIBuilder
        where T: Into<String> {

        let mut builder = URIBuilder::default();
        builder.url = url.into();
        builder
    }

}

impl ToString for URI {
    fn to_string(&self) -> String {
        let mut url = self.url.clone();

        let params = self.params.borrow();

        url.push('?');
        for (k, v) in params {
            url.push_str(format!("{}={}&", k, v).as_str());
        }

        url.pop();

        url
    }
}
