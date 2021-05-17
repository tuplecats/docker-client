//!
//! Docker client module.
//!
//! The module provides [DockerClient](struct.DockerClient.html) type used to manage docker containers.
//!
//! # DockerClient
//! The [DockerClient](struct.DockerClient.html) provides a set of methods to manage docker containers used docker API.
//!
//! # API Documentaion
//!
//! API documentaion available at [link](https://docs.docker.com/engine/api/v1.40/)
//!


mod client;
mod error;
mod response;

pub use client::{DockerClient, Auth};
pub use error::{DockerError, ErrorMessage};

