//! TODO doc

mod create;
mod info;
mod delete;
mod list;

pub use create::VolumeCreator;
pub use info::VolumeInfo;
pub use delete::DeletedInfo;
pub use list::VolumesList;