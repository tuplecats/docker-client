use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::additionals::network::NetworkSettings;
use crate::container::HealthCheck;

use crate::additionals::serde_helpers::*;
use crate::additionals::host::host_config::HostConfig;

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    hostname: Option<String>,
    domain_name: Option<String>,
    user: Option<String>,
    attach_stdin: Option<bool>,
    attach_stdout: Option<bool>,
    attach_stderr: Option<bool>,
    exposed_ports: HashMap<String, EmptyObject>,
    tty: Option<bool>,
    open_stdin: Option<bool>,
    stdin_once: Option<bool>,
    env: Vec<String>,
    cmd: Vec<String>,
    health_check: Option<HealthCheck>,
    args_escaped: Option<bool>,
    image: Option<String>,
    volumes: HashMap<String, EmptyObject>,
    work_dir: Option<String>,
    entry_point: Vec<String>,
    network_disabled: Option<bool>,
    mac_address: Option<String>,
    on_build: Vec<String>,
    labels: HashMap<String, String>,
    stop_signal: Option<String>,
    stop_timeout: Option<i32>,
    shell: Vec<String>,
    host_config: Option<HostConfig>,
    network_config: Option<NetworkSettings>,
}

impl ConfigBuilder {

    /// Creates a new default instance of `ConfigBuilder` to construct a `Config`.
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn host_config(&mut self, cfg: HostConfig) -> &mut Self {
        self.host_config = Some(cfg);

        self
    }

    /// Creates a new `ConfigBuilder` initialized with `image`.
    ///
    /// This method returns an instance of `ConfigBuilder` which can be used to create a `Config`.
    ///
    /// # Examples
    ///
    /// Create a new `CreationBuilder` with image.
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("example-image").build();
    /// ```
    pub fn with_image<T>(image: T) -> Self
        where T: Into<String>
    {
        let mut builder = ConfigBuilder::new();
        builder.image = Some(image.into());
        builder
    }

    /// Set hostname for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .hostname("example-hostname")
    ///     .build();
    /// ```
    pub fn hostname<T>(&mut self, name: T) -> &mut Self
        where T: Into<String>
    {
        self.hostname = Some(name.into());

        self
    }

    /// Set domain name for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .domain_name("example-domainname")
    ///     .build();
    /// ```
    pub fn domain_name<T>(&mut self, name: T) -> &mut Self
        where T: Into<String>
    {
        self.domain_name = Some(name.into());

        self
    }

    /// Set user for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .user("example-user")
    ///     .build();
    /// ```
    pub fn user<T>(&mut self, name: T) -> &mut Self
        where T: Into<String>
    {
        self.user = Some(name.into());

        self
    }

    /// Set boolean flag `attach_stdin` for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .attach_stdin(true)
    ///     .build();
    /// ```
    pub fn attach_stdin(&mut self, b: bool) -> &mut Self {
        self.attach_stdin = Some(b);

        self
    }

    /// Set boolean flag `attach_stdout` for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .attach_stdout(true)
    ///     .build();
    /// ```
    pub fn attach_stdout(&mut self, b: bool) -> &mut Self {
        self.attach_stdout = Some(b);

        self
    }

    /// Set boolean flag `attach_stderr` for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .attach_stderr(true)
    ///     .build();
    /// ```
    pub fn attach_stderr(&mut self, b: bool) -> &mut Self {
        self.attach_stderr = Some(b);

        self
    }

    /// Expose port of container to this Config builder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .expose_port("22/tcp")
    ///     .build();
    /// ```
    pub fn expose_port<T>(&mut self, port: T) -> &mut Self
        where T: Into<String>
    {
        self.exposed_ports.insert(port.into(), EmptyObject{});

        self
    }

    /// Set boolean flag `tty` for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .tty(true)
    ///     .build();
    /// ```
    pub fn tty(&mut self, b: bool) -> &mut Self {
        self.tty = Some(b);

        self
    }

    /// Set boolean flag `open_stdin` for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .open_stdin(true)
    ///     .build();
    /// ```
    pub fn open_stdin(&mut self, b: bool) -> &mut Self {
        self.open_stdin = Some(b);

        self
    }

    /// Set boolean flag `stdin_once` for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .stdin_once(true)
    ///     .build();
    /// ```
    pub fn stdin_once(&mut self, b: bool) -> &mut Self {
        self.stdin_once = Some(b);

        self
    }

    /// Append environment variable for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .env("VAR=example-value")
    ///     .build();
    /// ```
    pub fn env<T>(&mut self, env: T) -> &mut Self
        where T: Into<String>
    {
        self.env.push(env.into());

        self
    }

    /// Append command for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .cmd("example-cmd")
    ///     .build();
    /// ```
    pub fn cmd<T>(&mut self, cmd: T) -> &mut Self
        where T: Into<String>
    {
        self.cmd.push(cmd.into());

        self
    }

    /// Set `HealthCheck` for this container.
    pub fn health_check(&mut self, health_check: Option<HealthCheck>) -> &mut Self {
        self.health_check = health_check;

        self
    }

    /// Set args escaped for this container.
    ///
    /// # Note
    ///
    /// Only for Windows.
    pub fn args_escaped(&mut self, b: bool) -> &mut Self {
        self.args_escaped = Some(b);

        self
    }

    /// Set image for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::new()
    ///     .image("alpine")
    ///     .build();
    /// ```
    pub fn image<T>(&mut self, image: T) -> &mut Self
        where T: Into<String>
    {
        self.image = Some(image.into());

        self
    }

    /// Append volume for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .volume("/path/to/volume")
    ///     .build();
    /// ```
    pub fn volume<T>(&mut self, volume: T) -> &mut Self
        where T: Into<String>
    {
        self.volumes.insert(volume.into(), EmptyObject{});

        self
    }

    /// Set work directory for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .work_dir("/path/to/work_dir")
    ///     .build();
    /// ```
    pub fn work_dir<T>(&mut self, work_dir: T) -> &mut Self
        where T: Into<String>
    {
        self.work_dir = Some(work_dir.into());

        self
    }

    /// Append entry point script.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .entry_point("example-entry-point")
    ///     .build();
    /// ```
    pub fn entry_point<T>(&mut self, entry_point: T) -> &mut Self
        where T: Into<String>
    {
        self.entry_point.push(entry_point.into());

        self
    }

    /// Set flag `network_disabled` for this container.
    ///
    /// # Note
    ///
    /// * If `b` is `false` then network will enable.
    /// * If `b` is `true` then network will disable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .network_disabled(true)
    ///     .build();
    /// ```
    pub fn network_disabled(&mut self, b: bool) -> &mut Self {
        self.network_disabled = Some(b);

        self
    }

    /// Set MAC address for this container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .mac_address("1A:2B:3C:4D:5E:6F")
    ///     .build();
    /// ```
    pub fn mac_address<T>(&mut self, mac_address: T) -> &mut Self
        where T: Into<String>
    {
        self.mac_address = Some(mac_address.into());

        self
    }

    /// Append on build script.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .on_build("command-on-build")
    ///     .build();
    /// ```
    pub fn on_build<T>(&mut self, cmd: T) -> &mut Self
        where T: Into<String>
    {
        self.on_build.push(cmd.into());

        self
    }

    /// Append label of container.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .label("example-label-key", "example-label-value")
    ///     .build();
    /// ```
    pub fn label<T, U>(&mut self, k: T, v: U) -> &mut Self
        where
            T: Into<String>,
            U: Into<String>
    {
        self.labels.insert(k.into(), v.into());

        self
    }

    /// Set stop signal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .stop_signal("command")
    ///     .build();
    /// ```
    pub fn stop_signal<T>(&mut self, stop_signal: T) -> &mut Self
        where T: Into<String>
    {
        self.stop_signal = Some(stop_signal.into());

        self
    }

    /// Set stop timeout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .stop_timeout(None)
    ///     .stop_timeout(Some(100))
    ///     .build();
    /// ```
    pub fn stop_timeout(&mut self, time: Option<i32>) -> &mut Self {
        self.stop_timeout = time;

        self
    }

    /// Append shell command.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine")
    ///     .shell("shell-command")
    ///     .shell("shell-command-1")
    ///     .build();
    /// ```
    pub fn shell<T>(&mut self, cmd: T) -> &mut Self
        where T: Into<String>
    {
        self.shell.push(cmd.into());

        self
    }

    pub fn network_config(&mut self, cfg: Option<NetworkSettings>) -> &mut Self {
        self.network_config = cfg;

        self
    }

    /// Build `Config` from `ConfigBuilder`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use docker_client::container::ConfigBuilder;
    /// let builder = ConfigBuilder::with_image("alpine").build();
    /// ```
    pub fn build(&self) -> Config {
        Config {
            hostname: self.hostname.clone(),
            domain_name: self.domain_name.clone(),
            user: self.user.clone(),
            attach_stdin: self.attach_stdin.clone(),
            attach_stdout: self.attach_stdout.clone(),
            attach_stderr: self.attach_stderr.clone(),
            exposed_ports: self.exposed_ports.clone(),
            tty: self.tty.clone(),
            open_stdin: self.open_stdin.clone(),
            stdin_once: self.stdin_once.clone(),
            env: self.env.clone(),
            labels: self.labels.clone(),
            cmd: self.cmd.clone(),
            entry_point: self.entry_point.clone(),
            image: self.image.clone(),
            volumes: self.volumes.clone(),
            health_check: self.health_check.clone(),
            work_dir: self.work_dir.clone(),
            network_disabled: self.network_disabled.clone(),
            network_config: self.network_config.clone(),
            host_config: self.host_config.clone()
        }
    }
}

/// A struct of metadata to create a container.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {

    #[serde(skip_serializing_if = "Option::is_none", rename = "Hostname")]
    hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Domainname")]
    domain_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "User")]
    user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "AttachStdin")]
    attach_stdin: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "AttachStdout")]
    attach_stdout: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "AttachStderr")]
    attach_stderr: Option<bool>,

    #[serde(skip_serializing_if = "HashMap::is_empty", rename = "ExposedPorts", default)]
    exposed_ports: HashMap<String, EmptyObject>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Tty")]
    tty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "OpenStdin")]
    open_stdin: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "StdinOnce")]
    stdin_once: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty", rename = "Env", deserialize_with = "nullable_priority_vec")]
    env: Vec<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", rename = "Labels")]
    labels: HashMap<String, String>,

    #[serde(skip_serializing_if = "Vec::is_empty", rename = "Cmd", deserialize_with = "nullable_priority_vec")]
    cmd: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", rename = "Entrypoint", deserialize_with = "nullable_priority_vec")]
    entry_point: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Image")]
    image: Option<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", rename = "Volumes", deserialize_with = "nullable_priority_hash")]
    volumes: HashMap<String, EmptyObject>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "Healthcheck")]
    health_check: Option<HealthCheck>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "WorkingDir")]
    work_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkDisabled")]
    network_disabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "NetworkConfig")]
    network_config: Option<NetworkSettings>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "HostConfig")]
    host_config: Option<HostConfig>
}

impl Config {

    /// Creates a new default instance of `ConfigBuilder` to construct a `Config`.
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Creates a new `ConfigBuilder` initialized with `image`.
    //
    // This method returns an instance of `ConfigBuilder` which can be used to create a `Config`.
    pub fn with_image<T>(image: T) -> ConfigBuilder
        where T: Into<String>
    {
        let mut builder = ConfigBuilder::default();
        builder.image = Some(image.into());
        builder
    }

}