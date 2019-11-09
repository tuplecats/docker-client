use crate::container::{ToRequest};
use crate::http::{Request, URI};

#[derive(Debug, Default)]
pub struct RemoverBuilder {
    id: String,
    v: Option<bool>,
    force: Option<bool>,
    link: Option<bool>
}

#[derive(Debug)]
pub struct Remover {
    id: String,
    v: Option<bool>,
    force: Option<bool>,
    link: Option<bool>
}


impl Remover {
    pub fn new() -> RemoverBuilder {
        RemoverBuilder::default()
    }
}

impl RemoverBuilder {
    pub fn id<T>(&mut self, id: T) -> &mut Self
        where T: Into<String>
    {
        self.id = id.into();

        self
    }

    pub fn with_remove_volumes(&mut self, v: bool) -> &mut Self {
        self.v = Some(v);

        self
    }

    pub fn with_force_delete(&mut self, v: bool) -> &mut Self {
        self.force = Some(v);

        self
    }

    pub fn with_remove_link(&mut self, v: bool) -> &mut Self {
        self.link = Some(v);

        self
    }

    pub fn build(&self) -> Remover {
        Remover {
            id: self.id.clone(),
            v: self.v,
            force: self.force,
            link: self.link
        }
    }
}

impl ToRequest for Remover {
    fn to_request(&self) -> Request {

        let url = format!("/containers/{}", self.id);

        let mut uri = URI::with_path(url);

        if self.v.is_some() {
            uri.parameter("v".to_string(), self.v.unwrap().to_string());
        }
        if self.force.is_some() {
            uri.parameter("force".to_string(), self.force.unwrap().to_string());
        }
        if self.link.is_some() {
            uri.parameter("link".to_string(), self.link.unwrap().to_string());
        }

        Request::delete()
            .url(uri.build())
            .build()

    }
}