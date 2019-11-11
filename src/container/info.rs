//!
//! Info container types.
//!

use serde::{Deserialize};
use crate::Config;

/*#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    #[serde(rename(deserialize = "IP"))]
    ip: Option<String>,

    #[serde(rename(deserialize = "PrivatePort"))]
    private_port: Option<i32>,

    #[serde(rename(deserialize = "PublicPort"))]
    public_port: Option<i32>,

    #[serde(rename(deserialize = "Type"))]
    protocol: Option<String>,
}*/

/// Mounts information
#[derive(Deserialize, Debug)]
pub struct Mounts {

    #[serde(rename(deserialize = "Type"))]
    mount_type: Option<String>,

    #[serde(rename(deserialize = "Name"))]
    name: Option<String>,

    #[serde(rename(deserialize = "Source"))]
    source: String,

    #[serde(rename(deserialize = "Destination"))]
    destination: String,

    #[serde(rename(deserialize = "Driver"))]
    driver: Option<String>,

    #[serde(rename(deserialize = "Mode"))]
    mode: String,

    #[serde(rename(deserialize = "RW"))]
    rw: bool,

    #[serde(rename(deserialize = "Propagation"))]
    propagation: String
}

/*
#[derive(Deserialize, Debug)]
struct Network {
    #[serde(rename(deserialize = "IPAMConfig"))]
    ip_am_config: Option<String>,

    #[serde(rename(deserialize = "Links"))]
    links: Option<String>,

    #[serde(rename(deserialize = "Aliases"))]
    aliases: Option<String>,

    #[serde(rename(deserialize = "NetworkID"))]
    network_id: Option<String>,

    #[serde(rename(deserialize = "EndpointID"))]
    endpoint_id: Option<String>,

    #[serde(rename(deserialize = "Getaway"))]
    getaway: Option<String>,

    #[serde(rename(deserialize = "IPAddress"))]
    ip_address: Option<String>,

    #[serde(rename(deserialize = "IPPrefixLen"))]
    ip_prefix_len: Option<i32>,

    #[serde(rename(deserialize = "IPv6Gateway"))]
    ip_v6_getaway: Option<String>,

    #[serde(rename(deserialize = "GlobalIPv6Address"))]
    ip_v6_address: Option<String>,

    #[serde(rename(deserialize = "GlobalIPv6PrefixLen"))]
    ip_v6_prefix_len: Option<i32>,

    #[serde(rename(deserialize = "MacAddress"))]
    mac_address: Option<String>,

}
*/

/// State information
#[derive(Deserialize, Debug)]
pub struct State {
    #[serde(rename(deserialize = "Status"))]
    status: String,

    #[serde(rename(deserialize = "Running"))]
    running: bool,

    #[serde(rename(deserialize = "Paused"))]
    paused: bool,

    #[serde(rename(deserialize = "Restarting"))]
    restarting: bool,

    #[serde(rename(deserialize = "OOMKilled"))]
    oom_killed: bool,

    #[serde(rename(deserialize = "Dead"))]
    dead: bool,

    #[serde(rename(deserialize = "Pid"))]
    pid: i32,

    #[serde(rename(deserialize = "ExitCode"))]
    exit_code: i32,

    #[serde(rename(deserialize = "Error"))]
    error: String,

    #[serde(rename(deserialize = "StartedAt"))]
    started: String,

    #[serde(rename(deserialize = "FinishedAt"))]
    finished: String,

}

/// Container information
#[derive(Deserialize, Debug)]
pub struct ContainerInfo {
    #[serde(rename(deserialize = "Id"))]
    id: String,

    #[serde(rename(deserialize = "Created"))]
    created: String,

    #[serde(rename(deserialize = "Path"))]
    path: String,

    #[serde(rename(deserialize = "Args"))]
    args: Vec<String>,

    #[serde(rename(deserialize = "State"))]
    state: State,

    #[serde(rename(deserialize = "Image"))]
    image: String,

    #[serde(rename(deserialize = "ResolvConfPath"))]
    resolve_conf_path: String,

    #[serde(rename(deserialize = "HostnamePath"))]
    hostname_path: String,

    #[serde(rename(deserialize = "HostsPath"))]
    hosts_path: String,

    #[serde(rename(deserialize = "LogPath"))]
    log_path: String,

    #[serde(rename(deserialize = "Name"))]
    name: String,

    #[serde(rename(deserialize = "RestartCount"))]
    restart_count: i32,

    #[serde(rename(deserialize = "Driver"))]
    driver: String,

    #[serde(rename(deserialize = "MountLabel"))]
    mount_label: String,

    #[serde(rename(deserialize = "ProcessLabel"))]
    process_label: String,

    #[serde(rename(deserialize = "AppArmorProfile"))]
    app_armor_profile: String,

   // #[serde(rename(deserialize = "ExecIDs"))]
   // exec_ids: Option<Vec<String>>,

    #[serde(rename(deserialize = "SizeRW"))]
    size_rw: Option<i64>,

    #[serde(rename(deserialize = "SizeRootFs"))]
    size_root_fs: Option<i64>,

    #[serde(rename(deserialize = "Config"))]
    config: Config,
}

