use crate::container::{ToRequest};
use crate::http::{Request, URI};

#[derive(Debug, Default)]
pub struct KillerBuilder {
    id: String,
    signal: Option<String>,
}

#[derive(Debug)]
pub struct Killer {
    id: String,
    signal: Option<String>,
}

impl Killer {
    pub fn new() -> KillerBuilder {
        KillerBuilder::default()
    }
}

impl KillerBuilder {
    pub fn id<T>(&mut self, id: T) -> &mut KillerBuilder
        where T: Into<String>
    {
        self.id = id.into();

        self

    }

    pub fn signal<T>(&mut self, signal: T) -> &mut KillerBuilder
        where T: Into<Option<String>>
    {
        self.signal = signal.into();

        self
    }

    pub fn build(&self) -> Killer {
        Killer {
            id: self.id.clone(),
            signal: self.signal.clone()
        }
    }
}

impl ToRequest for Killer {
    fn to_request(&self) -> Request {

        let url = format!("/containers/{}/kill", self.id);

        let mut uri = URI::with_path(url);

        if self.signal.is_some() {
            let signal = self.signal.clone().unwrap();
            uri.parameter("signal".to_string(), signal);
        }

        Request::post()
            .url(uri.build())
            .build()
    }
}