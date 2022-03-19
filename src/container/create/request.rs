
use super::Config;

#[derive(Default, Clone)]
pub struct CreateBuilder {

    name: String,

    config: Config,

}

impl CreateBuilder {

    pub fn with_config(cfg: Config) -> Self {
        CreateBuilder {
            name: String::new(),
            config: cfg
        }
    }

    pub fn name<T>(mut self, v: T) -> Self
        where T: Into<String>
    {
        self.name = v.into();

        self
    }

    pub fn config(mut self, cfg: Config) -> Self {
        self.config = cfg;

        self
    }

    pub fn build(self) -> Create {
        Create {
            name: self.name,
            config: self.config
        }
    }
}

pub struct Create {

    name: String,

    config: Config

}

impl Create {

    pub fn new() -> CreateBuilder {
        CreateBuilder::default()
    }

    pub fn get_path(&self) -> String {
        let mut path = format!("/containers/create?");

        if !self.name.is_empty() {
            path.push_str(format!("name={}&", self.name).as_str());
        }

        path.pop();
        path
    }

    pub fn body(&self) -> String {
        serde_json::to_string(&self.config).unwrap()
    }
}