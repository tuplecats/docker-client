//!
//! Container module.
//!

use crate::http::Request;

pub mod create;
pub mod remove;
pub mod killer;
mod health_check;
//pub mod inspector;
//pub mod info;

pub use health_check::HealthCheck;
pub use create::CreatedContainer;
pub use create::{CreatorBuilder, Creator};
pub use remove::{RemoverBuilder, Remover};
pub use killer::{KillerBuilder, Killer};

/// To request conversion trait.
pub trait ToRequest {

    /// Performs the conversion.
    fn to_request(&self) -> Request;

}