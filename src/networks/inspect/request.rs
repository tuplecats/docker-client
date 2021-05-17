
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

    pub fn with_name<T>(&mut self, id: T) -> Self
        where T: Into<String>
    {
        let mut builder = RequestBuilder::default();
        builder.id = id.into();
        builder
    }

    pub fn build(&self) -> Request {
        Request {
            id: self.id.clone(),
            verbose: self.verbose,
            scope: self.scope.clone()
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
        format!("/networks/{}", self.id)
    }

}

