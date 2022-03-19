
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::additionals::network::NetworkSettings;
use crate::additionals::mount::Mount;


// response
#[derive(Debug, Serialize, Deserialize)]
struct PortInfo {

    #[serde(rename(deserialize = "IP"))]
    ip: String,

    #[serde(rename(deserialize = "PrivatePort"))]
    private_port: u16,

    #[serde(rename(deserialize = "PublicPort"))]
    public_port: u16,

    #[serde(rename(deserialize = "Type"))]
    port_type: String

}

#[derive(Debug, Serialize, Deserialize)]
struct HostConfig {

    #[serde(rename(deserialize = "NetworkMode"))]
    network_mode: String

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortContainerInfo {

    #[serde(rename(deserialize = "Id"))]
    id: String,

    #[serde(rename(deserialize = "Names"))]
    names: Vec<String>,

    #[serde(rename(deserialize = "Image"))]
    image: String,

    #[serde(rename(deserialize = "ImageID"))]
    image_id: String,

    #[serde(rename(deserialize = "Command"))]
    command: String,

    #[serde(rename(deserialize = "Created"))]
    created: u64,

    #[serde(rename(deserialize = "Ports"))]
    ports: Vec<PortInfo>,

    #[serde(rename(deserialize = "SizeRW"), skip_serializing_if = "Option::is_none")]
    size_rw: Option<i64>,

    #[serde(rename(deserialize = "SizeRootFS"), skip_serializing_if = "Option::is_none")]
    size_root_fs: Option<i64>,

    #[serde(rename(deserialize = "Labels"))]
    labels: HashMap<String, String>,

    #[serde(rename(deserialize = "State"))]
    state: String,

    #[serde(rename(deserialize = "Status"))]
    status: String,

    #[serde(rename(deserialize = "HostConfig"))]
    host_config: HostConfig,

    #[serde(rename(deserialize = "NetworkSettings"))]
    network_settings: NetworkSettings,

    #[serde(rename(deserialize = "Mounts"))]
    mounts: Vec<Mount>,
}

impl ShortContainerInfo {

    pub fn image(&self) -> &str {
        &self.image
    }

    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }

    pub fn id(&self) -> &str {
        &self.id
    }

}