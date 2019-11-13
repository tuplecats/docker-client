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

    /// TODO doc
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /// TODO doc
    pub fn kind(&self) -> i32 {
        self.kind
    }

}