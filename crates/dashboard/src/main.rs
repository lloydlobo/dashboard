//! # dashboard
//!
//! `dashboard` is list of "GitHub Actions build status" of all the repos under my account, for my
//! own use.

#![deny(missing_debug_implementations, missing_docs)]

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
#[tokio::main]
pub async fn main() -> app::Result<(), AppError> {
    dotenv::dotenv().ok();
    let start = std::time::Instant::now();
    pretty_env_logger::env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_level(log::LevelFilter::Debug)
        .init();

    if let Err(e) = app::try_main_refactor_v3("README.md").await {
        eprintln!("{}", anyhow!(e));
        std::process::exit(1)
    }

    log::info!("Finished in {:#.2?}", start.elapsed());

    Ok(())
}

//------------------------------------------------------------------------------
