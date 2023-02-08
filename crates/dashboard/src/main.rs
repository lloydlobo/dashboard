//! # dashboard
//!
//! `dashboard` is list of "GitHub Actions build status" of all the repos under my account, for my
//! own use.
//!
//! ## Usage
//!
//! Add this to your markdown file:
//!
//! ```md
//! <!--START_SECTION:dashboard-->
//! <!--END_SECTION:dashboard-->
//! ```
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

// #![deny(missing_debug_implementations, missing_docs)]

use anyhow::anyhow;
use dashboard::app::{self, AppError};
use lazy_static::lazy_static;

pub use self::app::*;

lazy_static! {
    // static ref ERR: AppError = AppError::LogicBug(anyhow!("Failed to find data").to_string());
}

//------------------------------------------------------------------------------

/// `main` entrypoint.
///
/// # Errors
///
/// This function will return an error if .
pub fn main() -> app::Result<(), AppError> {
    let start = std::time::Instant::now();
    pretty_env_logger::env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_level(log::LevelFilter::Debug)
        .init();

    if let Err(e) = app::try_main_refactor_v3("README.md") {
        eprintln!("{}", anyhow!(e));
        std::process::exit(1)
    }

    log::info!("Finished in {:#.2?}", start.elapsed());

    Ok(())
}

//------------------------------------------------------------------------------
