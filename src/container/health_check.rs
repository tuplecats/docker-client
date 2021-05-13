//!
//! Health check types.
//!
//! The module provides [HealthCheckBuilder](struct.HealthCheckBuilder.html) and [HealthCheck](struct.HealthCheck.html) types
//! used to create a support structure for `Config` structure .
//!
//! # HealthCheckBuilder
//! The [HealthCheckBuilder](struct.HealthCheckBuilder.html) provides a set of methods to create a structure [HealthCheck](struct.HealthCheck.html).
//!
//! # HealthCheck
//! The [HealthCheck](struct.HealthCheck.html) is a helper structure for `Config` structure.
//!
//! # Examples
//!
//! Kill container example.
//! ```rust
//! use docker_client::{DockerClient, Config};
//! use docker_client::container::Create;
//! use docker_client::container::HealthCheck;
//!
//! fn main() {
//!     let client = DockerClient::new();
//!
//!     let health_check = HealthCheck::new().test("echo test").build();
//!
//!     let request = Create::new().name("name")
//!         .config(
//!             Config::with_image("alpine")
//!                 .health_check(Some(health_check))
//!                 .build()
//!         ).build();
//!
//!     match client.create_container(request) {
//!         Ok(container) => { println!("{:?}", container); },
//!         Err(_) => {},
//!     }
//! }
//! ```
use serde::{Deserialize, Serialize};

/// `HealthCheckBuilder` struct
#[derive(Debug, Default)]
pub struct HealthCheckBuilder {
    test: Vec<String>,
    interval: Option<u64>,
    timeout: Option<u64>,
    retries: Option<u64>,
    start_period: Option<u64>
}

/// `HealthCheck` struct.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheck {

    #[serde(skip_serializing_if = "Vec::is_empty", rename = "Test")]
    test: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Interval")]
    interval: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Timeout")]
    timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Retries")]
    retries: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "StartPeriod")]
    start_period: Option<u64>
}

impl HealthCheckBuilder {

    /// Creates a new default instance of `HealthCheckBuilder` to construct a `HealthCheck`.
    pub fn new() -> Self {
        HealthCheckBuilder::default()
    }

    /// Set test field of `HealthCheckBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::HealthCheck;
    /// let builder = HealthCheck::new()
    ///     .test("test-command")
    ///     .build();
    /// ```
    pub fn test<T>(&mut self, cmd: T) -> &mut Self
        where T: Into<String>
    {
        self.test.push(cmd.into());

        self
    }

    /// Set interval field of `HealthCheckBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::HealthCheck;
    /// let builder = HealthCheck::new()
    ///     .interval(Some(1000))
    ///     .build();
    /// ```
    pub fn interval(&mut self, interval: Option<u64>) -> &mut Self {
        self.interval = interval;

        self
    }

    /// Set timeout field of `HealthCheckBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::HealthCheck;
    /// let builder = HealthCheck::new()
    ///     .timeout(Some(1000))
    ///     .build();
    /// ```
    pub fn timeout(&mut self, interval: Option<u64>) -> &mut Self {
        self.timeout = interval;

        self
    }

    /// Set retries field of `HealthCheckBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::HealthCheck;
    /// let builder = HealthCheck::new()
    ///     .retries(Some(3))
    ///     .build();
    /// ```
    pub fn retries(&mut self, interval: Option<u64>) -> &mut Self {
        self.retries = interval;

        self
    }

    /// Set start_period field of `HealthCheckBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::HealthCheck;
    /// let builder = HealthCheck::new()
    ///     .start_period(Some(1000))
    ///     .build();
    /// ```
    pub fn start_period(&mut self, interval: Option<u64>) -> &mut Self {
        self.start_period = interval;

        self
    }

    /// Build `HealthCheck` from `HealthCheckBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::HealthCheck;
    /// let builder = HealthCheck::new().build();
    /// ```
    pub fn build(&self) -> HealthCheck {
        HealthCheck {
            test: self.test.clone(),
            interval: self.interval.clone(),
            timeout: self.timeout.clone(),
            retries: self.retries.clone(),
            start_period: self.start_period.clone()
        }
    }
}

impl HealthCheck {

    /// Creates a new default instance of `HealthCheckBuilder` to construct a `HealthCheck`.
    pub fn new() -> HealthCheckBuilder {
        HealthCheckBuilder::default()
    }

}