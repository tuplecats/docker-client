//!
//! Kill container types.
//!
//! The module provides [KillerBuilder](struct.KillerBuilder.html) and [Killer](struct.Killer.html) types
//! used to create a support structure to kill a container.
//!
//! # CreatorBuilder
//! The [KillerBuilder](struct.KillerBuilder.html) provides a set of methods to create a structure [Killer](struct.Killer.html).
//!
//! # Creator
//! The [Killer](struct.Killer.html) is a helper structure for sending a request to kill a container.
//!
//! # API Documentaion
//!
//! API documentaion available at [link](https://docs.docker.com/engine/api/v1.40/#operation/ContainerKill)
//!
//! # Examples
//!
//! Kill container example.
//! ```rust
//! use docker_client::DockerClient;
//! use docker_client::Killer;
//!
//! fn main() {
//!     let client = match DockerClient::connect("/var/run/docker.sock") {
//!         Ok(client) => client,
//!         Err(e) => panic!("Cannot connect to socket!"),
//!     };
//!
//!     let killer = Killer::new()
//!         .id("example-kill")
//!         .build();
//!
//!     match client.kill_container(killer) {
//!         Ok(_) => {},
//!         Err(_) => {},
//!     }
//! }
//! ```


use crate::container::{ToRequest};
use crate::http::{Request, URI};

/// A Killer builder.
///
/// This type can be used to construct an instance of `Killer` through a builder-like pattern.
#[derive(Debug, Default)]
pub struct KillerBuilder {
    id: String,
    signal: Option<String>,
}

/// Represents a Killer.
#[derive(Debug)]
pub struct Killer {
    id: String,
    signal: Option<String>,
}

impl Killer {
    /// Creates a new default instance of `KillerBuilder` to construct a `Killer`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use docker_client::Killer;
    ///
    /// let killer = Killer::new()
    ///     .id("example-id")
    ///     .signal("some-signal")
    ///     .build();
    ///
    /// ```
    pub fn new() -> KillerBuilder {
        KillerBuilder::default()
    }
}

impl KillerBuilder {

    /// Set `id` of the `KillerBuilder`.
    pub fn id<T>(&mut self, id: T) -> &mut KillerBuilder
        where T: Into<String>
    {
        self.id = id.into();

        self
    }

    /// Set `signal` of the `KillerBuilder`.
    pub fn signal<T>(&mut self, signal: T) -> &mut KillerBuilder
        where T: Into<String>
    {
        self.signal = Some(signal.into());

        self
    }

    /// Build `Killer` from `KillerBuilder`
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