use std::collections::HashMap;
use super::Network;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug)]
pub struct NetworkSettingsBuilder {

    networks: HashMap<String, Network>

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkSettings {

    #[serde(rename = "Networks")]
    networks: HashMap<String, Network>

}

impl NetworkSettingsBuilder {

    pub fn new() -> Self {
        NetworkSettingsBuilder::default()
    }

    pub fn add_config(mut self, name: String, config: &Network) -> Self {
        self.networks.insert(name, config.clone());

        self
    }

    pub fn build(self) -> NetworkSettings {
        NetworkSettings {
            networks: self.networks
        }
    }

}