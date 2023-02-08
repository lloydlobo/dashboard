//! TODO:

//------------------------------------------------------------------------------
pub(crate) mod constant {
    //! TODO:

    #[allow(dead_code)]
    /// Name of `dashboard` `package`in `/dashboard/Cargo.toml`.
    pub(crate) const PKG_NAME: &str = env!("CARGO_PKG_NAME");

    /// Path to `gh` cli output for `repo list` command.
    pub(crate) const PATH_JSON_GH_REPO_LIST: &str = "gh_repo_list.json";

    /// Path to markdown output for the list of `repo list` items.
    pub(crate) const PATH_MD_OUTPUT: &str = "README.md";

    /// Desired json fields of repository list response from github cli.
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

    /// Word count limit for description.
    pub(crate) const DESC_WC: usize = 60;
}

pub mod app {
    //! `app` module contains `App` which contains prelude for all modules in this crate.
    use std::{
        fs::{File, OpenOptions},
        path::Path,
        sync::Arc,
    };

    use anyhow::anyhow;
    use crossbeam::thread;
    use parser::findrepl::{self, CommentBlock};
    use serde::{Deserialize, Serialize};

    use crate::{
        config,
        constant::{DESC_WC, PATH_JSON_GH_REPO_LIST, PATH_MD_OUTPUT},
        db::DB,
        gh::{GitCliOps, GitRepo, GitRepoListItem},
        util,
    };

    /// `Result<T, E>`
    ///
    /// This is a reasonable return type to use throughout your application but also
    /// for `fn main`; if you do, failures will be printed along with any
    /// [context](https://docs.rs/anyhow/1.0.69/anyhow/trait.Context.html) and a backtrace if one was captured.
    pub type Result<T, E> = anyhow::Result<T, E>;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct App {
        pub(crate) config: config::Config,
        pub(crate) db: DB,
    }

    pub fn try_main_refactor_v3(file_path: &str) -> Result<(), AppError> {
        let mut dashboard =
            App { config: config::Config {}, db: DB { data: None, repo_list: None } };

        GitCliOps::fetch_repos_write_data(&mut dashboard.db)?;

        // Spawning the two operations into separate threads for parallel execution
        thread::scope(|s| {
            s.spawn(|_| update_markdown_file(dashboard.db.data.as_ref(), file_path));
            s.spawn(|_| write_json_file(dashboard.db.data.as_ref(), file_path));
        }) // PERF: Learn to handle error of type: `e: Box<dyn Any + Send>`.
        .map_err(|e| AppError::CrossbeamError(anyhow!("{:?}", e)))?;

        Ok(())
    }

    fn update_markdown_file(data: Option<&Vec<GitRepo>>, file_path: &str) -> Result<(), AppError> {
        // If data is Some, convert `Vec<GitRepo>` into a list of GitRepoListItem.
        let list = match data {
            Some(data) => data.iter().map(GitRepoListItem::new).collect::<Vec<_>>(),
            None => return Err(AppError::UnwrapError("Failed to find data".to_string())),
        };

        let (text, block) = rayon::join(
            || list.iter().map(fmt_markdown_list_item).collect::<Vec<_>>().join("\n"),
            || CommentBlock::new("tag_1".to_string()),
        );
        findrepl::replace_par(&text, block, Path::new(file_path)).map_err(AppError::ParserError)?;
        log::info!("Updated git repo list in file {}", file_path);

        Ok(())
    }

    fn write_json_file(data: Option<&Vec<GitRepo>>, file_path: &str) -> Result<(), AppError> {
        let path = util::replace_file_extension(file_path, "json");

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .map_err(|e| AppError::Io(Arc::new(e)))?;

        let data = match data {
            Some(data) => data,
            None => return Err(AppError::UnwrapError("No data found".to_string())),
        };

        log::info!("Writing git repo list to file {}", &path);

        serde_json::to_writer_pretty(file, data).map_err(AppError::SerdeError)
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

    /// `AppError`
    //
    /// Instead of cloning the `std::io::Error`, we can store the error within the `AppError`
    /// as an `Arc` (Atomic Reference Counted) smart pointer. Allows for multiple references to
    /// the same error to be stored in different places without having to clone it.
    #[derive(Debug, thiserror::Error)]
    pub enum AppError {
        /// An error occurred while performing an I/O operation
        #[error("I/O error: {0}")]
        Io(#[from] Arc<std::io::Error>),
        /// An error occurred in the code logic
        #[error("Error in logic: {0}")]
        LogicBug(String),
        /// An error occurred while parsing input
        #[error("Parsing error: {0}")]
        ParserError(#[from] parser::ParserError),
        /// An error occurred in parser crate.
        #[error("parser package I/O error: {0}")]
        FindReplaceError(parser::ParserError),
        /// An error occurred with a regular expression
        #[error("Regex error")]
        RegexError(#[from] regex::Error),
        /// An error occurred while serializing or deserializing with serde
        #[error("Serde error: {0}")]
        SerdeError(#[from] serde_json::Error),
        /// Catch the panic and return a value of
        #[error("Unwrap on a None value error: {0}")]
        UnwrapError(String),
        /// An error occurred while interacting with the `xshell` terminal
        #[error("Xshell error")]
        XshellError(String),
        /// An error occurred while performing an I/O operation with the xshell terminal.
        #[error("Xshell I/O error: {0}")]
        XshellIo(#[from] xshell::Error),
        /// An error occurred while performing an I/O operation across channels with crossbeam.
        #[error("Crossbeam I/O error: {0}")]
        CrossbeamError(#[from] anyhow::Error),
        // The error trait from the serde_json crate would suffice. When you serialize or
        // deserialize data using the serde_json library, the errors that may occur are related to
        // JSON format specifically. These errors are captured by the serde_json::Error type. In
        // contrast, the serde crate provides a more generic mechanism for serialization and
        // deserialization of data, and its error type is serde::ser::Error. If you are only
        // dealing with JSON data, it is best to use the serde_json crate and handle
        // serde_json::Error errors. , if you're using the toml crate, you could wrap a
        // toml::de::Error in your custom error enum , and return that in the case of a
        // serialization or deserialization error. If using the ron , crate, you would wrap
        // ron::de::Error
        //
        // TODO:
        // /// An error occurred using the anyhow library
        // #[error("Anyhow error: {0}")]
        // AnyhowError(#[from] anyhow::Error),
        // /// An error occurred with Github Actions or CI workflows
        // #[error("CI/CD error")]
        // CiCdError(String),
        // /// Error while processing the GitHub Actions workflow or CI cron job.
        // #[error("GitHub Actions/CI error: {0}")]
        // GithubActionsCi(String),
        // /// An error occurred while fetching the GitHub CLI response.
        // #[error("GitHub CLI error: {0}")]
        // Reqwest(#[from] reqwest::Error),
    }
}

//------------------------------------------------------------------------------

pub(crate) mod db {

    use serde::{Deserialize, Serialize};
    use xshell::{cmd, Shell};

    use crate::{
        app::AppError,
        constant::ARGS_GH_REPO_LIST_JSON,
        gh::{self, GitCliOps, GitRepo, GitRepoListItem},
    };

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DB {
        pub data: Option<Vec<gh::GitRepo>>,
        pub repo_list: Option<Vec<GitRepoListItem>>,
    }

    /// # Errors
    ///
    /// * If not connected to the internet or server side error:
    ///     ```sh
    ///     error connecting to api.github.com
    ///     check your internet connection or https://githubstatus.com
    ///     Xshell I/O error: command exited with non-zero code `gh repo list --source -L 999 --json
    ///     createdAt,description,diskUsage,id,name,pushedAt,repository Topics,sshUrl,stargazerCount,
    ///     updatedAt,url`: 1
    ///     ```
    impl GitCliOps for DB {
        /// Assigns the fetched response to `self.data`.
        fn fetch_repos_write_data(&mut self) -> Result<(), AppError> {
            let sh = Shell::new().map_err(AppError::XshellIo)?;
            let opts_json_args: String = ARGS_GH_REPO_LIST_JSON.join(",");

            let repos: String = cmd!(sh, "gh repo list --source -L 999 --json {opts_json_args}")
                .read()
                .map_err(AppError::XshellIo)?;
            log::info!("Fetched repositories with command: `gh repo list`");

            let repos: Vec<GitRepo> = serde_json::from_str(&repos).map_err(AppError::SerdeError)?;
            log::info!("Deserialized {} repositories", repos.len());

            self.data = Some(repos);

            Ok(())
        }
    }
}

//------------------------------------------------------------------------------

pub(crate) mod gh {
    use serde::{Deserialize, Serialize};

    use crate::app::AppError;

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

    impl GitRepoListItem {
        pub fn new(repo: &GitRepo) -> Self {
            Self {
                name: repo.name.clone(),
                url: repo.url.clone(),
                description: repo.description.clone(),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RepositoryTopic {
        pub name: String,
    }

    pub trait GitCliOps {
        /// Use GitHub CLI `gh utility` in `xshell` to fetch list of repositories and,
        /// mutate `self.data` to the json `response` of [`Vec<GitRepo>`].
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        ///
        /// * [`std::env::current_dir`] - returns an error while creating new [`xshell::Shell`].
        /// * [`xshell::cmd!`] - on `read` returns a non-zero return code considered to be an error.
        /// * [`serde_json`] - conversion can fail if the structure of the input does not match the
        ///   structure expected by `Vec<GitRepo>`.
        ///
        /// # NOTE
        ///
        /// `xshell::Shell` - doesn't use the shell directly, but rather re-implements parts of
        /// scripting environment in Rust.
        fn fetch_repos_write_data(&mut self) -> Result<(), AppError>;
    }
}

pub(crate) mod util {
    #![allow(dead_code)]

    //! Common utility functions and items. YAGNI!
    //!
    //! # Find and replace file extensions:
    //!
    //! In general, file stem methods would be more efficient in cases where the goal is to simply
    //! extract a portion of the file name and modify it. This is because the file_stem and
    //! set_file_stem methods are designed specifically for this purpose, and can handle it with
    //! minimal overhead.
    //!
    //! However, if the goal is to manipulate the file name in a more complex way, such as replacing
    //! a specific substring or splitting the name into multiple components, then regex might be a
    //! better choice. Regular expressions are more flexible and allow for more complex string
    //! manipulations, but they can also be more computationally expensive and require more code to
    //! implement.

    use std::path::Path;

    use regex::Regex;

    /// `replace_file_extension`
    ///
    /// In this function, we first create a Path from the file_path. Then, we get the file stem of
    /// the path using file_stem() and convert it to a &str using to_str(). Finally, we return a
    /// new String created using the format! macro that consists of the stem, a dot (.), and the
    /// new_extension.
    pub(crate) fn replace_file_extension(file_path: &str, new_extension: &str) -> String {
        let path = Path::new(file_path);
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let new_file_path = format!("{}.{}", stem, new_extension);
        new_file_path
    }

    /// `replace_extension_regex`
    ///
    /// The regex crate is used in this example to define a regular expression that matches the file
    /// extension in the given file_path. The Regex::new method creates a new Regex object from a
    /// string pattern, and the replace method replaces the matched text with the new extension. The
    /// result is then returned as a String.
    /// In this regular expression, the (?i) flag makes the match case-insensitive, and \.[^./]+$
    /// matches a dot followed by one or more characters that are not dots or slashes, until the end
    /// of the string.
    pub(crate) fn replace_extension_regex(file_path: &str, new_extension: &str) -> String {
        let re = Regex::new(r"(?i)\.[^./]+$").unwrap();
        re.replace(file_path, new_extension).to_string()
    }

    /* /// `FileExtension` provides two methods, to replace the file extension to a new `ext`, using
        /// the `file_stem` method, and a `regex` regular expression.
        pub trait FileExtension {
            /// ```rust
            /// use std::path::Path;
            ///
            /// use crate::dashboard::util::FileExtension;
            ///
            /// let path = Path::new("file.md");
            /// let new_file_path = path.replace_file_extension_with_file_stem("file.txt", "new");
            /// assert_eq!("file.txt", new_file_path);
            /// ```
            fn replace_file_extension_with_file_stem(&self, path: &str, ext: &str) -> String;

            /// ```rust
            /// use std::path::Path;
            ///
            /// use crate::dashboard::util::FileExtension;
            ///
            /// let path = Path::new("file.md");
            /// let new_file_path = path.replace_file_extension_with_regex("file.txt", "new");
            /// println!("{}", new_file_path);
            /// ```
            fn replace_file_extension_with_regex(&self, path: &str, extension: &str) -> String;
        }

        impl FileExtension for Path {
            fn replace_file_extension_with_file_stem(&self, path: &str, ext: &str) -> String {
                let path = Path::new(path);
                let stem = path.file_stem().unwrap().to_str().unwrap();
                let new_file_path = format!("{}.{}", stem, ext);
                new_file_path
            }

            fn replace_file_extension_with_regex(&self, path: &str, ext: &str) -> String {
                let re = Regex::new(r"(?i)\.[^./]+$").unwrap();
                re.replace(path, ext).to_string()
            }
        }
    */
}

//------------------------------------------------------------------------------

pub(crate) mod config {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Config {}
}

//------------------------------------------------------------------------------

pub(crate) mod archive {
    #![allow(dead_code)]

    use std::{
        fs::{File, OpenOptions},
        path::Path,
        sync::Arc,
    };

    use anyhow::anyhow;
    use crossbeam::thread;
    use parser::findrepl::{self, CommentBlock};

    use crate::{
        app::{fmt_markdown_list_item, App, AppError},
        config,
        constant::{PATH_JSON_GH_REPO_LIST, PATH_MD_OUTPUT},
        db::DB,
        gh::{GitCliOps, GitRepo, GitRepoListItem},
    };

    // pub fn try_main_refactor_v2(file_path: &str) -> Result<(), AppError> {
    //     let mut dashboard =
    //         App { config: config::Config {}, db: DB { data: None, repo_list: None } };
    //     GitCliOps::fetch_repos_write_data(&mut dashboard.db)?;
    //     update_markdown_file(dashboard.db.data.as_ref(), file_path)?;
    //     write_json_file(dashboard.db.data.as_ref(), file_path)?;
    //     Ok(())
    // }

    pub fn try_main_refactor() -> Result<(), AppError> {
        let mut dashboard =
            App { config: config::Config {}, db: DB { data: None, repo_list: None } };

        GitCliOps::fetch_repos_write_data(&mut dashboard.db)?;

        thread::scope(|_| {
            let list: Vec<_> = match dashboard.db.data.as_ref() {
                Some(data) => data.iter().map(GitRepoListItem::new).collect(),
                None => return Err(AppError::UnwrapError("Failed to find data".to_string())),
            };

            rayon::join(
                || {
                    log::info!("Updating git repo list in file {}", PATH_MD_OUTPUT);
                    findrepl::replace_par(
                        &list.iter().map(fmt_markdown_list_item).collect::<Vec<_>>().join("\n"),
                        CommentBlock::new("tag_1".to_string()),
                        Path::new(PATH_MD_OUTPUT),
                    )
                    .map_err(AppError::ParserError)
                },
                || dashboard.db.repo_list = Some(list.clone()),
            )
            .0
        })
        .map_err(|e| AppError::CrossbeamError(anyhow!("{e:?}")))??;

        thread::scope(|_| {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(PATH_JSON_GH_REPO_LIST)
                .map_err(|e| AppError::Io(Arc::new(e)))?;
            let data = match dashboard.db.data.as_ref() {
                Some(data) => data,
                None => return Err(AppError::UnwrapError("No data found".to_string())),
            };

            log::info!("Writing git repo list to file {}", PATH_JSON_GH_REPO_LIST);
            serde_json::to_writer_pretty(file, data).map_err(AppError::SerdeError)
        })
        .map_err(|e| AppError::CrossbeamError(anyhow!("{e:?}")))??;

        Ok(())
    }

    pub(crate) fn try_main() -> Result<(), AppError> {
        let mut dashboard =
            App { config: config::Config {}, db: DB { data: None, repo_list: None } };

        dashboard.db.fetch_repos_write_data()?;

        crossbeam::scope(|_| {
            let list: Vec<_> = dashboard
                .db
                .data
                .as_ref()
                .ok_or_else(|| AppError::LogicBug(anyhow!("Failed to find data").to_string()))
                .unwrap()
                .iter()
                .map(GitRepoListItem::new)
                .collect();
            rayon::join(
                || {
                    let text =
                        list.iter().map(fmt_markdown_list_item).collect::<Vec<_>>().join("\n");
                    findrepl::replace_par(
                        &text,
                        CommentBlock::new("tag_1".to_string()),
                        Path::new(PATH_MD_OUTPUT),
                    )
                    .map_err(AppError::ParserError)
                    .unwrap()
                },
                || dashboard.db.repo_list = Some(list.clone()),
            );
        })
        .unwrap();

        {
            let file: File = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(PATH_JSON_GH_REPO_LIST)
                .map_err(|e| AppError::Io(Arc::new(e)))?;
            let data: &Vec<GitRepo> = dashboard
                .db
                .data
                .as_ref()
                .ok_or_else(|| AppError::UnwrapError("No data found".to_string()))?;
            serde_json::to_writer_pretty(file, data).map_err(AppError::SerdeError).unwrap();
            log::info!("Wrote git repo list to file {}", PATH_JSON_GH_REPO_LIST);
        }

        Ok(())
    }
}

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

//------------------------------------------------------------------------------
