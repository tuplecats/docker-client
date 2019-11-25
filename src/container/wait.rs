//! Wait module

use crate::client::ErrorMessage;
use serde::Deserialize;

/// Wait condition enum
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


/// Wait status structure
#[derive(Deserialize, Debug)]
pub struct WaitStatus {
    #[serde(rename = "StatusCode")]
    status_code: i32,

    #[serde(rename = "Error")]
    error: Option<ErrorMessage>,
}

impl WaitStatus {

    /// Return status code
    pub fn status_code(&self) -> i32 {
        self.status_code
    }

    /// Return error `ErrorMessage`
    pub fn error(&self) -> Option<ErrorMessage> {
        self.error.clone()
    }
}