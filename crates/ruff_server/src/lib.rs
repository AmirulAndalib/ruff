//! ## The Ruff Language Server
#![allow(dead_code)]

// Use
pub use edit::{Document, PositionEncoding};
pub use server::Server;
const SERVER_NAME: &str = "ruff";
const DIAGNOSTIC_NAME: &str = "Ruff";

// Modules
mod edit;
mod format;
mod lint;
mod server;
mod session;

// Types
pub(crate) type Result<T> = anyhow::Result<T>;
