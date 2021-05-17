
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortBinding {

    #[serde(rename = "HostIP", skip_serializing_if = "Option::is_none")]
    host_ip: Option<String>,

    #[serde(rename = "HostPort")]
    host_port: String
}

#[derive(Debug, Default)]
pub struct HostConfigBuilder {

    binds: Vec<String>,

    port_bindings: HashMap<String, Vec<PortBinding>>,

    sysctls: HashMap<String, String>,

    auto_remove: Option<bool>,

}

impl HostConfigBuilder {

    pub fn new() -> Self {
        HostConfigBuilder::default()
    }

    pub fn bind_port(&mut self, container_port: String, host_ip: Option<String>, host_port: String) -> &mut Self {
        match self.port_bindings.contains_key(&container_port) {
            true => { self.port_bindings.get_mut(&container_port).unwrap().push(PortBinding {host_ip, host_port}); }
            false => { self.port_bindings.insert(container_port, vec![PortBinding { host_ip, host_port }]); }
        }

        self
    }

    pub fn mount(&mut self, host_path: String, container_path: String, read_only: bool) -> &mut Self {
        let method = match read_only {
            true => "ro",
            false => "rw"
        };

        self.binds.push(format!("{}:{}:{}", host_path, container_path, method));

        self
    }

    pub fn sysctl(&mut self, k: String, value: String) -> &mut Self {
        self.sysctls.insert(k, value);

        self
    }

    pub fn auto_remove(&mut self, b: bool) -> &mut Self {
        self.auto_remove = Some(b);

        self
    }

    pub fn build(&self) -> HostConfig {
        HostConfig {
            binds: self.binds.clone(),
            port_bindings: self.port_bindings.clone(),
            sysctls: self.sysctls.clone(),
            auto_remove: self.auto_remove.unwrap_or(false)
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostConfig {

    #[serde(rename = "Binds", skip_serializing_if = "Vec::is_empty")]
    binds: Vec<String>,

    #[serde(rename = "PortBindings", skip_serializing_if = "HashMap::is_empty")]
    port_bindings: HashMap<String, Vec<PortBinding>>,

    #[serde(rename = "Sysctls", skip_serializing_if = "HashMap::is_empty")]
    sysctls: HashMap<String, String>,

    #[serde(rename = "AutoRemove")]
    auto_remove: bool

}