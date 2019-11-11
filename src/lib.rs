#![deny(warnings, missing_docs, missing_debug_implementations)]
//! # docker_client
//!
//! `docker_client` is a client that use docker API. Current support API is 1.40.
//!
//! # Examples
//!
//! ```
//! use docker_client::DockerClient;
//! use docker_client::container::Config;
//!
//! fn main() {
//!     // Create docker client
//!     let client = DockerClient::connect("/var/run/docker.sock");
//!
//!     let config = Config::with_image("alpine").name("test").build();
//!
//!     // Create container
//!     match client.create_container(config) {
//!         Ok(_) => {},
//!         Err(_) => {}
//!     };
//!
//!     // Rename container
//!     match client.rename_container("test", "test1") {
//!         Ok(_) => {},
//!         Err(_) => {}
//!     }
//! }
//! ```

#[cfg(test)]
#[macro_use]
extern crate doc_comment;

#[cfg(test)]
doctest!("../README.MD", another);

extern crate serde;
extern crate serde_json;
extern crate unix_socket;
extern crate hyper;
extern crate hyperlocal;
extern crate futures;
extern crate tokio;
extern crate tokio_core;

pub mod container;
pub mod client;

pub use client::DockerError;
pub use client::DockerClient;
pub use container::{Config, Killer, Remover};