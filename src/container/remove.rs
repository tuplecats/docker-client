use crate::container::{ToRequest};
use crate::http::{Request, URI};

/// Remover builder struct.
#[derive(Debug, Default)]
pub struct RemoverBuilder {
    id: String,
    v: Option<bool>,
    force: Option<bool>,
    link: Option<bool>
}

/// Remover struct.
#[derive(Debug)]
pub struct Remover {
    id: String,
    v: Option<bool>,
    force: Option<bool>,
    link: Option<bool>
}


impl Remover {
    /// Creates a new default instance of `RemoverBuilder` to construct a `Remover`.
    pub fn new() -> RemoverBuilder {
        RemoverBuilder::default()
    }
}

impl RemoverBuilder {

    /// Set `id` of the `RemoverBuilder`.
    pub fn id<T>(&mut self, id: T) -> &mut Self
        where T: Into<String>
    {
        self.id = id.into();

        self
    }

    /// Set flag `v` of the `RemoverBuilder`.
    pub fn with_remove_volumes(&mut self, v: bool) -> &mut Self {
        self.v = Some(v);

        self
    }

    /// Set flag `force` of the `RemoverBuilder`.
    pub fn with_force_delete(&mut self, v: bool) -> &mut Self {
        self.force = Some(v);

        self
    }

    /// Set flag `link` of the `RemoverBuilder`.
    pub fn with_remove_link(&mut self, v: bool) -> &mut Self {
        self.link = Some(v);

        self
    }

    /// Build `Remover` from `RemoverBuilder`.
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