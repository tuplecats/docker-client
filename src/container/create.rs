use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::http::{Request, URI};
use crate::container::ToRequest;
use crate::container::health_check::HealthCheck;

#[derive(Debug, Serialize, Clone)]
struct EmptyObject;

#[derive(Debug, Default)]
pub struct CreatorBuilder {
    name: Option<String>,
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
    //host_config: Option<HostConfig>,
    //network_config: Option<NetworkConfig>,
}

macro_rules! try_into_opt {
    ($field:ident) => {
        match $field {
            Some(n) => Some(n.into()),
            None => None,
        }
    }
}

impl CreatorBuilder {
    pub fn new() -> Self {
        CreatorBuilder::default()
    }

    pub fn from_image<T>(image: T) -> Self
        where T: Into<String> {

        let mut builder = CreatorBuilder::new();
        builder.image = Some(image.into());
        builder
    }

    pub fn name<T>(&mut self, name: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.name = try_into_opt!(name);

        self
    }

    pub fn hostname<T>(&mut self, name: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.hostname = try_into_opt!(name);

        self
    }

    pub fn domain_name<T>(&mut self, name: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.domain_name = try_into_opt!(name);

        self
    }

    pub fn user<T>(&mut self, name: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.user = try_into_opt!(name);

        self
    }

    pub fn attach_stdin(&mut self, b: Option<bool>) -> &mut Self {
        self.attach_stdin = b;

        self
    }

    pub fn attach_stdout(&mut self, b: Option<bool>) -> &mut Self {
        self.attach_stdout = b;

        self
    }

    pub fn attach_stderr(&mut self, b: Option<bool>) -> &mut Self {
        self.attach_stderr = b;

        self
    }

    pub fn expose_port<T>(&mut self, port: T) -> &mut Self
        where T: Into<String>
    {
        self.exposed_ports.insert(port.into(), EmptyObject{});

        self
    }

    pub fn tty(&mut self, b: Option<bool>) -> &mut Self {
        self.tty = b;

        self
    }

    pub fn open_stdin(&mut self, b: Option<bool>) -> &mut Self {
        self.open_stdin = b;

        self
    }

    pub fn stdin_once(&mut self, b: Option<bool>) -> &mut Self {
        self.stdin_once = b;

        self
    }

    pub fn env<T>(&mut self, env: T) -> &mut Self
        where T: Into<String>
    {
        self.env.push(env.into());

        self
    }

    pub fn cmd(&mut self, cmd: String) -> &mut Self {
        self.cmd.push(cmd);

        self
    }

    pub fn health_check(&mut self, health_check: Option<HealthCheck>) -> &mut Self {
        self.health_check = health_check;

        self
    }

    pub fn args_escaped(&mut self, b: Option<bool>) -> &mut Self {
        self.args_escaped = b;

        self
    }

    pub fn image<T>(&mut self, image: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.image = try_into_opt!(image);

        self
    }

    pub fn volume(&mut self, volume: String) -> &mut Self {
        self.volumes.insert(volume, EmptyObject{});

        self
    }

    pub fn work_dir<T>(&mut self, work_dir: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.work_dir = try_into_opt!(work_dir);

        self
    }

    pub fn entry_point<T>(&mut self, entry_point: T) -> &mut Self
        where T: Into<String>
    {
        self.entry_point.push(entry_point.into());

        self
    }

    pub fn network_disabled(&mut self, b: Option<bool>) -> &mut Self {
        self.network_disabled = b;

        self
    }

    pub fn mac_address<T>(&mut self, mac_address: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.mac_address = try_into_opt!(mac_address);

        self
    }

    pub fn on_build(&mut self, cmd: String) -> &mut Self {
        self.on_build.push(cmd);

        self
    }

    pub fn label(&mut self, k: String, v: String) -> &mut Self {
        self.labels.insert(k, v);

        self
    }

    pub fn stop_signal<T>(&mut self, stop_signal: Option<T>) -> &mut Self
        where T: Into<String>
    {
        self.stop_signal = try_into_opt!(stop_signal);

        self
    }

    pub fn stop_timeout(&mut self, time: Option<i32>) -> &mut Self {
        self.stop_timeout = time;

        self
    }

    pub fn shell(&mut self, cmd: String) -> &mut Self {
        self.shell.push(cmd);

        self
    }

    pub fn build(&self) -> Creator {
        Creator {
            name: self.name.clone(),
            hostname: self.hostname.clone(),
            domain_name: self.domain_name.clone(),
            user: self.user.clone(),
            attach_stdin: self.attach_stdin.clone(),
            attach_stdout: self.attach_stdout.clone(),
            attach_stderr: self.attach_stderr.clone(),
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
            network_disabled: self.network_disabled.clone()
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Creator {

    #[serde(skip_serializing)]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Hostname"))]
    hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Domainname"))]
    domain_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "User"))]
    user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "AttachStdin"))]
    attach_stdin: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "AttachStdout"))]
    attach_stdout: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "AttachStderr"))]
    attach_stderr: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Tty"))]
    tty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "OpenStdin"))]
    open_stdin: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "StdinOnce"))]
    stdin_once: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty", rename(serialize = "Env"))]
    env: Vec<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", rename(serialize = "Labels"))]
    labels: HashMap<String, String>,

    #[serde(skip_serializing_if = "Vec::is_empty", rename(serialize = "Cmd"))]
    cmd: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", rename(serialize = "Entrypoint"))]
    entry_point: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Image"))]
    image: Option<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", rename(serialize = "Volumes"))]
    volumes: HashMap<String, EmptyObject>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Healthcheck"))]
    health_check: Option<HealthCheck>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "WorkingDir"))]
    work_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "NetworkDisabled"))]
    network_disabled: Option<bool>,

}

impl Creator {

    pub fn new() -> CreatorBuilder {
        CreatorBuilder::default()
    }

    pub fn from<T>(image: T) -> CreatorBuilder
        where T: Into<String>
    {
        let mut builder = CreatorBuilder::default();
        builder.image = Some(image.into());
        builder
    }

}

impl ToRequest for Creator {
    fn to_request(&self) -> Request {
        let mut url = URI::with_path("/containers/create");

        if self.name.is_some() {
            url.parameter("name", self.name.clone().unwrap().as_str());
        }

        Request::post()
            .url(url.build())
            .content(serde_json::to_string(self).unwrap())
            .build()
    }
}

#[derive(Deserialize, Debug)]
pub struct CreatedContainer {

    #[serde(rename(deserialize = "Id"))]
    id: String,

    #[serde(rename(deserialize = "Warnings"))]
    warnings: Vec<String>,
}

impl CreatedContainer {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }
}