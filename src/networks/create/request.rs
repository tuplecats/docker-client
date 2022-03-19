
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct IPAMBuilder {

    driver: String,

    config: Vec<HashMap<String, String>>,

    options: HashMap<String, String>
}

impl Default for IPAMBuilder {
    fn default() -> Self {
        IPAMBuilder {
            driver: String::from("default"),
            config: vec![],
            options: Default::default()
        }
    }
}

impl IPAMBuilder {
    
    pub fn new() -> Self {
        IPAMBuilder::default()
    }

    pub fn driver<T>(mut self, v: T) -> Self
        where T: Into<String>
    {
        self.driver = v.into();

        self
    }

    pub fn add_config(mut self, map: HashMap<String, String>) -> Self {
        self.config.push(map);

        self
    }

    pub fn add_option<T, U>(mut self, k: T, v: U) -> Self
        where
            T: Into<String>,
            U: Into<String>
    {
        self.options.insert(k.into(), v.into());

        self
    }

    pub fn build(self) -> IPAM {
        IPAM {
            driver: self.driver,
            config: self.config,
            options: self.options
        }
    }
    
}

pub struct RequestBuilder {
    
    name: String,

    check_duplicate: bool,

    driver: String,

    internal: bool,

    attachable: bool,

    ingress: bool,

    ip_am: IPAM,

    enable_ipv6: bool,

    options: HashMap<String, String>,

    labels: HashMap<String, String>
}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            name: String::new(),
            check_duplicate: true,
            driver: String::from("bridge"),
            internal: false,
            attachable: false,
            ingress: false,
            ip_am: IPAMBuilder::new().build(),
            enable_ipv6: false,
            options: Default::default(),
            labels: Default::default()
        }
    }
}

impl RequestBuilder {
    
    pub fn with_name<T>(name: T) -> Self
        where T: Into<String>
    {
        let mut builder = RequestBuilder::default();
        builder.name = name.into();
        builder
    }

    pub fn build(&self) -> Request {
        Request {
            name: self.name.clone(),
            check_duplicate: self.check_duplicate,
            driver: self.driver.clone(),
            internal: self.internal,
            attachable: self.attachable,
            ingress: self.ingress,
            ip_am: self.ip_am.clone(),
            enable_ipv6: self.enable_ipv6,
            options: self.options.clone(),
            labels: self.labels.clone()
        }
    }
    
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IPAM {

    #[serde(rename = "String")]
    driver: String,

    #[serde(rename = "Config")]
    config: Vec<HashMap<String, String>>,

    #[serde(rename = "Options")]
    options: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct Request {

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "CheckDuplicate")]
    check_duplicate: bool,

    #[serde(rename = "Driver")]
    driver: String,

    #[serde(rename = "Internal")]
    internal: bool,

    #[serde(rename = "Attachable")]
    attachable: bool,

    #[serde(rename = "Ingress")]
    ingress: bool,

    #[serde(rename = "IPAM")]
    ip_am: IPAM,

    #[serde(rename = "EnableIPv6")]
    enable_ipv6: bool,

    #[serde(rename = "Options")]
    options: HashMap<String, String>,

    #[serde(rename = "Labels")]
    labels: HashMap<String, String>
}

impl Request {

    pub fn get_path(&self) -> String {
        String::from("/networks/create")
    }

}