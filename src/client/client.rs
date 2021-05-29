use serde_json as json;

use crate::container::{Killer, Remover, CreatedContainer, WaitCondition, WaitStatus, Create};
use crate::container::FSChanges;
use crate::container::{ShortContainerInfo};
use crate::container::inspect::{Inspect, ContainerInfo};
use crate::container::processes_list::{ProcessesList, TopList};

use crate::client::DockerError;
use crate::client::response::DockerResponse;

use hyper::{Client, Request};

use serde::{Deserialize, Serialize};

#[cfg(feature = "unix-socket")]
use hyperlocal::UnixConnector;

use hyper::Uri;

use crate::image::ShortImageInfo;
use crate::volume::{VolumeCreator, VolumeInfo, DeletedInfo, VolumesList};
use hyper::client::HttpConnector;

use std::env;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum ClientConfig {
    TCP {
        client: Client<HttpConnector, hyper::Body>,
    },
    #[cfg(feature = "unix-socket")]
    UNIX {
        client: Client<UnixConnector, hyper::Body>,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
    pub email: String,

    #[serde(rename = "serveraddress", skip_serializing_if = "Option::is_none")]
    pub server_address: Option<String>
}

/// `DockerClient` struct.
#[derive(Debug, Clone)]
pub struct DockerClient {
    host: String,
    config: ClientConfig,
    auth: Option<Auth>
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
    ///     let client = DockerClient::new();
    /// }
    /// ```
    pub fn new() -> DockerClient {

        match env::var("DOCKER_HOST").ok() {
            Some(host) => {
                #[cfg(feature = "unix-socket")]
                if let Some(path) = host.strip_prefix("unix://") {
                    return DockerClient::unix(path, None);
                }
                DockerClient::stream(host, None)
            },
            #[cfg(feature = "unix-socket")]
            None => {
                DockerClient::unix("/var/run/docker.sock", None)
            },
            None => {
                DockerClient::stream("tcp://localhost:2375", None)
            }
        }

    }

    pub fn with_auth(auth: Auth) -> Self {
        let mut ctx = Self::new();
        ctx.auth = Some(auth);
        ctx
    }

    pub fn registry_auth(&self) -> String {
        //let auth = self.auth.clone().unwrap();
        base64::encode(
            json::to_string(&self.auth.as_ref().unwrap()).unwrap()
        )
    }

    #[cfg(feature = "unix-socket")]
    pub fn unix<T>(host: T, auth: Option<Auth>) -> DockerClient
        where T: Into<String>
    {
        DockerClient {
            host: host.into(),
            config: ClientConfig::UNIX {
                client: Client::builder()
                    .pool_max_idle_per_host(0)
                    .build:: < _, hyper::Body>(UnixConnector::default())
            },
            auth
        }
    }

    pub fn stream<T>(host: T, auth: Option<Auth>) -> DockerClient
        where T: Into<String>
    {
        DockerClient {
            host: host.into().strip_prefix("tcp://").unwrap().to_string(),
            config: ClientConfig::TCP {
                client: Client::builder()
                    .pool_max_idle_per_host(0)
                    .build::<_, hyper::Body>(HttpConnector::new())
            },
            auth
        }
    }

    pub fn make_uri<T>(&self, path: T) -> hyper::Uri
        where T: Into<String>
    {
        match self.config {
            ClientConfig::TCP {..} => {
                Uri::builder().scheme("http")
                    .authority(self.host.as_str())
                    .path_and_query(path.into().as_str())
                    .build()
                    .unwrap()
            },
            #[cfg(feature = "unix-socket")]
            ClientConfig::UNIX {..} => {
                hyperlocal::Uri::new(self.host.as_str(), path.into().as_str()).into()
            }
        }
    }

    async fn execute_async(&self, request: hyper::Request<hyper::Body>) -> Result<DockerResponse, DockerError> {
        let config = self.config.clone();
        let response = match config {
            ClientConfig::TCP { ref client, ..} => client.request(request).await,
            #[cfg(feature = "unix-socket")]
            ClientConfig::UNIX { ref client, ..} => client.request(request).await
        };

        match response {
            Ok(resp) => Ok(
                DockerResponse {
                    status: resp.status().as_u16(),
                    body: hyper::body::to_bytes(resp.into_body()).await.unwrap()
                }
            ),
            Err(_) => Err(DockerError::ClosedConnection)
        }
    }

}


impl DockerClient {

    pub async fn top(&self, request: ProcessesList) -> Result<TopList, DockerError> {

        let uri = self.make_uri(request.get_path());
        let request = Request::get(uri).body(hyper::Body::empty()).unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)
    }

    pub async fn containers_list(&self, request: crate::container::list::Request) -> Result<Vec<ShortContainerInfo>, DockerError> {

        let uri = self.make_uri(request.get_path());
        let request = Request::get(uri).body(hyper::Body::empty()).unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
            .map_err(|e| e)

    }

    /// Create a container
    ///
    /// # Arguments
    /// * `Config` is container to create.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use docker_client::DockerClient;
    /// use docker_client::container::{Create, Config};
    ///
    /// fn main() {
    ///
    ///    let client = DockerClient::new();
    ///
    ///     let config = Create::new().config(Config::with_image("alpine").build()).name("hi").build();
    ///     match client.create_container(config) {
    ///         Ok(_) => {}
    ///         Err(_) => {}
    ///     }
    /// }
    /// ```
    pub async fn create_container(&self, request: Create) -> Result<CreatedContainer, DockerError> {

        let uri = self.make_uri(request.get_path());

        let request = Request::post(uri)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(request.body()))
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    201 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::ContainerExists(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     let changes = client.get_fs_changes("test").unwrap_or(Vec::new());
    ///
    ///     for change in &changes {
    ///         println!("{:?}", change);
    ///     }
    /// }
    /// ```
    pub async fn get_fs_changes<T>(&self, id: T) -> Result<Vec<FSChanges>, DockerError>
        where T: Into<String>
    {

        let uri = self.make_uri(format!("/containers/{}/changes", id.into()));
        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => {
                        let obj: Option<Vec<FSChanges>> = json::from_str(response.body_as_string().as_str()).unwrap();
                        Ok(obj.unwrap_or(Vec::new()))
                    },
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn start_container<T, U>(&self, id: T, _detach_keys: U) -> Result<(), DockerError>
        where
            T: Into<String>,
            U: Into<String>
    {

        let uri = self.make_uri(format!("/containers/{}/start", id.into()));
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    304 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn stop_container<T>(&self, id: T, _wait: Option<i32>) -> Result<(), DockerError>
        where T: Into<String>
    {
        let path = format!("/containers/{}/stop", id.into());

        let uri = self.make_uri(path);
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    304 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn pause_container<T>(&self, id: T) -> Result<(), DockerError>
        where T: Into<String>
    {

        let uri = self.make_uri(format!("/containers/{}/pause", id.into()));
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn unpause_container<T>(&self, id: T) -> Result<(), DockerError>
        where T: Into<String> {

        let uri = self.make_uri(format!("/containers/{}/unpause", id.into()));
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn rename_container<T>(&self, id: T, new_name: T) -> Result<(), DockerError>
        where T: Into<String>
    {

        let uri = self.make_uri(format!("/containers/{}/rename?name={}", id.into(), new_name.into()));
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::ContainerExists(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn kill_container(&self, killer: Killer) -> Result<(), DockerError> {

        let uri = self.make_uri(killer.get_path());
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::NotRunning(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn remove_container(&self, remover: Remover) -> Result<(), DockerError> {

        let uri = self.make_uri(remover.get_path());
        let request = Request::delete(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::NotRunning(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    /// use docker_client::container::inspect::Inspect;
    ///
    /// fn main() {
    ///
    ///    let client = DockerClient::new();
    ///
    ///     match client.inspect_container(Inspect::container("vigilant_antonelli".to_string())) {
    ///         Ok(s) => { println!("{:?}", s) }
    ///         Err(e) => {}
    ///     }
    ///
    /// }
    /// ```
    pub async fn inspect_container(&self, request: Inspect) -> Result<ContainerInfo, DockerError> {

        let uri = self.make_uri(request.get_path());
        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     match client.get_container_log("test-container") {
    ///         Ok(log) => { println!("Log: {}", log); }
    ///         Err(e) => { println!("Error: {:?}", e); }
    ///     }
    ///
    /// }
    /// ```
    pub async fn get_container_log<T>(&self, id: T) -> Result<String, DockerError>
        where T: Into<String>
    {

        let uri = self.make_uri(format!("/containers/{}/logs?stdout=true", id.into()));
        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(response.body_as_string()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     match client.wait_container("test-container", WaitCondition::NotRunning) {
    ///         Ok(status) => { println!("Status: {:?}", status); }
    ///         Err(e) => { println!("Error: {:?}", e); }
    ///     }
    ///
    /// }
    /// ```
    pub async fn wait_container<T>(&self, id: T, condition: WaitCondition) -> Result<WaitStatus, DockerError>
        where T: Into<String>
    {
        let uri = self.make_uri(format!("/containers/{}/wait?condition={}", id.into(), condition.to_string()));
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    /// # use std::path::Path;
    /// fn main() {
    ///    let client = DockerClient::new();
    ///
    ///     let mut path = std::env::temp_dir();
    ///     path.push("export_container");
    ///     path.set_extension("tar");
    ///
    ///     match client.export_container("test-container", path.as_path()) {
    ///         Ok(_) => {},
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub async fn export_container<T>(&self, id: T, file: &Path) -> Result<(), DockerError>
        where T: Into<String>
    {

        let uri = self.make_uri(format!("/containers/{}/export", id.into()));
        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => {
                        response.save_to_file(file)
                            .map_err(|_| DockerError::UnknownStatus)
                    },
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     match client.get_image_list() {
    ///         Ok(list) => { println!("{:?}", list); },
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub async fn get_image_list(&self) -> Result<Vec<ShortImageInfo>, DockerError> {

        let uri = self.make_uri("/images/json");
        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
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
    pub async fn create_volume(&self, volume: VolumeCreator) -> Result<(), DockerError> {

        let uri = self.make_uri("/volumes/create");
        let request = Request::post(uri)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(json::to_string(&volume).unwrap()))
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    201 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     match client.inspect_volume("test") {
    ///         Ok(info) => { println!("{:?}", info); },
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub async fn inspect_volume<T>(&self, name: T) -> Result<VolumeInfo, DockerError>
        where T: Into<String>
    {

        let uri = self.make_uri(format!("/volumes/{}", name.into()));
        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     match client.remove_volume("test", false) {
    ///         Ok(_) => {},
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub async fn remove_volume<T>(&self, name: T, force: bool) -> Result<(), DockerError>
        where T: Into<String>
    {

        let uri = self.make_uri(format!("/volumes/{}?force={}", name.into(), force.to_string()));
        let request = Request::delete(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    204 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::Busy(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     match client.delete_unused_volumes() {
    ///         Ok(_) => {},
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub async fn delete_unused_volumes(&self) -> Result<DeletedInfo, DockerError> {

        let uri = self.make_uri("/volumes/prune");
        let request = Request::post(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
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
    ///    let client = DockerClient::new();
    ///
    ///     match client.get_volumes_list() {
    ///         Ok(list) => { println!("{:?}", list); },
    ///         Err(e) => { println!("Error: {:?}", e); },
    ///     }
    ///
    /// }
    /// ```
    pub async fn get_volumes_list(&self) -> Result<VolumesList, DockerError> {

        let uri = self.make_uri("/volumes");
        let request = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }


    pub async fn pull_image(&self, request: crate::image::create::Request) -> Result<(), DockerError> {
        let uri = self.make_uri(request.get_path());
        let mut request_builder = Request::post(uri);

        if self.auth.is_some() {
            request_builder = request_builder.header("X-Registry-Auth", self.registry_auth());
        }

        let request = request_builder.body(hyper::Body::empty()).unwrap();

        self.execute_async(request).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    pub async fn create_network(&self, request: crate::networks::create::Request) -> Result<crate::networks::create::CreatedNetwork, DockerError> {
        let uri = self.make_uri(request.get_path());
        let req = Request::post(uri)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(hyper::Body::from(json::to_string(&request).unwrap()))
            .unwrap();

        self.execute_async(req).await
            .and_then(|response| {
                match response.status {
                    201 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::NetworkExists(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    pub async fn inspect_network(&self, request: crate::networks::inspect::Request) -> Result<(), DockerError> {
        let uri = self.make_uri(request.get_path());
        let req = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(req).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    pub async fn connect_container_to_network(&self, request: crate::networks::connect::Request) -> Result<(), DockerError> {
        let uri = self.make_uri(request.get_path());
        let req = Request::post(uri)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(hyper::Body::from(json::to_string(&request).unwrap()))
            .unwrap();

        self.execute_async(req).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    pub async fn create_exec_instance(&self, request: crate::exec::create::Request) -> Result<String, DockerError> {
        let uri = self.make_uri(request.get_path());
        let req = Request::post(uri)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(hyper::Body::from(json::to_string(&request).unwrap()))
            .unwrap();

        self.execute_async(req).await
            .and_then(|response| {
                match response.status {
                    201 => Ok(json::from_str::<crate::exec::create::Exec>(response.body_as_string().as_str()).unwrap().id),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::ContainerPaused(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    pub async fn start_exec(&self, id: String) -> Result<(), DockerError> {
        let uri = self.make_uri(format!("/exec/{}/start", &id));
        let req = Request::post(uri)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(hyper::body::Body::from("{}"))
            .unwrap();

        self.execute_async(req).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(()),
                    400 => Err(DockerError::BadParameters(json::from_str(response.body_as_string().as_str()).unwrap())),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    409 => Err(DockerError::ContainerPaused(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

    pub async fn inspect_exec(&self, id: String) -> Result<crate::exec::inspect::ExecStatus, DockerError> {
        let uri = self.make_uri(format!("/exec/{}/json", &id));
        let req = Request::get(uri)
            .body(hyper::Body::empty())
            .unwrap();

        self.execute_async(req).await
            .and_then(|response| {
                match response.status {
                    200 => Ok(json::from_str(response.body_as_string().as_str()).unwrap()),
                    404 => Err(DockerError::NotFound(json::from_str(response.body_as_string().as_str()).unwrap())),
                    500 => Err(DockerError::ServerError(json::from_str(response.body_as_string().as_str()).unwrap())),
                    _ => Err(DockerError::UnknownStatus),
                }
            })
    }

}