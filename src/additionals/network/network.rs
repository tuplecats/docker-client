use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::{IPAMConfig};

#[derive(Debug, Default)]
pub struct NetworkBuilder {

    ip_am_config: Option<IPAMConfig>,

    links: Vec<String>,

    aliases: Vec<String>,

    network_id: Option<String>,

    endpoint_id: Option<String>,

    gateway: Option<String>,

    ip_address: Option<String>,

    ip_prefix_len: Option<i32>,

    ipv6_gateway: Option<String>,

    global_ipv6_address: Option<String>,

    global_ipv6_prefix_len: Option<i64>,

    mac_address: Option<String>,

    driver_opts: HashMap<String, String>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {

    #[serde(rename = "IPAMConfig", skip_serializing_if = "Option::is_none")]
    ip_am_config: Option<IPAMConfig>,

    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    links: Option<Vec<String>>,

    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    aliases: Option<Vec<String>>,

    #[serde(rename = "NetworkID")]
    network_id: String,

    #[serde(rename = "EndpointID")]
    endpoint_id: String,

    #[serde(rename = "Gateway")]
    gateway: String,

    #[serde(rename = "IPAddress")]
    ip_address: String,

    #[serde(rename = "IPPrefixLen")]
    ip_prefix_len: i32,

    #[serde(rename = "IPv6Gateway")]
    ipv6_gateway: String,

    #[serde(rename = "GlobalIPv6Address")]
    global_ipv6_address: String,

    #[serde(rename = "GlobalIPv6PrefixLen")]
    global_ipv6_prefix_len: i64,

    #[serde(rename = "MacAddress")]
    mac_address: String,

    #[serde(rename = "DriverOpts", skip_serializing_if = "Option::is_none")]
    driver_opts: Option<HashMap<String, String>>
}

impl NetworkBuilder {

    pub fn new() -> Self {
        NetworkBuilder::default()
    }

    pub fn ip_am_config(&mut self, cfg: Option<IPAMConfig>) -> &mut Self {
        self.ip_am_config = cfg;

        self
    }

    pub fn add_link(&mut self, link: String) -> &mut Self {
        self.links.push(link);

        self
    }

    pub fn add_alias(&mut self, alias: String) -> &mut Self {
        self.aliases.push(alias);

        self
    }

    pub fn network_id(&mut self, id: Option<String>) -> &mut Self {
        self.network_id = id;

        self
    }

    pub fn endpoint_id(&mut self, id: Option<String>) -> &mut Self {
        self.endpoint_id = id;

        self
    }

    pub fn gateway(&mut self, gateway: Option<String>) -> &mut Self {
        self.gateway = gateway;

        self
    }

    pub fn ip_address(&mut self, address: Option<String>) -> &mut Self {
        self.ip_address = address;

        self
    }

    pub fn ip_prefix_len(&mut self, len: Option<i32>) -> &mut Self {
        self.ip_prefix_len = len;

        self
    }

    pub fn ipv6_gateway(&mut self, gateway: Option<String>) -> &mut Self {
        self.ipv6_gateway = gateway;

        self
    }

    pub fn global_ipv6_address(&mut self, address: Option<String>) -> &mut Self {
        self.global_ipv6_address = address;

        self
    }

    pub fn global_ipv6_prefix_len(&mut self, len: Option<i64>) -> &mut Self {
        self.global_ipv6_prefix_len = len;

        self
    }

    pub fn mac_address(&mut self, mac_address: Option<String>) -> &mut Self {
        self.mac_address = mac_address;

        self
    }

    pub fn add_driver_opt(&mut self, key: String, value: String) -> &mut Self {
        self.driver_opts.insert(key, value);

        self
    }

    pub fn build(&self) -> Network {
        Network {
            ip_am_config: self.ip_am_config.clone(),
            links: Some(self.links.clone()),
            aliases: Some(self.aliases.clone()),
            network_id: match self.network_id.clone() { Some(v) => v, None => "".to_string() },
            endpoint_id: match self.endpoint_id.clone() { Some(v) => v, None => "".to_string() },
            gateway: match self.gateway.clone() { Some(v) => v, None => "".to_string() },
            ip_address: match self.ip_address.clone() { Some(v) => v, None => "".to_string() },
            ip_prefix_len: self.ip_prefix_len.unwrap_or(0),
            ipv6_gateway: match self.ipv6_gateway.clone() { Some(v) => v, None => "".to_string() },
            global_ipv6_address: match self.global_ipv6_address.clone() { Some(v) => v, None => "".to_string() },
            global_ipv6_prefix_len: self.global_ipv6_prefix_len.unwrap_or(0),
            mac_address: match self.mac_address.clone() { Some(v) => v, None => "".to_string() },
            driver_opts: Some(self.driver_opts.clone())
        }
    }

}