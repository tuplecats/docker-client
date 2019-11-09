use crate::http::Request;

mod create;
pub mod info;
mod remove;
mod killer;
pub mod inspector;
pub mod health_check;

pub use health_check::HealthCheck;
pub use create::CreatedContainer;
pub use create::Creator;
pub use remove::Remover;
pub use killer::Killer;



pub trait ToRequest {
    fn to_request(&self) -> Request;
}