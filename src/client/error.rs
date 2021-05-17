use serde::Deserialize;

/// `DockerError` enum.
#[derive(Debug)]
pub enum DockerError {
    /// Bad parameters (HTTP status is 401)
    BadParameters(ErrorMessage), // 401

    /// Server error (HTTP status is 500)
    ServerError(ErrorMessage), // 500

    /// Server error (HTTP status is 404)
    NotFound(ErrorMessage), // 404

    /// Server error (HTTP status is 409)
    NotRunning(ErrorMessage), // 409

    /// Server error (HTTP status is 304)
    AlreadyStarted(ErrorMessage), // 304

    /// Server error (HTTP status is 409)
    ContainerExists(ErrorMessage), // 409

    /// Server error (HTTP status is 409)
    NetworkExists(ErrorMessage), // 409

    /// Busy by container (HTTP status is 409)
    Busy(ErrorMessage), // 409

    /// Container paused
    ContainerPaused(ErrorMessage),

    /// Unknown status
    UnknownStatus,

    /// Closed connection
    ClosedConnection,
}

/// `ErrorMessage` struct.
#[derive(Deserialize, Debug, Clone)]
pub struct ErrorMessage {
    /// Error message get from response.
    pub message: String,
}

impl DockerError {

    pub fn get_error_message(&self) -> Option<String> {
        match self {
            DockerError::BadParameters(ref msg) => { Some(msg.message.clone()) }
            DockerError::ServerError(ref msg) => { Some(msg.message.clone()) }
            DockerError::NotFound(ref msg) => { Some(msg.message.clone()) }
            DockerError::NotRunning(ref msg) => { Some(msg.message.clone()) }
            DockerError::AlreadyStarted(ref msg) => { Some(msg.message.clone()) }
            DockerError::ContainerExists(ref msg) => { Some(msg.message.clone()) }
            DockerError::Busy(ref msg) => { Some(msg.message.clone()) }
            DockerError::NetworkExists(ref msg) => { Some(msg.message.clone()) }
            DockerError::ContainerPaused(ref msg) => { Some(msg.message.clone()) }
            DockerError::UnknownStatus => { None }
            DockerError::ClosedConnection => { None }
        }
    }

}