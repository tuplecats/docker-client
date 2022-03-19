use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default)]
pub struct IPAMConfigBuilder {

    ipv4_address: String,

    ipv6_address: String,

    link_local_ips: Vec<String>

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IPAMConfig {

    #[serde(rename = "IPV4Address")]
    ipv4_address: String,

    #[serde(rename = "IPV6Address")]
    ipv6_address: String,

    #[serde(rename = "LinkLocalIPs")]
    link_local_ips: Vec<String>

}

impl IPAMConfigBuilder {

    pub fn new() -> Self {
        IPAMConfigBuilder::default()
    }

    pub fn ipv4_address(mut self, address: String) -> Self {
        self.ipv4_address = address.clone();

        self
    }

    pub fn ipv6_address(mut self, address: String) -> Self {
        self.ipv6_address = address.clone();

        self
    }

    pub fn add_local_ip(mut self, address: String) -> Self {
        self.link_local_ips.push(address.clone());

        self
    }

    pub fn build(self) -> IPAMConfig {
        IPAMConfig {
            ipv4_address: self.ipv4_address,
            ipv6_address: self.ipv6_address,
            link_local_ips: self.link_local_ips
        }
    }

}