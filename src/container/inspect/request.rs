
pub struct Inspect {

    id: String,

    size: Option<bool>

}

impl Inspect {

    pub fn container(_id: String) -> Self {
        Inspect {
            id: _id,
            size: None
        }
    }

    pub fn size(&mut self, v: bool) {
        self.size = Some(v);
    }

    pub fn get_path(&self) -> String {

        format!("/containers/{}/json?size={}", self.id.clone(), self.size.unwrap_or(false).to_string())

    }

}