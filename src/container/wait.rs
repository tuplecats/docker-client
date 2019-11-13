//! TODO doc

use crate::client::ErrorMessage;
use serde::Deserialize;

/// TODO doc
#[derive(Debug)]
pub enum WaitCondition {
    /// Not running
    NotRunning,

    /// Next exit
    NextExit,

    /// Removed
    Removed,
}

impl ToString for WaitCondition {
    fn to_string(&self) -> String {
        match self {
            WaitCondition::NotRunning => String::from("not-running"),
            WaitCondition::NextExit => String::from("next-exit"),
            WaitCondition::Removed => String::from("removed"),
        }
    }
}

impl Default for WaitCondition {
    fn default() -> Self {
        WaitCondition::NotRunning
    }
}


/// TODO doc
#[derive(Deserialize, Debug)]
pub struct WaitStatus {
    #[serde(rename = "StatusCode")]
    status_code: i32,

    #[serde(rename = "Error")]
    error: Option<ErrorMessage>,
}

impl WaitStatus {

    /// TODO doc
    pub fn status_code(&self) -> i32 {
        self.status_code
    }

    /// TODO doc
    pub fn error(&self) -> Option<ErrorMessage> {
        self.error.clone()
    }
}