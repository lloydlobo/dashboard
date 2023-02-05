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
//!
//! ### Data - Github API
//!
//! Output of Github CLI command `gh repo list` is serialized to `gh_repo_list.json`
//!
//! ### Parsed API - Markdown
//!
//! The detail of each git repository is appended as a list item to `README.md`:
//!
//! ```md
//! * [name](url) — description
//! ```
//!
//! * `name` - Repository name
//! * `url` - Repository URL
//! * `description` - Description of the repository
//!

use anyhow::anyhow;
use thiserror::Error;

//------------------------------------------------------------------------------

/// Name of `dashboard` `package`in `/dashboard/Cargo.toml`.
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Path to `gh` cli output for `repo list` command.
const JSON_GH_REPO_LIST: &str = "gh_repo_list.json";

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

/// `Result<T, E>`
///
/// This is a reasonable return type to use throughout your application but also
/// for `fn main`; if you do, failures will be printed along with any
/// [context](https://docs.rs/anyhow/1.0.69/anyhow/trait.Context.html) and a backtrace if one was captured.
pub type Result<T, E> = anyhow::Result<T, E>;

/// `AppError`
///
/// # Example
///
/// ```ignore
/// fn simple_err(msg: &str) -> Result<(), anyhow::Error> {
///     return Err(anyhow!("MissingAttribute: {}", msg));
/// }
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
