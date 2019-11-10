//!
//! Container module.
//!

use crate::http::Request;

pub mod create;
pub mod remove;
pub mod kill;
pub mod health_check;

pub use health_check::HealthCheck;
pub use create::{CreatedContainer, CreatorBuilder, Creator};
pub use remove::{RemoverBuilder, Remover};
pub use kill::{KillerBuilder, Killer};

/// To request conversion trait.
pub trait ToRequest {

    /// Performs the conversion.
    fn to_request(&self) -> Request;

}