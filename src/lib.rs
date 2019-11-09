#![deny(warnings, missing_debug_implementations)]

//! # docker_client
//!
//! `docker_client` is a client that use docker API. Current support API is 1.40.
//!
//! # Examples
//!
//! ```
//! use docker_client::client::DockerClient;
//! use docker_client::container::Creator;
//!
//! fn main() {
//!     // Create docker client
//!     let client = match DockerClient::connect("/var/run/docker.sock") {
//!         Ok(client) => client,
//!         Err(e) => panic!("Cannot connect to socket!"),
//!     };
//!
//!     let creator = Creator::from("alpine").name(Some("test")).build();
//!
//!     // Create container
//!     match client.create_container(creator) {
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

extern crate serde;
extern crate serde_json;
extern crate unix_socket;

mod http;
pub mod container;
pub mod client;
mod converter;
