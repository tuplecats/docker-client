use std::collections::HashMap;
use serde::Serialize;

/// TODO doc
#[derive(Debug, Default)]
pub struct VolumeCreatorBuilder {
    name: String,
    driver: String,
    driver_opts: HashMap<String, String>,
    labels: HashMap<String, String>,
}

/// TODO doc
#[derive(Serialize, Debug)]
pub struct VolumeCreator {
    #[serde(skip_serializing_if = "String::is_empty", rename = "Name")]
    name: String,

    #[serde(skip_serializing_if = "String::is_empty", rename = "Driver")]
    driver: String,

    #[serde(skip_serializing_if = "HashMap::is_empty", rename = "DriverOpts")]
    driver_opts: HashMap<String, String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", rename = "Labels")]
    labels: HashMap<String, String>,
}

impl VolumeCreator {

    /// TODO doc
    pub fn builder() -> VolumeCreatorBuilder {
        VolumeCreatorBuilder::default()
    }

}

impl VolumeCreatorBuilder {

    /// TODO doc
    pub fn new() -> Self {
        VolumeCreatorBuilder::default()
    }

    /// TODO doc
    pub fn name<T>(&mut self, name: T) -> &mut Self
        where T: Into<String>
    {
        self.name = name.into();

        self
    }

    /// TODO doc
    pub fn driver<T>(&mut self, driver: T) -> &mut Self
        where T: Into<String>
    {
        self.driver = driver.into();

        self
    }

    /// TODO doc
    pub fn driver_opt<T, U>(&mut self, key: T, value: U) -> &mut Self
        where
            T: Into<String>,
            U: Into<String>
    {
        self.driver_opts.insert(key.into(), value.into());

        self
    }

    /// TODO doc
    pub fn label<T, U>(&mut self, key: T, value: U) -> &mut Self
        where
            T: Into<String>,
            U: Into<String>
    {
        self.labels.insert(key.into(), value.into());

        self
    }

    /// TODO doc
    pub fn build(&self) -> VolumeCreator {
        VolumeCreator {
            name: self.name.clone(),
            driver: self.driver.clone(),
            driver_opts: self.driver_opts.clone(),
            labels: self.labels.clone(),
        }
    }
}