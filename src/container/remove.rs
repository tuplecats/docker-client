//!
//! Remove container types.
//!
//! The module provides [RemoverBuilder](struct.RemoverBuilder.html) and [Remover](struct.Remover.html) types
//! used to create a support structure to remove a container.
//!
//! # RemoverBuilder
//! The [RemoverBuilder](struct.RemoverBuilder.html) provides a set of methods to create a structure [Remover](struct.Remover.html).
//!
//! # Remover
//! The [Remover](struct.Remover.html) is a helper structure for sending a request to remove a container.
//!
//! # API Documentaion
//!
//! API documentaion available at [link](https://docs.docker.com/engine/api/v1.40/#operation/ContainerDelete)
//!
//! # Examples
//!
//! Remove container example.
//! ```rust
//! use docker_client::DockerClient;
//! use docker_client::Remover;
//!
//! fn main() {
//!     let client = match DockerClient::connect("/var/run/docker.sock") {
//!         Ok(client) => client,
//!         Err(e) => panic!("Cannot connect to socket!"),
//!     };
//!
//!     let remover = Remover::new()
//!         .id("example-remove")
//!         .build();
//!
//!     match client.remove_container(remover) {
//!         Ok(_) => {},
//!         Err(_) => {},
//!     }
//! }
//! ```


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

    /// Creates a new default instance of `RemoverBuilder` to construct a `Remover`.
    pub fn new() -> Self {
        RemoverBuilder::default()
    }

    /// Set `id` of the `RemoverBuilder`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::RemoverBuilder;
    /// let builder = RemoverBuilder::new()
    ///     .id("container-id-or-name")
    ///     .build();
    /// ```
    pub fn id<T>(&mut self, id: T) -> &mut Self
        where T: Into<String>
    {
        self.id = id.into();

        self
    }

    /// Set flag `v` of the `RemoverBuilder`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::RemoverBuilder;
    /// let builder = RemoverBuilder::new()
    ///     .with_remove_volumes(true)
    ///     .build();
    /// ```
    pub fn with_remove_volumes(&mut self, v: bool) -> &mut Self {
        self.v = Some(v);

        self
    }

    /// Set flag `force` of the `RemoverBuilder`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::RemoverBuilder;
    /// let builder = RemoverBuilder::new()
    ///     .with_force_delete(true)
    ///     .build();
    /// ```
    pub fn with_force_delete(&mut self, v: bool) -> &mut Self {
        self.force = Some(v);

        self
    }

    /// Set flag `link` of the `RemoverBuilder`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::RemoverBuilder;
    /// let builder = RemoverBuilder::new().build();
    /// ```
    pub fn with_remove_link(&mut self, v: bool) -> &mut Self {
        self.link = Some(v);

        self
    }

    /// Build `Remover` from `RemoverBuilder`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::RemoverBuilder;
    /// let builder = RemoverBuilder::new()
    ///     .id("container-id-or-name")
    ///     .build();
    /// ```
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