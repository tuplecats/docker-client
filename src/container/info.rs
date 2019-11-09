use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    #[serde(rename(deserialize = "IP"))]
    ip: Option<String>,

    #[serde(rename(deserialize = "PrivatePort"))]
    private_port: Option<i32>,

    #[serde(rename(deserialize = "PublicPort"))]
    public_port: Option<i32>,

    #[serde(rename(deserialize = "Type"))]
    protocol: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Mounts {
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

#[derive(Deserialize, Debug)]
pub struct ContainerInfo {
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
    created: i64,

    #[serde(rename(deserialize = "State"))]
    state: String,

    #[serde(rename(deserialize = "Status"))]
    status: String,

    #[serde(rename(deserialize = "Ports"))]
    ports: Vec<Port>,

    #[serde(rename(deserialize = "Labels"))]
    labels: HashMap<String, String>,

    #[serde(rename(deserialize = "SizeRW"))]
    size_rw: Option<i32>,

    #[serde(rename(deserialize = "SizeRootFs"))]
    size_root_fs: Option<i32>,

    #[serde(rename(deserialize = "HostConfig"))]
    host_cfg: Option<HashMap<String, String>>,

    #[serde(rename(deserialize = "Mounts"))]
    mounts: Option<Vec<Mounts>>,

    #[serde(rename(deserialize = "NetworkSettings"))]
    network_settings: HashMap<String, HashMap<String, Network>>,
}

