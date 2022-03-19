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
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    /// Return kind
    pub fn kind(&self) -> i32 {
        self.kind
    }

}