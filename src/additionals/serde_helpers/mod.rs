use std::collections::HashMap;
use serde::{Deserializer, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmptyObject;

pub fn nullable_priority_hash<'de, D>(deserializer: D) -> Result<HashMap<String, EmptyObject>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Default::default()))
}

pub fn nullable_priority_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Vec::new()))
}
