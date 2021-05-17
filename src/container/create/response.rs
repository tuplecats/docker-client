use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct CreatedContainer {

    #[serde(rename(deserialize = "Id"))]
    id: String,

    #[serde(rename(deserialize = "Warnings"))]
    warnings: Vec<String>,
}

impl CreatedContainer {

    pub fn id(&self) -> &String {
        &self.id
    }

}