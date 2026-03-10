#![cfg_attr(feature = "strict", deny(warnings))]

pub mod error;
pub use error::DataError;

pub mod deduplication;
pub mod file_reconstruction;
pub mod processing;
pub mod progress_tracking;
