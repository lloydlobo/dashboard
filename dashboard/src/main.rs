//! # dashboard
//!
//! `dashboard` is list of "GitHub Actions build status" of all the repos under my account, for my own use.
//!
//! ## Development
//!
//! ### Usage
//!
//! ```sh
//! $ CARGO_LOG=error cargo r -p dashboard
//! ```

use anyhow::anyhow;
use thiserror::Error;

//------------------------------------------------------------------------------

fn main() -> Result<(), AppError> {
    pretty_env_logger::init();
    if let Err(e) = try_main() {
        eprintln!("{}", anyhow!(e));
        std::process::exit(1)
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn try_main() -> Result<(), AppError> {
    Ok(())
}

//------------------------------------------------------------------------------

pub type Result<T, E> = anyhow::Result<T, E>;

/// # Example
///
/// ```ignore
/// fn simple_err(msg: &str) -> Result<(), anyhow::Error> {
///     return Err(anyhow!("MissingAttribute: {}", msg));
/// }
///
/// fn return_err(msg: &str) -> AppError {
///     AppError::UnknownWithMsg(msg.to_string())
/// }
/// ```
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },

    #[error("Missing attribute: {0}")]
    MissingAttribute(String),

    #[error("Unknown error")]
    Unknown,

    #[error("Unknown error: {0}")]
    UnknownWithMsg(String),
}

//------------------------------------------------------------------------------
