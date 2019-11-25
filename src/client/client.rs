use serde_json as json;

use crate::container::{Config, Killer, Remover, CreatedContainer, WaitCondition, WaitStatus};
use crate::container::info::ContainerInfo;
use crate::container::FSChanges;

use crate::client::DockerError;
use crate::client::future::DockerFuture;
use crate::client::response::DockerResponse;

use hyper::{Client, Request};
use hyper::rt::{Future, Stream};

use hyperlocal::{UnixConnector, Uri};
use tokio_core::reactor::Core;
use crate::image::ShortImageInfo;
use crate::volume::{VolumeCreator, VolumeInfo, DeletedInfo, VolumesList};

/// `DockerClient` struct.
#[derive(Debug)]
pub struct DockerClient {
    socket: String,
    client: Client<UnixConnector, hyper::Body>,
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
            socket: path.into(),
            client: Client::builder()
                .keep_alive(false)
                .build::<_, hyper::Body>(UnixConnector::new()),
        }
    }

    fn execute(&self, request: hyper::Request<hyper::Body>) -> Result<DockerResponse, DockerError> {
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

        match core.run(future) {
            Ok(v) => Ok(v),
            Err(_) => Err(DockerError::ClosedConnection),
        }
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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), config.get_path().as_str()).into();

        let request = Request::post(uri)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(json::to_string(&config).unwrap()))
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    201 => Ok(json::from_str(response.body.as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    409 => Err(DockerError::ContainerExists(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
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
        where T: Into<String>
    {
        let path = format!("/containers/{}/changes", id.into());

        let url: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::get(url)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => {
                        let obj: Option<Vec<FSChanges>> = json::from_str(response.body.as_str()).unwrap();
                        Ok(obj.unwrap_or(Vec::new()))
                    },
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)


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
        where T: Into<String>
    {
        let path = format!("/containers/{}/start", id.into());

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    304 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)

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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    304 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    409 => Err(DockerError::ContainerExists(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), killer.get_path().as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    409 => Err(DockerError::NotRunning(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), remover.get_path().as_str()).into();

        let request = Request::delete(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    409 => Err(DockerError::NotRunning(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
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

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body.as_str()).unwrap()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
    }

    /// Get container logs
    ///
    /// Get stdout and stderr logs from a container.
    ///
    /// # Note
    /// This endpoint works only for containers with the json-file or journald logging driver.
    ///
    /// # Arguments
    /// `id` - ID or name of the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.get_container_log("test-container") {
    ///         Ok(log) => { println!("Log: {}", log); }
    ///         Err(e) => { println!("Error: {:?}", e); }
    ///     }
    ///
    /// }
    /// ```
    pub fn get_container_log<T>(&self, id: T) -> Result<String, DockerError>
        where T: Into<String>
    {
        let path = format!("/containers/{}/logs?stdout=true", id.into());

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(response.body),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
    }


    /// Wait for a container
    ///
    /// Block until a container stops, then returns the exit code.
    ///
    /// # Arguments
    /// `id` - ID or name of the container.
    /// `condition` - Wait until a container state reaches the given condition, either 'not-running' (default), 'next-exit', or 'removed'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// # use docker_client::container::WaitCondition;
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.wait_container("test-container", WaitCondition::NotRunning) {
    ///         Ok(status) => { println!("Status: {:?}", status); }
    ///         Err(e) => { println!("Error: {:?}", e); }
    ///     }
    ///
    /// }
    /// ```
    pub fn wait_container<T>(&self, id: T, condition: WaitCondition) -> Result<WaitStatus, DockerError>
        where T: Into<String>
    {
        let path = format!("/containers/{}/wait?condition={}", id.into(), condition.to_string());

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body.as_str()).unwrap()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }


    /// Export a container
    ///
    /// Return empty object or DockerError
    ///
    /// # Arguments
    /// `id` - ID or name of the container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.export_container("test-container") {
    ///         Ok(_) => {},
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub fn export_container<T>(&self, id: T) -> Result<(), DockerError>
        where T: Into<String>
    {
        let path = format!("/containers/{}/export", id.into());

        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    /// Get images list
    ///
    /// Return vector of ShortImageInfo or DockerError
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.get_image_list() {
    ///         Ok(list) => { println!("{:?}", list); },
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub fn get_image_list(&self) -> Result<Vec<ShortImageInfo>, DockerError> {

        let path = "/images/json";
        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path).into();

        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body.as_str()).unwrap()),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    /// Create a volume
    ///
    /// Return empty object or DockerError
    ///
    /// # Arguments
    /// * `volume` - VolumeCreator struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// # use docker_client::volume::VolumeCreator;
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     let creator = VolumeCreator::builder()
    ///         .name("test")
    ///         .build();
    ///
    ///     match client.create_volume(creator) {
    ///         Ok(_) => {},
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub fn create_volume(&self, volume: VolumeCreator) -> Result<(), DockerError> {

        let path = "/volumes/create";
        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path).into();

        let request = Request::post(uri)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(json::to_string(&volume).unwrap()))
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    201 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    /// Inspect volume
    ///
    /// Return VolumeInfo or DockerError
    ///
    /// # Arguments
    /// * `name` - ID or name of the volume.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.inspect_volume("test") {
    ///         Ok(info) => { println!("{:?}", info); },
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub fn inspect_volume<T>(&self, name: T) -> Result<VolumeInfo, DockerError>
        where T: Into<String>
    {

        let path = format!("/volumes/{}", name.into());
        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body.as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    /// Remove volume
    ///
    /// Instruct the driver to remove the volume.
    ///
    /// # Arguments
    /// * `name` - ID or name of the volume.
    /// * `force` - Force the removal of the volume.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.remove_volume("test", false) {
    ///         Ok(_) => {},
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub fn remove_volume<T>(&self, name: T, force: bool) -> Result<(), DockerError>
        where T: Into<String>
    {

        let path = format!("/volumes/{}?force={}", name.into(), force.to_string());
        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path.as_str()).into();

        let request = Request::delete(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body.as_str()).unwrap())),
                    409 => Err(DockerError::Busy(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    /// Delete unused volumes
    ///
    /// Return empty or DockerError
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.delete_unused_volumes() {
    ///         Ok(_) => {},
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub fn delete_unused_volumes(&self) -> Result<DeletedInfo, DockerError> {

        let path = "/volumes/prune";
        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path).into();

        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body.as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    /// Get volumes list
    ///
    /// Return VolumesList or DockerError
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::{DockerClient, DockerError};
    /// fn main() {
    ///     let client = DockerClient::connect("/var/run/docker.sock");
    ///
    ///     match client.get_volumes_list() {
    ///         Ok(list) => { println!("{:?}", list); },
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub fn get_volumes_list(&self) -> Result<VolumesList, DockerError> {

        let path = "/volumes";
        let uri: hyper::Uri = Uri::new(self.socket.as_str(), path).into();

        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute(request)
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body.as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body.as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body.as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }


}