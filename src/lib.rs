#![deny(warnings, missing_debug_implementations)]

/// # docker_client
///
/// docker_client is a client that use docker api
///
/// # Examples
/// ```
/// use docker_client::client::DockerClient;
/// fn main() {
///     let client = match DockerClient::connect("/path/to/unix/socket") {
///         Ok(client) => client,
///         Err(e) => panic!("Cannot connect to socket!"),
///     };
///
///     // Rename container
///     match client.rename_container("container-id", "new-container-name") {
///         Ok(()) => {},
///         Err(_) => panic!("Cannot rename container!"),
///     }
/// }
/// ```

extern crate serde;
extern crate serde_json;
extern crate unix_socket;

mod http;
pub mod container;
pub mod client;
mod converter;
