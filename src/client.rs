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

use unix_socket::UnixStream;
use serde_json as json;

use crate::container::ToRequest;
use serde::Deserialize;
use crate::container::{Creator, Killer, Remover, CreatedContainer};
use crate::http::{Request, URI};

/// `DockerClient` struct.
#[derive(Debug)]
pub struct DockerClient {
    stream: UnixStream,
}

/// `ErrorMessage` struct.
#[derive(Deserialize, Debug)]
pub struct ErrorMessage {
    /// Error message get from response.
    pub message: String
}

/// `DockerError` enum.
#[derive(Debug)]
pub enum DockerError {
    /// Bad parameters (HTTP status is 401)
    BadParameters(ErrorMessage), // 401

    /// Server error (HTTP status is 500)
    ServerError(ErrorMessage), // 500

    /// Server error (HTTP status is 404)
    NotFound(ErrorMessage), // 404

    /// Server error (HTTP status is 409)
    NotRunning(ErrorMessage), // 409

    /// Server error (HTTP status is 304)
    AlreadyStarted(ErrorMessage), // 304

    /// Server error (HTTP status is 409)
    ContainerExists(ErrorMessage), // 409

    /// Unknown staus
    UnknownStatus
}

/// `FSChanges` struct.
#[derive(Deserialize, Debug)]
pub struct FSChanges {
    #[serde(rename(deserialize = "Path"))]
    path: String,

    #[serde(rename(deserialize = "Kind"))]
    kind: i32,
}

impl DockerClient {

    /// Connect to docker
    ///
    /// # Arguments
    /// * `sock` - connection address
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::DockerClient;
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    /// }
    /// ```
    pub fn connect<T>(sock: T) -> std::io::Result<DockerClient>
        where T: Into<String>
    {
        Ok(DockerClient {
            stream: UnixStream::connect(sock.into())?
        })
    }

    /// Create a container
    ///
    /// # Arguments
    /// * `creator` is container to create.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::DockerClient;
    /// use docker_client::container::Creator;
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     let creator = Creator::with_image("alpine").name("test").build();
    ///     match client.create_container(creator) {
    ///         Ok(_) => {},
    ///         Err(_) => {}
    ///     }
    /// }
    /// ```
    pub fn create_container(&self, creator: Creator) -> Result<CreatedContainer, DockerError> {
        let response = creator.to_request().send(self.stream.try_clone().unwrap());

        match response.status {
            201 => Ok(json::from_str(response.body.as_str()).unwrap()),
            400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            409 => Err(DockerError::ContainerExists(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

    /// Returns which files in a container's filesystem have been added, deleted, or modified.
    ///
    /// # Arguments
    /// * `id` - ID or name of the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::DockerClient;
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     let changes = client.get_fs_changes("test").unwrap_or(Vec::new());
    ///
    ///     for change in &changes {
    ///         println!("{:?}", change);
    ///     }
    /// }
    /// ```
    pub fn get_fs_changes<T>(&self, id: T) -> Result<Vec<FSChanges>, DockerError>
        where T: Into<String> {

        let id = id.into();

        let url = URI::with_path(format!("/containers/{}/changes", id)).build();

        let response = Request::get()
            .url(url)
            .build()
            .send(self.stream.try_clone().unwrap());

        match response.status {
            200 => {
                let obj: Option<Vec<FSChanges>> = json::from_str(response.body.as_str()).unwrap();
                Ok(obj.unwrap_or(Vec::new()))
            },
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

    /// Start a container.
    ///
    /// # Arguments
    /// * `id` - ID or name of the container.
    /// * `detach_keys` - The key sequence for detaching a container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     match client.start_container("test", "-d") {
    ///         Ok(_) => {},
    ///         Err(e) => {
    ///             match e {
    ///                 DockerError::NotFound(e) => println!("{}", e.message),
    ///                 DockerError::ServerError(e) => println!("{}", e.message),
    ///                 _ => {}
    ///             }
    ///         },
    ///     }
    ///
    /// }
    /// ```
    pub fn start_container<T>(&self, id: T, detach_keys: T) -> Result<(), DockerError>
        where T: Into<String> {

        let id = id.into();
        let detach_keys = detach_keys.into();

        let url = URI::with_path(format!("/containers/{}/start", id))
            .parameter("detachKeys", detach_keys.as_str())
            .build();

        let response = Request::post()
            .url(url)
            .build()
            .send(self.stream.try_clone().unwrap());

        match response.status {
            204 => Ok(()),
            304 => Ok(()),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

    /// Stop a container.
    ///
    /// # Arguments
    /// * `id` - ID or name of the container.
    /// * `wait` - Number of seconds to wait before killing the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     match client.stop_container("test", Some(12)) {
    ///         Ok(_) => {},
    ///         Err(e) => {
    ///             match e {
    ///                 DockerError::NotFound(e) => println!("{}", e.message),
    ///                 DockerError::ServerError(e) => println!("{}", e.message),
    ///                 _ => {}
    ///             }
    ///         },
    ///     }
    ///
    /// }
    /// ```
    pub fn stop_container<T>(&self, id: T, wait: Option<i32>) -> Result<(), DockerError>
        where T: Into<String> {

        let mut url = URI::with_path(format!("/containers/{}/stop", id.into()));

        if wait.is_some() {
            url.parameter("t", wait.unwrap().to_string());
        }

        let response = Request::post()
            .url(url.build())
            .build()
            .send(self.stream.try_clone().unwrap());

        match response.status {
            204 => Ok(()),
            304 => Ok(()),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

    /// Pause a container.
    ///
    /// # Arguments
    /// * `id` - ID or name of the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     match client.pause_container("test") {
    ///         Ok(_) => {},
    ///         Err(e) => {
    ///             match e {
    ///                 DockerError::NotFound(e) => println!("{}", e.message),
    ///                 DockerError::ServerError(e) => println!("{}", e.message),
    ///                 _ => {}
    ///             }
    ///         },
    ///     }
    ///
    /// }
    /// ```
    pub fn pause_container<T>(&self, id: T) -> Result<(), DockerError>
        where T: Into<String> {

        let url = URI::with_path(format!("/containers/{}/pause", id.into())).build();

        let response = Request::post()
            .url(url)
            .build()
            .send(self.stream.try_clone().unwrap());

        match response.status {
            204 => Ok(()),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }

    }

    /// Unpause a container.
    ///
    /// # Arguments
    /// * `id` - ID or name of the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     match client.unpause_container("test") {
    ///         Ok(_) => {},
    ///         Err(e) => {
    ///             match e {
    ///                 DockerError::NotFound(e) => println!("{}", e.message),
    ///                 DockerError::ServerError(e) => println!("{}", e.message),
    ///                 _ => {}
    ///             }
    ///         },
    ///     }
    ///
    /// }
    /// ```
    pub fn unpause_container<T>(&self, id: T) -> Result<(), DockerError>
        where T: Into<String> {

        let url = URI::with_path(format!("/containers/{}/unpause", id.into())).build();

        let response = Request::post()
            .url(url)
            .build()
            .send(self.stream.try_clone().unwrap());

        match response.status {
            204 => Ok(()),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }

    }

    /// Rename a container.
    ///
    /// # Arguments
    /// * `id` - ID or name of the container.
    /// * `new_name` - New name for the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    ///
    /// fn main() {
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     match client.rename_container("test", "test1") {
    ///         Ok(_) => {},
    ///         Err(e) => {
    ///             match e {
    ///                 DockerError::NotFound(e) => println!("{}", e.message),
    ///                 DockerError::ContainerExists(e) => println!("{}", e.message),
    ///                 DockerError::ServerError(e) => println!("{}", e.message),
    ///                 _ => {}
    ///             }
    ///         },
    ///     }
    ///
    /// }
    /// ```
    pub fn rename_container<T>(&self, id: T, new_name: T) -> Result<(), DockerError>
        where T: Into<String> {

        let url = URI::with_path(format!("/containers/{}/rename", id.into()))
            .parameter("name", new_name.into())
            .build();

        let response = Request::post()
            .url(url)
            .build()
            .send(self.stream.try_clone().unwrap());

        match response.status {
            204 => Ok(()),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            409 => Err(DockerError::ContainerExists(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

    /// Kill a container.
    ///
    /// # Arguments
    /// * `killer` is a struct with metadata to kill a container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    /// use docker_client::container::Killer;
    ///
    /// fn main() {
    ///
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     let killer = Killer::new()
    ///         .id("test")
    ///         .build();
    ///
    ///     match client.kill_container(killer) {
    ///         Ok(_) => {}
    ///         Err(e) => {
    ///             match e {
    ///                 DockerError::NotFound(e) => println!("{}", e.message),
    ///                 DockerError::NotRunning(e) => println!("{}", e.message),
    ///                 DockerError::ServerError(e) => println!("{}", e.message),
    ///                 _ => {}
    ///             }
    ///         }
    ///     }
    ///
    /// }
    /// ```
    pub fn kill_container(&self, killer: Killer) -> Result<(), DockerError> {
        let response = killer.to_request().send(self.stream.try_clone().unwrap());

        match response.status {
            204 => Ok(()),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            409 => Err(DockerError::NotRunning(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

    /// Remove a container.
    ///
    /// # Arguments
    /// * `remover` is a struct with metadata to remove a container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    /// use docker_client::container::Remover;
    ///
    /// fn main() {
    ///
    ///     let client = match DockerClient::connect("/var/run/docker.sock") {
    ///         Ok(client) => client,
    ///         Err(e) => panic!("Cannot connect to socket!"),
    ///     };
    ///
    ///     let remover = Remover::new()
    ///         .id("test")
    ///         .with_remove_volumes(true)
    ///         .build();
    ///
    ///     match client.remove_container(remover) {
    ///         Ok(_) => {}
    ///         Err(e) => {
    ///             match e {
    ///                 DockerError::BadParameters(e) => println!("{}", e.message),
    ///                 DockerError::NotFound(e) => println!("{}", e.message),
    ///                 DockerError::NotRunning(e) => println!("{}", e.message),
    ///                 DockerError::ServerError(e) => println!("{}", e.message),
    ///                 _ => {}
    ///             }
    ///         }
    ///     }
    ///
    /// }
    /// ```
    pub fn remove_container(&self, remover: Remover) -> Result<(), DockerError> {
        let response = remover.to_request().send(self.stream.try_clone().unwrap());

        match response.status {
            204 => Ok(()),
            400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            409 => Err(DockerError::NotRunning(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

}