
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedNetwork {

    #[serde(rename = "Id")]
    id: String,

    #[serde(rename = "Warning")]
    warning: String,
}

impl CreatedNetwork {

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn warning(&self) -> String {
        self.warning.clone()
    }

}