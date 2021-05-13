
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostConfigBuilder {

    cpu_shares: Option<i32>,

    memory: i64,

    c_group_parent: Option<String>,

    blkio_weight: Option<i32>,

}

impl Default for HostConfigBuilder {

    fn default() -> Self {
        HostConfigBuilder {
            memory: 0,
            ..Default::default()
        }
    }

}

impl HostConfigBuilder {

    pub fn new() -> Self {
        HostConfigBuilder::default()
    }

    pub fn cpu_shares(&mut self, cpu: i32) -> &mut Self {
        self.cpu_shares = Some(cpu);

        self
    }

    pub fn memory(&mut self, mem: i64) -> &mut Self {
        self.memory = mem;

        self
    }

    pub fn c_group_parent<T>(&mut self, cgp: T) -> &mut Self
        where T: Into<String>
    {
        self.c_group_parent = Some(cgp.into());

        self
    }

    pub fn blkio_weight(&mut self, weight: Option<i32>) -> &mut Self {
        self.blkio_weight = weight;

        self
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostConfig {

    cpu_shares: Option<i32>

}