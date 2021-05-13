use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopList {

    #[serde(rename = "Titles")]
    titles: Vec<String>,

    #[serde(rename = "Processes")]
    processes: Vec<Vec<String>>

}