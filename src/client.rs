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


use serde_json as json;

use serde::Deserialize;
use crate::container::{Config, Killer, Remover, CreatedContainer};
use hyper::{Client, Request};
use hyper::rt::{Stream, Future};
use hyperlocal::{UnixConnector, Uri};
use tokio_core::reactor::Core;
use tokio::prelude::stream::Concat2;
use tokio::prelude::Async;
use crate::container::info::ContainerInfo;

/// `DockerClient` struct.
#[derive(Debug)]
pub struct DockerClient {
    path: String,
    client: Client<UnixConnector, hyper::Body>,
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


struct DockerFuture {
    status: hyper::StatusCode,
    body: Concat2<hyper::Body>,
}

#[derive(Clone)]
struct DockerResponse {
    status: u16,
    body: String,
}

impl Future for DockerFuture {
    type Item = DockerResponse;
    type Error = hyper::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.body.poll().map(|s| {
            match s {
                Async::NotReady => Ok(Async::<Self::Item>::NotReady),
                Async::Ready(s) => {
                    Ok(Async::Ready(DockerResponse{
                        status: self.status.as_u16(),
                        body: std::str::from_utf8(&s).unwrap_or("").to_string(),
                    }))
                },
                //_ => Ok(Async::<Self::Item>::NotReady),
            }
        }).unwrap()
    }
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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    /// }
    /// ```
    pub fn connect<T>(path: T) -> DockerClient
        where T: Into<String>
    {
        DockerClient {
            path: path.into(),
            client: Client::builder()
                .keep_alive(false)
                .build::<_, hyper::Body>(UnixConnector::new()),
        }
    }

    fn execute(&self, request: hyper::Request<hyper::Body>) -> DockerResponse {
        let mut core = Core::new().unwrap();

        let client = self.client.clone();

        let future = client.request(request)
            .and_then(|res|
                DockerFuture {
                    status: res.status(),
                    body: res.into_body().concat2()
                }
            )
            .map_err(hyper::Error::from)
            .map_err(Box::<hyper::Error>::from);

        core.run(future).unwrap()
    }

}

impl DockerClient {


    /// Create a container
    ///
    /// # Arguments
    /// * `Config` is container to create.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::DockerClient;
    /// use docker_client::container::Config;
    ///
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     let config = Config::with_image("alpine").name("test").build();
    ///     match client.create_container(config) {
    ///         Ok(_) => {},
    ///         Err(_) => {}
    ///     }
    /// }
    /// ```
    pub fn create_container(&self, config: Config) -> Result<CreatedContainer, DockerError> {

        let uri: hyper::Uri = Uri::new(self.path.as_str(), config.get_path().as_str()).into();

        let request = Request::post(uri)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(json::to_string(&config).unwrap()))
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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
        let path = format!("/containers/{}/changes", id);

        let url: hyper::Uri = Uri::new(self.path.as_str(), path.as_str()).into();

        let request = Request::get(url)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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
    pub fn start_container<T>(&self, id: T, _detach_keys: T) -> Result<(), DockerError>
        where T: Into<String> {

        let id = id.into();
        let path = format!("/containers/{}/start", id);

        let uri: hyper::Uri = Uri::new(self.path.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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
    pub fn stop_container<T>(&self, id: T, _wait: Option<i32>) -> Result<(), DockerError>
        where T: Into<String>
    {
        let path = format!("/containers/{}/stop", id.into());

        let uri: hyper::Uri = Uri::new(self.path.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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
        where T: Into<String>
    {
        let path = format!("/containers/{}/pause", id.into());

        let uri: hyper::Uri = Uri::new(self.path.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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

        let path = format!("/containers/{}/unpause", id.into());

        let uri: hyper::Uri = Uri::new(self.path.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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

        let path = format!("/containers/{}/rename?name={}", id.into(), new_name.into());

        let uri: hyper::Uri = Uri::new(self.path.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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

        let uri: hyper::Uri = Uri::new(self.path.as_str(), killer.get_path().as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

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
    ///     let client = DockerClient::connect("/var/run/docker.sock");
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

        let uri: hyper::Uri = Uri::new(self.path.as_str(), remover.get_path().as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);

        match response.status {
            204 => Ok(()),
            400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            409 => Err(DockerError::NotRunning(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

    /// Inspect a container.
    ///
    /// Return `ContainerInfo` structure about a container.
    ///
    /// # Arguments
    /// * `id` - ID or name of the container.
    /// * `size` - Return the size of container as fields SizeRw and SizeRootFs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::{DockerClient, DockerError};
    ///
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.inspect_container("test", true) {
    ///         Ok(s) => { println!("{:?}", s) },
    ///         Err(e) => {},
    ///     }
    ///
    /// }
    /// ```
    pub fn inspect_container<T>(&self, id: T, size: bool) -> Result<ContainerInfo, DockerError>
        where T: Into<String>
    {
        let path = format!("/containers/{}/json?size={}", id.into(), size.to_string());

        let uri: hyper::Uri = Uri::new(self.path.as_str(), path.as_str()).into();

        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        let response = self.execute(request);
        std::fs::write("text.txt", response.body.clone()).unwrap();

        match response.status {
            200 => Ok(json::from_str(response.body.as_str()).unwrap()),
            404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
            500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
            _ => Err(DockerError::UnknownStatus),
        }
    }

}