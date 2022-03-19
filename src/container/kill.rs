//!
//! Kill container types.
//!
//! The module provides [KillerBuilder](struct.KillerBuilder.html) and [Killer](struct.Killer.html) types
//! used to create a support structure to kill a container.
//!
//! # KillerBuilder
//! The [KillerBuilder](struct.KillerBuilder.html) provides a set of methods to create a structure [Killer](struct.Killer.html).
//!
//! # Killer
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
//! #[tokio::main]
//! async fn main() {
//!     let client = DockerClient::new();
//!
//!     let killer = Killer::new()
//!         .id("example-kill")
//!         .signal("SIGNAL")
//!         .build();
//!
//!     match client.kill_container(killer).await {
//!         Ok(_) => {},
//!         Err(_) => {},
//!     }
//! }
//! ```


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

    /// Return path for request
    pub fn get_path(&self) -> String {
        let mut path = format!("/containers/{}/kill?", self.id);

        if self.signal.is_some() {
            path.push_str(format!("signal={}&", self.signal.clone().unwrap()).as_str());
        }

        path.pop();
        path
    }
}

impl KillerBuilder {

    /// Creates a new default instance of `KillerBuilder` to construct a `Killer`.
    pub fn new() -> Self {
        KillerBuilder::default()
    }

    /// Set `id` of the `KillerBuilder`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::KillerBuilder;
    /// let builder = KillerBuilder::new()
    ///     .id("example-id-or-name")
    ///     .build();
    /// ```
    pub fn id<T>(mut self, id: T) -> Self
        where T: Into<String>
    {
        self.id = id.into();

        self
    }

    /// Set `signal` of the `KillerBuilder`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::KillerBuilder;
    /// let builder = KillerBuilder::new()
    ///     .signal("SIGNAL")
    ///     .build();
    /// ```
    pub fn signal<T>(mut self, signal: T) -> Self
        where T: Into<String>
    {
        self.signal = Some(signal.into());

        self
    }

    /// Build `Killer` from `KillerBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::KillerBuilder;
    /// let builder = KillerBuilder::new().build();
    /// ```
    pub fn build(self) -> Killer {
        Killer {
            id: self.id,
            signal: self.signal
        }
    }
}