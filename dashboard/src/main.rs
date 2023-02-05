//! # dashboard
//!
//! `dashboard` is list of "GitHub Actions build status" of all the repos under my account, for my
//! own use.
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

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use anyhow::anyhow;
use app::PATH_JSON_GH_REPO_LIST;
use db::DB;

use crate::{app::PATH_MD_OUTPUT, gh::GitRepoListItem};
pub use crate::{
    app::{AppError, Result},
    gh::{GitCliOps, GitRepo, RepositoryTopic},
};

//------------------------------------------------------------------------------

pub fn main() -> app::Result<(), app::AppError> {
    let mut builder = pretty_env_logger::env_logger::builder();
    builder.filter_level(log::LevelFilter::Info).init();

    if let Err(e) = try_main() {
        eprintln!("{}", anyhow!(e));
        std::process::exit(1)
    }
    Ok(())
}

pub fn try_main() -> app::Result<(), app::AppError> {
    let mut dashboard =
        app::App { config: config::Config {}, db: DB { data: None, repo_list: None } };

    {
        dashboard.db.fetch_gh_repo_list_json()?;
    }
    {
        let file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&PATH_JSON_GH_REPO_LIST)
            .unwrap();
        serde_json::to_writer_pretty(file, &dashboard.clone().db.data.unwrap()).unwrap();
        log::info!("Wrote git repo list to file `{PATH_JSON_GH_REPO_LIST}`");
    }
    {
        let data: Vec<GitRepo> = (&dashboard.clone()).db.data.clone().unwrap();
        let md_list: Vec<GitRepoListItem> = data
            .iter()
            .map(|repo| GitRepoListItem {
                name: (*repo.name).to_string(),
                url: (*repo.url).to_string(),
                description: (*repo.description).to_string(),
            })
            .collect();
        dashboard.db.repo_list = Some(md_list);
        {
            let repo_list: Vec<GitRepoListItem> = dashboard.db.repo_list.unwrap();
            let items: Vec<String> =
                repo_list.iter().map(markdown::fmt_markdown_list_item).collect();
            let mut file: File = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&PATH_MD_OUTPUT)
                .unwrap();
            // Write to file.
            for line in items {
                let line = format!("{}\n", line);
                file.write_all(line.as_bytes()).unwrap();
            }
            log::info!("Wrote repo list items to file `{PATH_MD_OUTPUT}`");
        }
    }

    Ok(())
}

mod markdown {
    use std::{
        fs::{rename, File},
        io::{BufRead, BufReader, Write},
        path::Path,
    };

    use crate::{gh::GitRepoListItem, AppError, Result};

    /// Word count limit for description.
    pub const DESC_WC: usize = 60;

    /// Indicates if visitor is inside markdown start and end section blocks.
    pub enum SectionState {
        OutsideSection,
        InSection,
    }

    #[macro_export]
    macro_rules! comment_block {
        ($section_name:expr, $marker:expr) => {
            format!("<!--{}_SECTION:{}-->", $marker, $section_name)
        };
    }

    fn update_markdown_file(path: &Path, section_tag: &str, content: &str) -> Result<(), AppError> {
        let mut section_state = SectionState::OutsideSection;
        let mut new_lines: Vec<String> = Vec::new();

        let file = File::open(path).unwrap();
        let reader: BufReader<File> = BufReader::new(file);

        reader.lines().for_each(|line| match line.unwrap().as_str() {
            line if line.contains(&comment_block!(section_tag, "START")) => {
                section_state = SectionState::InSection;
                new_lines.push(line.to_owned());
                new_lines.push(content.to_owned());
            }
            line if line.contains(&comment_block!(section_tag, "END")) => {
                section_state = SectionState::OutsideSection;
                new_lines.push(line.to_owned());
            }
            line if matches!(section_state, SectionState::OutsideSection) => {
                new_lines.push(line.to_owned());
            }
            &_ => {}
        });

        Ok(())
    }

    /// Create and format a new markdown list item with repo name, url and its description.
    pub(crate) fn fmt_markdown_list_item(i: &GitRepoListItem) -> String {
        match i.description.is_empty() {
            true => format!("* [{}]({})", i.name, i.url),
            false => match i.description.len() > DESC_WC {
                true => {
                    format!("* [{}]({}) — {}...", i.name, i.url, i.description.split_at(DESC_WC).0)
                }
                false => format!("* [{}]({}) — {}", i.name, i.url, i.description),
            },
        }
    }
}

//------------------------------------------------------------------------------

pub mod app {
    //! `app` module contains `App` which contains prelude for all modules in this crate.

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

    #[allow(dead_code)]
    /// Name of `dashboard` `package`in `/dashboard/Cargo.toml`.
    pub(crate) const PKG_NAME: &str = env!("CARGO_PKG_NAME");

    /// Path to `gh` cli output for `repo list` command.
    pub(crate) const PATH_JSON_GH_REPO_LIST: &str = "gh_repo_list.json";

    /// Path to markdown output for the list of `repo list` items.
    pub(crate) const PATH_MD_OUTPUT: &str = "test_readme.md";

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
    use std::{
        fs::{File, OpenOptions},
        path::Path,
    };

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

            let repos_struct_de: Vec<GitRepo> = serde_json::from_str(&repos_json_ser)
                .context(anyhow!("Failed to Deserialize repositories"))
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
