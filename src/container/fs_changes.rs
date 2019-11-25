use serde::Deserialize;

/// `FSChanges` struct.
#[derive(Deserialize, Debug)]
pub struct FSChanges {
    #[serde(rename(deserialize = "Path"))]
    path: String,

    #[serde(rename(deserialize = "Kind"))]
    kind: i32,
}

impl FSChanges {

    /// Return path
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /// Return kind
    pub fn kind(&self) -> i32 {
        self.kind
    }

}