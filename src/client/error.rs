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

    /// Unknown status
    UnknownStatus,

    /// Closed connection
    ClosedConnection,
}

/// `ErrorMessage` struct.
#[derive(Deserialize, Debug)]
pub struct ErrorMessage {
    /// Error message get from response.
    pub message: String,
}