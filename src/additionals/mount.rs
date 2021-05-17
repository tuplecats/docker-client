use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BindOptions {

    #[serde(rename(deserialize = "Propagation"))]
    propagation: String,

    #[serde(rename(deserialize = "NonRecursive"))]
    non_recursive: bool

}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverConfig {

    #[serde(rename(deserialize = "Name"))]
    name: String,

    #[serde(rename(deserialize = "Options"))]
    options: HashMap<String, String>

}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeOptions {

    #[serde(rename(deserialize = "NoCopy"))]
    no_copy: bool,

    #[serde(rename(deserialize = "Labels"))]
    labels: HashMap<String, String>,

    #[serde(rename(deserialize = "DriverConfig"))]
    driver_config: DriverConfig

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TmpfsOptions {

    #[serde(rename(deserialize = "SizeBytes"))]
    size_bytes: i64,

    #[serde(rename(deserialize = "Mode"))]
    mode: i32

}

fn default_read_only() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mount {

    #[serde(rename(deserialize = "Target"))]
    target: Option<String>,

    #[serde(rename(deserialize = "Source"))]
    source: String,

    #[serde(rename(deserialize = "Type"))]
    mount_type: String,

    #[serde(rename(deserialize = "ReadOnly"), default = "default_read_only")]
    read_only: bool,

    #[serde(rename(deserialize = "Consistency"), default = "String::new")]
    consistency: String,

    #[serde(rename(deserialize = "BindOptions"))]
    bind_options: Option<BindOptions>,

    #[serde(rename(deserialize = "VolumeOptions"))]
    volume_options: Option<VolumeOptions>,

    #[serde(rename(deserialize = "TmpfsOptions"), skip_serializing_if = "Option::is_none")]
    tmpfs_options: Option<TmpfsOptions>

}