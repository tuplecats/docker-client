//request
#[derive(Default)]
pub struct ContainersListBuilder {

    all: Option<bool>,

    limit: Option<i32>,

    size: Option<bool>

}

impl ContainersListBuilder {

    pub fn new() -> Self {
        ContainersListBuilder::default()
    }

    pub fn all(&mut self, v: bool) -> &mut Self {
        self.all = Some(v);

        self
    }

    pub fn limit(&mut self, v: i32) -> &mut Self {
        self.limit = Some(v);

        self
    }

    pub fn size(&mut self, v: bool) -> &mut Self {
        self.size = Some(v);

        self
    }

    pub fn build(&self) -> ContainersList {
        ContainersList {
            all: self.all.clone(),
            limit: self.limit.clone(),
            size: self.size.clone()
        }
    }

}

#[derive(Debug, Clone)]
pub struct ContainersList {

    all: Option<bool>,

    limit: Option<i32>,

    size: Option<bool>

}

impl ContainersList {

    pub fn new() -> ContainersListBuilder {
        ContainersListBuilder::default()
    }

    pub fn get_path(&self) -> String {
        let mut path = "/containers/json?".to_string();

        if self.all.is_some() {
            path.push_str(format!("all={}&", self.all.unwrap()).as_str());
        }
        if self.limit.is_some() {
            path.push_str(format!("limit={}&", self.limit.unwrap()).as_str());
        }
        if self.size.is_some() {
            path.push_str(format!("size={}&", self.size.unwrap()).as_str());
        }

        path.pop();
        path
    }

}