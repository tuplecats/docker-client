//!#![deny(warnings, missing_docs, missing_debug_implementations)]
//! # docker_client
//!
//! `docker_client` is a client that use docker API. Current support API is 1.40.
//!
//! # Examples
//!
//! ```
//! use docker_client::DockerClient;
//! use docker_client::container::{Config, Create};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create docker client
//!     let client = DockerClient::new();
//!
//!     let request = Create::new().config(
//!         Config::with_image("alpine").build()
//!     )
//!     .name("test")
//!     .build();
//!
//!     // Create container
//!     match client.create_container(request).await {
//!         Ok(_) => {},
//!         Err(_) => {}
//!     };
//!
//!     // Rename container
//!     match client.rename_container("test", "test1").await {
//!         Ok(_) => {},
//!         Err(_) => {}
//!     }
//! }
//! ```

//#![deny(warnings)]

#[cfg(test)]
#[macro_use]
extern crate doc_comment;

#[cfg(test)]
doctest!("../README.MD", another);

extern crate serde;
extern crate serde_json;

extern crate hyper;

#[cfg(feature = "unix-socket")]
extern crate hyperlocal;

extern crate futures;
extern crate tokio;
extern crate tokio_core;

extern crate base64;

pub mod container;
pub mod client;
pub mod image;
pub mod volume;
pub mod additionals;
pub mod networks;
pub mod exec;

pub use client::DockerError;
pub use client::DockerClient;
pub use container::{Config, Killer, Remover};