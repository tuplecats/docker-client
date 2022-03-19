
pub struct RequestBuilder {

    id: String,

    verbose: bool,

    scope: String

}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            id: String::new(),
            verbose: false,
            scope: String::new()
        }
    }
}

impl RequestBuilder {

    pub fn with_name<T>(id: T) -> Self
        where T: Into<String>
    {
        let mut builder = RequestBuilder::default();
        builder.id = id.into();
        builder
    }

    pub fn build(self) -> Request {
        Request {
            id: self.id,
            verbose: self.verbose,
            scope: self.scope
        }
    }

}

pub struct Request {

    id: String,

    verbose: bool,

    scope: String

}

impl Request {

    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub fn get_path(&self) -> String {
        let mut path = format!("/networks/{}?", self.id);

        if self.verbose {
            path.push_str("verbose=true&")
        }

        if !self.scope.is_empty() {
            path.push_str(format!("scope={}", self.scope.as_str()).as_str())
        }

        path
    }

}

