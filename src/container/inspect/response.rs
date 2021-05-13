use std::collections::HashMap;
use crate::Config;
use crate::additionals::network::NetworkSettings;
use serde::Deserialize;
use crate::additionals::serde_helpers::*;

#[derive(Debug, Deserialize)]
pub struct HealthCheckResult {

    #[serde(rename = "Start")]
    start: String,

    #[serde(rename = "End")]
    end: String,

    #[serde(rename = "ExitCode")]
    exit_code: i32,

    #[serde(rename = "Output")]
    output: String

}

#[derive(Debug, Deserialize)]
pub struct Health {

    #[serde(rename = "Status")]
    status: String,

    #[serde(rename = "FailingStreak")]
    failing_streak: i32,

    #[serde(rename = "Log")]
    log: Vec<HealthCheckResult>

}

#[derive(Debug, Deserialize)]
pub struct State {

    #[serde(rename = "Status")]
    status: String,

    #[serde(rename = "Running")]
    running: bool,

    #[serde(rename = "Paused")]
    paused: bool,

    #[serde(rename = "Restarting")]
    restarting: bool,

    #[serde(rename = "OOMKilled")]
    oom_killed: bool,

    #[serde(rename = "Dead")]
    dead: bool,

    #[serde(rename = "Pid")]
    pid: i32,

    #[serde(rename = "ExitCode")]
    exit_code: i32,

    #[serde(rename = "StartedAt")]
    started_at: String,

    #[serde(rename = "FinishedAt")]
    finished_at: String,

    #[serde(rename = "Health")]
    health: Option<Health>,

}

#[derive(Debug, Deserialize)]
pub struct GraphDriverData {

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Data")]
    data: HashMap<String, String>
}

#[derive(Debug, Deserialize)]
pub struct MountPoint {

    #[serde(rename = "Type")]
    mount_type: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Source")]
    source: String,

    #[serde(rename = "Destination")]
    destination: String,

    #[serde(rename = "Driver")]
    driver: String,

    #[serde(rename = "Mode")]
    mode: String,

    #[serde(rename = "RW")]
    rw: bool,

    #[serde(rename = "Propagation")]
    propagation: String

}

#[derive(Debug, Deserialize)]
pub struct ContainerInfo {

    #[serde(rename = "Id")]
    id: String,

    #[serde(rename = "Created")]
    created: String,

    #[serde(rename = "Path")]
    path: String,

    #[serde(rename = "Args")]
    args: Vec<String>,

    #[serde(rename = "State")]
    state: State,

    #[serde(rename = "Image")]
    image: String,

    #[serde(rename = "ResolvConfPath")]
    resolv_conf_path: String,

    #[serde(rename = "HostnamePath")]
    hostname_path: String,

    #[serde(rename = "HostsPath")]
    hosts_path: String,

    #[serde(rename = "LogPath")]
    log_path: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "RestartCount")]
    restart_count: i32,

    #[serde(rename = "Driver")]
    driver: String,

    #[serde(rename = "Platform")]
    platform: String,

    #[serde(rename = "MountLabel")]
    mount_label: String,

    #[serde(rename = "ProcessLabel")]
    process_label: String,

    #[serde(rename = "AppArmorProfile")]
    app_armor_profile: String,

    #[serde(rename = "ExecIDs", deserialize_with = "nullable_priority_vec")]
    exec_ids: Vec<String>,

    #[serde(rename = "GraphDriver")]
    graph_driver: GraphDriverData,

    #[serde(rename = "SizeRW")]
    size_rw: Option<i64>,

    #[serde(rename = "SizeRootFs")]
    size_root_fs: Option<i64>,

    #[serde(rename = "Mounts")]
    mounts: Vec<MountPoint>,

    #[serde(rename = "Config")]
    config: Config,

    #[serde(rename = "NetworkSettings")]
    network_settings: NetworkSettings
}
