use std::collections::HashMap;
use serde::{Deserialize, Deserializer};

/// Short image info
#[derive(Deserialize, Debug)]
pub struct ShortImageInfo {

    #[serde(rename = "Id")]
    id: String,

    #[serde(rename = "ParentId")]
    parent_id: String,

    #[serde(rename = "RepoTags")]
    repo_tags: Vec<String>,

    #[serde(rename = "RepoDigests", deserialize_with = "nullable_vector")]
    repo_digests: Vec<String>,

    #[serde(rename = "Created")]
    created: i64,

    #[serde(rename = "Size")]
    size: i64,

    #[serde(rename = "SharedSize")]
    shared_size: i64,

    #[serde(rename = "VirtualSize")]
    virtual_size: i64,

    #[serde(rename = "Labels", deserialize_with = "nullable_priority_hash")]
    labels: HashMap<String, String>,

    #[serde(rename = "Containers")]
    containers: i64,

}

impl ShortImageInfo {

    /// Return id of image
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return parent id
    pub fn parent_id(&self) -> &str {
        &self.parent_id
    }

    /// Return repo tags
    pub fn repo_tags(&self) -> &Vec<String> {
        &self.repo_tags
    }

    /// Return repo digests
    pub fn repo_digests(&self) -> &Vec<String> {
        &self.repo_digests
    }

    /// Return created
    pub fn created(&self) -> i64 {
        self.created
    }

    /// Return size of image
    pub fn size(&self) -> i64 {
        self.size
    }

    /// Return shared size
    pub fn shared_size(&self) -> i64 {
        self.shared_size
    }

    /// Return virtual size
    pub fn virtual_size(&self) -> i64 {
        self.virtual_size
    }

    /// Return labels
    pub fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }

    /// Return count of containers with this image
    pub fn containers(&self) -> i64 {
        self.containers
    }
}

fn nullable_priority_hash<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Default::default()))
}

fn nullable_vector<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Default::default()))
}