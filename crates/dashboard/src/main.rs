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

mod markdown;

use std::{fs::OpenOptions, path::Path, sync::Arc};

use anyhow::anyhow;
use app::PATH_JSON_GH_REPO_LIST;
use db::DB;
use parser::findrepl::{self, CommentBlock};

use crate::{app::PATH_MD_OUTPUT, gh::GitRepoListItem};
pub use crate::{
    app::{AppError, Result},
    gh::{GitCliOps, GitRepo, RepositoryTopic},
};

//------------------------------------------------------------------------------

pub fn main() -> app::Result<(), app::AppError> {
    pretty_env_logger::env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter_level(log::LevelFilter::Debug)
        .init();
    if let Err(e) = try_main() {
        eprintln!("{}", anyhow!(e));
        std::process::exit(1)
    }
    Ok(())
}

fn try_main() -> app::Result<(), app::AppError> {
    let mut dashboard =
        app::App { config: config::Config {}, db: DB { data: None, repo_list: None } };

    dashboard.db.fetch_gh_repo_list_json().map_err(|e| app::AppError::AnyhowError(e.into()))?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(PATH_JSON_GH_REPO_LIST)
        .map_err(|e| app::AppError::Io(Arc::new(e)))?;

    serde_json::to_writer_pretty(file, &dashboard.db.data.as_ref().unwrap())
        .map_err(|e| app::AppError::Io(Arc::new(e.into())))?;

    log::info!("Wrote git repo list to file {}", PATH_JSON_GH_REPO_LIST);

    let list = dashboard
        .db
        .data
        .unwrap()
        .iter()
        .map(|repo| GitRepoListItem {
            name: repo.name.to_string(),
            url: repo.url.to_string(),
            description: repo.description.to_string(),
        })
        .collect();

    dashboard.db.repo_list = Some(list);

    let text: String = dashboard
        .db
        .repo_list
        .unwrap()
        .iter()
        .map(markdown::fmt_markdown_list_item)
        .collect::<Vec<_>>()
        .join("\n");

    findrepl::replace(&text, CommentBlock::new("tag_1".to_string()), Path::new(PATH_MD_OUTPUT))
        .map_err(|e| app::AppError::RegexError(e.into()))?;

    Ok(())
}

/* fn try_main() -> app::Result<(), app::AppError> {
    let mut dashboard =
        app::App { config: config::Config {}, db: DB { data: None, repo_list: None } };
    if let Err(e) = dashboard.db.fetch_gh_repo_list_json() {
        return Err(app::AppError::AnyhowError(e.into()));
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(PATH_JSON_GH_REPO_LIST)
        .map_err(|e| app::AppError::Io(Arc::new(e)))?;
    serde_json::to_writer_pretty(file, &dashboard.db.data.as_ref().unwrap())
        .map_err(|e| app::AppError::Io(Arc::new(e.into())))?;
    log::info!("Wrote git repo list to file {}", PATH_JSON_GH_REPO_LIST);
    let list = dashboard
        .db
        .data
        .unwrap()
        .iter()
        .map(|repo| GitRepoListItem {
            name: repo.name.to_string(),
            url: repo.url.to_string(),
            description: repo.description.to_string(),
        })
        .collect();
    dashboard.db.repo_list = Some(list);
    let text: String = dashboard
        .db
        .repo_list
        .unwrap()
        .iter()
        .map(markdown::fmt_markdown_list_item)
        .collect::<Vec<_>>()
        .join("\n");
    findrepl::replace(&text, CommentBlock::new("tag_1".to_string()), Path::new(PATH_MD_OUTPUT))
        .map_err(|e| app::AppError::RegexError(e.into()))?;
    Ok(())
} */

//------------------------------------------------------------------------------

pub mod app {
    //! `app` module contains `App` which contains prelude for all modules in this crate.

    use std::sync::Arc;

    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use crate::{config, db::DB};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct App {
        pub(crate) config: config::Config,
        pub(crate) db: DB,
    }

    /// `Result<T, E>`
    ///
    /// This is a reasonable return type to use throughout your application but also
    /// for `fn main`; if you do, failures will be printed along with any
    /// [context](https://docs.rs/anyhow/1.0.69/anyhow/trait.Context.html) and a backtrace if one was captured.
    pub type Result<T, E> = anyhow::Result<T, E>;

    /// `AppError`
    #[derive(Debug, thiserror::Error)]
    pub enum AppError {
        /// An error occurred while performing an I/O operation
        /// Instead of cloning the `std::io::Error`, we can store the error within the `AppError`
        /// as an `Arc` (Atomic Reference Counted) smart pointer. Allows for multiple references to
        /// the same error to be stored in different places without having to clone it.
        #[error("I/O error: {0}")]
        Io(#[from] Arc<std::io::Error>),
        /// An error occurred while performing an I/O operation with the xshell terminal.
        #[error("Xshell I/O error: {0}")]
        XshellIo(#[from] Arc<xshell::Error>),
        /// An error occurred while fetching the GitHub CLI response.
        #[error("GitHub CLI error: {0}")]
        Reqwest(#[from] reqwest::Error),
        /// An error occurred while processing the GitHub Actions workflow or CI cron job.
        #[error("GitHub Actions/CI error: {0}")]
        GithubActionsCi(String),
        /// An error occurred in the code logic
        #[error("Error in logic: {0}")]
        LogicBug(String),
        /// An error occurred using the anyhow library
        #[error("Anyhow error: {0}")]
        AnyhowError(#[from] anyhow::Error),
        /// An error occurred while parsing input
        #[error("Parsing error: {0}")]
        Parsing(#[from] parser::ParserError),
        /// An error occurred with a regular expression
        #[error("Regex error")]
        RegexError(#[from] regex::Error),
        /// An error occurred while interacting with the `xshell` terminal
        #[error("Xshell error")]
        XshellError(String),
        /// An error occurred with Github Actions or CI workflows
        #[error("CI/CD error")]
        CiCdError(String),
    }

    #[allow(dead_code)]
    /// Name of `dashboard` `package`in `/dashboard/Cargo.toml`.
    pub(crate) const PKG_NAME: &str = env!("CARGO_PKG_NAME");

    /// Path to `gh` cli output for `repo list` command.
    pub(crate) const PATH_JSON_GH_REPO_LIST: &str = "gh_repo_list.json";

    /// Path to markdown output for the list of `repo list` items.
    pub(crate) const PATH_MD_OUTPUT: &str = "README.md";

    pub(crate) const ARGS_GH_REPO_LIST_JSON: &[&str] = &[
        "createdAt",
        "description",
        "diskUsage",
        "id",
        "name",
        "pushedAt",
        "repositoryTopics",
        "sshUrl",
        "stargazerCount",
        "updatedAt",
        "url",
    ];
}

//------------------------------------------------------------------------------

pub mod config {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Config {}
}

//------------------------------------------------------------------------------

pub mod db {

    use std::sync::Arc;

    use anyhow::{anyhow, Context};
    use serde::{Deserialize, Serialize};
    use xshell::{cmd, Shell};

    use super::Result;
    use crate::{
        app::{AppError, ARGS_GH_REPO_LIST_JSON},
        gh::{self, GitCliOps, GitRepo, GitRepoListItem},
    };

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DB {
        pub data: Option<Vec<gh::GitRepo>>,
        pub repo_list: Option<Vec<GitRepoListItem>>,
    }

    impl GitCliOps for DB {
        /// Assigns the fetched response to `self.data`.
        fn fetch_gh_repo_list_json(&mut self) -> Result<(), AppError> {
            let sh = Shell::new().unwrap();
            let opts_json_args: String = ARGS_GH_REPO_LIST_JSON.join(",");

            let repos_json_ser: String =
                cmd!(sh, "gh repo list --source -L 999 --json {opts_json_args}")
                    .read()
                    .context(anyhow!("Failed to fetch github repositories with `gh` utility"))
                    .unwrap();
            log::info!("Fetched repositories with command: `gh repo list`");

            // "Failed to Deserialize repositories. {}",
            let repos_struct_de: Vec<GitRepo> = serde_json::from_str(&repos_json_ser)
                .map_err(|e| AppError::Io(Arc::new(e.into())))
                .unwrap();
            log::info!("Deserialized {} repositories", repos_struct_de.len());

            self.data = Some(repos_struct_de);

            Ok(())
        }
    }
}

//------------------------------------------------------------------------------

pub mod gh {
    use serde::{Deserialize, Serialize};

    use crate::app;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")] // https://serde.rs/attr-rename.html
    pub struct GitRepo {
        pub created_at: String,
        pub description: String,
        pub disk_usage: u32,
        pub id: String,
        pub name: String,
        //pub  primary_language: Lang,
        pub pushed_at: String,
        pub repository_topics: Option<Vec<RepositoryTopic>>,
        pub ssh_url: String,
        pub stargazer_count: u32,
        pub updated_at: String,
        pub url: String,
    }

    /// Custom data structure to parse into markdown list item.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")] // https://serde.rs/attr-rename.html
    pub struct GitRepoListItem {
        /// Repository name.
        pub name: String,
        /// URL of the git repo.
        pub url: String,
        /// Description of the repository.
        pub description: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RepositoryTopic {
        pub name: String,
    }

    pub trait GitCliOps {
        /// Use GitHub CLI `gh utility` in `xshell` to fetch list of repositories and,
        /// mutate `self.data` to the json `response` of [`Vec<GitRepo>`].
        ///
        /// * `xshell::Shell` - doesn't use the shell directly, but rather re-implements parts of
        ///   scripting environment in Rust.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        ///
        /// * [`std::env::current_dir`] - returns an error while creating new [`xshell::Shell`].
        /// * [`xshell::cmd!`] - on `read` returns a non-zero return code considered to be an error.
        /// * [`serde_json`] - conversion can fail if the structure of the input does not match the
        ///   structure expected by `Vec<GitRepo>`.
        fn fetch_gh_repo_list_json(&mut self) -> Result<(), app::AppError>;
    }
}

//------------------------------------------------------------------------------

// mod pretty_error {
//
//     use miette::prelude::*;
// In this example, each variant of the AppError struct is passed to miette::code to format the
// error message as a code block. The resulting value is then passed to the write! macro to format
// the error for display.     impl Display for AppError {
//         fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//             match self {
//                 AppError::GitHubApi(e) => {
//                     let error = e.to_string();
//                     let error = miette::code(&error);
//                     write!(f, "GitHub API error: {}", error)
//                 }
//                 AppError::Regex(e) => {
//                     let error = e.to_string();
//                     let error = miette::code(&error);
//                     write!(f, "Regex error: {}", error)
//                 }
//                 AppError::Io(e) => {
//                     let error = e.to_string();
//                     let error = miette::code(&error);
//                     write!(f, "I/O error: {}", error)
//                 }
//                 AppError::Anyhow(e) => {
//                     let error = e.to_string();
//                     let error = miette::code(&error);
//                     write!(f, "Anyhow error: {}", error)
//                 }
//                 AppError::Shell(e) => {
//                     let error = e.to_string();
//                     let error = miette::code(&error);
//                     write!(f, "Shell error: {}", error)
//                 }
//                 AppError::Printer(e) => {
//                     let error = e.to_string();
//                     let error = miette::code(&error);
//                     write!(f, "Printer error: {}", error)
//                 }
//             }
//         }
//     }
// }

// pub fn try_main() -> app::Result<(), app::AppError> {
//     let mut dashboard =
//         app::App { config: config::Config {}, db: DB { data: None, repo_list: None } };
//     dashboard.db.fetch_gh_repo_list_json()?;
//     let file: File = OpenOptions::new() .read(true) .write(true) .create(true)
// .open(PATH_JSON_GH_REPO_LIST) .unwrap();     serde_json::to_writer_pretty(file,
// &dashboard.clone().db.data.unwrap()).unwrap();     log::info!("Wrote git repo list to file
// `{PATH_JSON_GH_REPO_LIST}`");     let list = dashboard .db .data .unwrap() .iter()
//         .map(|repo| GitRepoListItem {
//             name: (*repo.name).to_string(),
//             url: (*repo.url).to_string(),
//             description: (*repo.description).to_string(),
//         })
//         .collect();
//     dashboard.db.repo_list = Some(list);
//     let text: String = dashboard .db .repo_list .unwrap() .iter()
// .map(markdown::fmt_markdown_list_item) .collect::<Vec<_>>() .join("\n");
//     if let Err(e) =
//         findrepl::replace(&text, CommentBlock::new("tag_1".to_string()),
// Path::new(PATH_MD_OUTPUT))     {
//         panic!("called `Result::unwrap()` on an `Err` value: {}", &e)
//     };
//     Ok(())
// }

// #[derive(Error, Debug)]
// pub enum AppError {
//     #[error("Invalid header (expected {expected:?}, got {found:?})")]
//     InvalidHeader { expected: String, found: String },
//
//     #[error("Missing attribute: {0}")]
//     MissingAttribute(String),
//
//     #[error("Unknown error")]
//     Unknown,
//
//     #[error("Unknown error: {0}")]
//     UnknownWithMsg(String),
// }

/* fn try_main() -> app::Result<(), app::AppError> {
    let mut dashboard =
        app::App { config: config::Config {}, db: DB { data: None, repo_list: None } };
    if let Err(e) = dashboard.db.fetch_gh_repo_list_json() {
        return Err(e);
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(PATH_JSON_GH_REPO_LIST)
        .map_err(|e| anyhow!("Failed to open file `{}`: {}", PATH_JSON_GH_REPO_LIST, e))?;
    serde_json::to_writer_pretty(file, &dashboard.db.data.as_ref().unwrap())
        .map_err(|e| anyhow!("Failed to write to file `{}`: {}", PATH_JSON_GH_REPO_LIST, e))?;
    log::info!("Wrote git repo list to file `{}`", PATH_JSON_GH_REPO_LIST);
    let list = dashboard
        .db
        .data
        .clone()
        .unwrap()
        .iter()
        .map(|repo| GitRepoListItem {
            name: repo.name.to_string(),
            url: repo.url.to_string(),
            description: repo.description.to_string(),
        })
        .collect();
    dashboard.db.repo_list = Some(list);
    let text: String = dashboard
        .db
        .repo_list
        .unwrap()
        .iter()
        .map(markdown::fmt_markdown_list_item)
        .collect::<Vec<_>>()
        .join("\n");
    findrepl::replace(&text, CommentBlock::new("tag_1".to_string()), Path::new(PATH_MD_OUTPUT))
        .map_err(|e| anyhow!("Failed to replace text in file `{}`: {}", PATH_MD_OUTPUT, e))?;
    Ok(())
} */
