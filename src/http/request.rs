use unix_socket::UnixStream;
use std::io::Write;
use std::collections::HashMap;
use crate::http::{Response, URI};

#[derive(Debug, Clone)]
pub enum HTTPMethod {
    POST,
    GET,
    DELETE
}

impl Default for HTTPMethod {
    fn default() -> HTTPMethod {
        HTTPMethod::GET
    }
}

#[derive(Debug)]
pub struct Request {
    method: HTTPMethod,
    url: URI,
    headers: HashMap<String, String>,
    content: String
}

#[derive(Debug, Default)]
pub struct RequestBuilder {
    method: HTTPMethod,
    url: URI,
    headers: HashMap<String, String>,
    content: String
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    fn with_method(method: HTTPMethod) -> Self {
        let mut builder = RequestBuilder::default();
        builder.method = method;
        builder
    }

    pub fn method(&mut self, method: HTTPMethod) -> &mut Self {
        self.method = method;

        self
    }

    pub fn url(&mut self, url: URI) -> &mut Self {
        self.url = url;

        self
    }

    pub fn header<T, P>(&mut self, key: T, value: P) -> &mut Self
        where
            T: Into<String>,
            P: Into<String>
    {
        let key = key.into();
        let value = value.into();

        self.headers.insert(key, value);

        self
    }

    pub fn content<T>(&mut self, content: T) -> &mut Self
        where T: Into<String>
    {
        self.content = content.into();

        self
    }

    pub fn build(&self) -> Request {
        Request {
            method: self.method.clone(),
            url: self.url.clone(),
            headers: self.headers.clone(),
            content: self.content.clone()
        }
    }

}

impl Request {
    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub fn get() -> RequestBuilder {
        RequestBuilder::with_method(HTTPMethod::GET)
    }

    pub fn post() -> RequestBuilder {
        RequestBuilder::with_method(HTTPMethod::POST)
    }

    pub fn delete() -> RequestBuilder {
        RequestBuilder::with_method(HTTPMethod::DELETE)
    }

    pub fn send(&self, mut stream: UnixStream) -> Response {
        let request = self.to_string();

        match stream.write_all(request.as_bytes()) {
            Ok(_) => {
                Response::read(&mut stream)
            }
            _ => panic!(""),
        }
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {

        let method = match self.method {
            HTTPMethod::GET => "GET",
            HTTPMethod::POST => "POST",
            HTTPMethod::DELETE => "DELETE"
        };


        let mut headers = String::new();
        for (k, v) in &self.headers {
            headers.push_str(format!("\r\n{}: {}", k, v).as_str());
        }
        headers.push_str("\r\nHost: 127.0.0.1");

        match self.method {
            HTTPMethod::POST => {
                if self.content.len() > 0 {
                    headers.push_str(format!("\r\nContent-Length: {}", self.content.len()).as_str());
                    headers.push_str("\r\nContent-Type: application/json");
                }
            }
            _ => {}
        }

        format!("{} {} HTTP/1.1{}\r\n\r\n{}", method, self.url.to_string(), headers, self.content)
    }
}