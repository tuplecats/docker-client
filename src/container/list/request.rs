use std::collections::HashMap;
use super::Filters;
//request
#[derive(Default)]
pub struct RequestBuilder {

    all: Option<bool>,

    limit: Option<i32>,

    size: Option<bool>,

    filters: Filters
}

impl RequestBuilder {

    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn all(&mut self, v: bool) -> &mut Self {
        self.all = Some(v);

        self
    }

    pub fn limit(&mut self, v: i32) -> &mut Self {
        self.limit = Some(v);

        self
    }

    pub fn size(&mut self, v: bool) -> &mut Self {
        self.size = Some(v);

        self
    }

    pub fn filters(&mut self, f: Filters) -> &mut Self {
        self.filters = f;

        self
    }

    pub fn build(&self) -> Request {
        Request {
            all: self.all.clone(),
            limit: self.limit.clone(),
            size: self.size.clone(),
            filters: self.filters.clone()
        }
    }

}

#[derive(Debug, Clone)]
pub struct Request {

    all: Option<bool>,

    limit: Option<i32>,

    size: Option<bool>,

    filters: Filters,
}

impl Request {

    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    fn percent_encoded(value: String) -> String {
        let mut result = String::new();

        for char in value.chars() {
            match char {
                '"' => { result.push_str("%22"); },
                _ => { result.push(char); }
            };
        }

        result
    }

    pub fn get_path(&self) -> String {
        let mut path = "/containers/json?".to_string();

        if self.all.is_some() {
            path.push_str(format!("all={}&", self.all.unwrap()).as_str());
        }
        if self.limit.is_some() {
            path.push_str(format!("limit={}&", self.limit.unwrap()).as_str());
        }
        if self.size.is_some() {
            path.push_str(format!("size={}&", self.size.unwrap()).as_str());
        }

        if !self.filters.label().is_empty() {
            path.push_str(
                format!(
                    "filters={}&",
                    Request::percent_encoded(
                        serde_json::to_string(&self.filters.clone()).unwrap()
                    )
                ).as_str()
            );
        }

        path.pop();
        path
    }

}