
use serde::Deserialize;

/// Deleted info structure
#[derive(Debug, Deserialize)]
pub struct DeletedInfo {

    #[serde(rename = "VolumesDeleted")]
    volumes_deleted: Vec<String>,

    #[serde(rename = "SpaceReclaimed")]
    space_reclaimed: i64,
}
