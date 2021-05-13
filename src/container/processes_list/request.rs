
#[derive(Debug)]
pub struct ProcessesList {

    id: String,

    ps_args: String

}

impl ProcessesList {

    pub fn container(name: String) -> Self {
        ProcessesList {
            id: name,
            ps_args: String::new()
        }
    }

    pub fn ps_args(&mut self, args: String) {
        self.ps_args = args;
    }

    pub fn get_path(&self) -> String {
        let mut path = format!("/containers/{}/top", self.id);

        if !self.ps_args.is_empty() {
            path.push_str(format!("?ps_args={}", self.ps_args).as_str());
        }

        path
    }

}