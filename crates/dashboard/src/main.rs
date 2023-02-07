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

use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use anyhow::anyhow;
use app::PATH_JSON_GH_REPO_LIST;
use db::DB;
use parser::{
    findrepl::{self, CommentBlock},
    *,
};

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

    // let block = findrepl::CommentBlock {
    //     section_name: "tag_1".to_string(),
    //     marker: (Marker::Start, Marker::End),
    // };

    findrepl::replace(SAMPLE, CommentBlock::new("tag_1"), Path::new("README.md")).unwrap();

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
        {
            let file_path = Path::new("test.md");
            let section_tag = "dashboard";
            let content = r#"
<!--START_SECTION:dashboard-->
* [lloydlobo](https://github.com/lloydlobo/lloydlobo) — Lloyd Lobo's GitHub profile.
* [dashboard](https://github.com/lloydlobo/dashboard) — Mega dashboard for all my projects
* [homely-recipes](https://github.com/lloydlobo/homely-recipes) — The site that recommends hand-picked home-made recipes prepa...
* [rssh](https://github.com/lloydlobo/rssh) — rssh or Rust Shell allows keeping maintainable bash aliases ...
* [dio](https://github.com/lloydlobo/dio)
* [rusty](https://github.com/lloydlobo/rusty)
* [mononom-rust](https://github.com/lloydlobo/mononom-rust) — Projects built using Rust
* [mononom-api](https://github.com/lloydlobo/mononom-api)
* [jot](https://github.com/lloydlobo/jot)
* [rustspace](https://github.com/lloydlobo/rustspace)
* [rust-breakout-game](https://github.com/lloydlobo/rust-breakout-game) — Easy to play & build Breakout Game binary built in Rust with...
<!--END_SECTION:dashboard-->
                "#;

            {
                markdown::RegexMd::find_in_text(content, section_tag);
            }
            // markdown::update_markdown_file(file_path, section_tag, new_content)?;
        }
    }

    Ok(())
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

const SAMPLE: &str = r#"
* [lloydlobo](https://github.com/lloydlobo/lloydlobo) — Lloyd Lobo's GitHub profile.
* [dashboard](https://github.com/lloydlobo/dashboard) — Mega dashboard for all my projects
* [homely-recipes](https://github.com/lloydlobo/homely-recipes) — The site that recommends hand-picked home-made recipes prepa...
* [wavefncollapse](https://github.com/lloydlobo/wavefncollapse) — wavefncollapse
* [odin-rock-paper-scissors-docs](https://github.com/lloydlobo/odin-rock-paper-scissors-docs) — Documentation for the Game
* [styleguide-homely-recipes](https://github.com/lloydlobo/styleguide-homely-recipes) — Style, Brand, and Developer Guide for Homely Recipes
* [weather-app](https://github.com/lloydlobo/weather-app) — A functional stunning Weather App 🌥️   made using a sim...
* [rssh](https://github.com/lloydlobo/rssh) — rssh or Rust Shell allows keeping maintainable bash aliases ...
* [dio](https://github.com/lloydlobo/dio)
* [rusty](https://github.com/lloydlobo/rusty)
* [mononom-rust](https://github.com/lloydlobo/mononom-rust) — Projects built using Rust
* [mausam](https://github.com/lloydlobo/mausam) — A weather update desktop notifier made with Rust.
* [balance](https://github.com/lloydlobo/balance) — CLI tool to balance your budget
* [infinityper](https://github.com/lloydlobo/infinityper) — Simulate typed text in a terminal
* [treeleaf](https://github.com/lloydlobo/treeleaf) — Tree diagrams for the teriminal. Visualize mermaid-like data...
* [mononom-api](https://github.com/lloydlobo/mononom-api)
* [jot](https://github.com/lloydlobo/jot)
* [rustspace](https://github.com/lloydlobo/rustspace)
* [rust-breakout-game](https://github.com/lloydlobo/rust-breakout-game) — Easy to play & build Breakout Game binary built in Rust with...
* [rust_use-case_fibonacci](https://github.com/lloydlobo/rust_use-case_fibonacci) — Generate Fibonacci numbers instantly.
* [pompom](https://github.com/lloydlobo/pompom)
* [jots](https://github.com/lloydlobo/jots)
* [retro-racer](https://github.com/lloydlobo/retro-racer)
* [spaceshooter](https://github.com/lloydlobo/spaceshooter)
* [buck](https://github.com/lloydlobo/buck) — An envelope style budgeting app
* [amazone](https://github.com/lloydlobo/amazone) — E-Commerce Next.js, MongoDB, and Tailwind starter.
* [meinekraft](https://github.com/lloydlobo/meinekraft) — A minecraft clone built with React, Typescript, and ThreeJS.
* [dashboard-admin](https://github.com/lloydlobo/dashboard-admin)
* [neuraone](https://github.com/lloydlobo/neuraone)
* [go-react-todo](https://github.com/lloydlobo/go-react-todo) — A todo CRUD app with Go as backedn server & React as client ...
* [vortex-ball-collider](https://github.com/lloydlobo/vortex-ball-collider)
* [etcher-sketcher](https://github.com/lloydlobo/etcher-sketcher) — Sketch and etch your unique pixel art idea
* [cryptoculator](https://github.com/lloydlobo/cryptoculator) — A calculator app for The Odin Project
* [extensions](https://github.com/lloydlobo/extensions)
* [cryptoku](https://github.com/lloydlobo/cryptoku) — The API fetches the latest cryptocurrency pricing by market ...
* [quickpath](https://github.com/lloydlobo/quickpath) — Visualize shortest path between two points with algorithms
* [mononom-web-apps](https://github.com/lloydlobo/mononom-web-apps) — Web Apps that use neural networks, machine learning, and oth...
* [mononom-web](https://github.com/lloydlobo/mononom-web) — A monorepo which hosts static site generator type websites
* [odin-rock-paper-scissors](https://github.com/lloydlobo/odin-rock-paper-scissors) — A simple implementation of grade-school classic “rock pape...
* [gameoflife-rs](https://github.com/lloydlobo/gameoflife-rs)
* [ml_rs](https://github.com/lloydlobo/ml_rs)
* [mononom-alkhwarizmi](https://github.com/lloydlobo/mononom-alkhwarizmi)
* [mononom-scripting](https://github.com/lloydlobo/mononom-scripting) — Building JavaScript scripting CLI application for simple use...
* [loremipsum](https://github.com/lloydlobo/loremipsum)
* [advent-of-code](https://github.com/lloydlobo/advent-of-code)
* [leetcode](https://github.com/lloydlobo/leetcode) — leetcode problem solution documented
* [flashcarte](https://github.com/lloydlobo/flashcarte)
* [todo](https://github.com/lloydlobo/todo)
* [memo](https://github.com/lloydlobo/memo)
* [config](https://github.com/lloydlobo/config) — dotfiles
* [devlog](https://github.com/lloydlobo/devlog) — Developer Log detailing my learnings in Tech
* [course-cs50](https://github.com/lloydlobo/course-cs50) — CS50 Course Harvard
* [budjet](https://github.com/lloydlobo/budjet) — A ynab like budgeting app.
* [fyne](https://github.com/lloydlobo/fyne) — fyne
* [doyouenglish](https://github.com/lloydlobo/doyouenglish) — automate grammer fixes of your websites
* [thegameoflife](https://github.com/lloydlobo/thegameoflife) — Game of life terminal emulation.
* [studybuddy](https://github.com/lloydlobo/studybuddy) — Interactive note taking CLI application built with Go, Cobra...
* [chip8](https://github.com/lloydlobo/chip8) — A chip8 emulator powered by Go and SDL2.
* [fmtdot](https://github.com/lloydlobo/fmtdot) — fmtdot formats all files with given extension in a directory...
* [pswdhash](https://github.com/lloydlobo/pswdhash) — pswdhash CLI hashes passwords using bcrypt.
* [bak](https://github.com/lloydlobo/bak) — backup files in the current directory to a location
* [prep](https://github.com/lloydlobo/prep) — prep prepares CLI windows automation to run, test, and build...
* [robotgo](https://github.com/lloydlobo/robotgo) — Automation done right with robotgo.
* [okejoke](https://github.com/lloydlobo/okejoke) — okejoke gathers jokes on the fly in your CLI.
* [progcli](https://github.com/lloydlobo/progcli)
* [go-dad-jokes-cli](https://github.com/lloydlobo/go-dad-jokes-cli) — Go + Cobra for dad jokes API Cli usage
* [go-todo-client](https://github.com/lloydlobo/go-todo-client) — Client side TODO app for Go backend server
* [hello-go](https://github.com/lloydlobo/hello-go) — go starter repo test
* [go-greetings](https://github.com/lloydlobo/go-greetings) — Tutorial: Create a Go module
* [bashingthrudashell](https://github.com/lloydlobo/bashingthrudashell)
* [fzz](https://github.com/lloydlobo/fzz)
* [calcy](https://github.com/lloydlobo/calcy) — A calculator app for The Odin Project
* [progress](https://github.com/lloydlobo/progress) — A tool to stay motivated and fulfill the dictum of progress ...
* [fem-splitter](https://github.com/lloydlobo/fem-splitter) — Got to split money while tipping? Splitter to the rescue! A ...
* [test](https://github.com/lloydlobo/test) — test repository
* [.dotfiles](https://github.com/lloydlobo/.dotfiles)
* [astronvim_config](https://github.com/lloydlobo/astronvim_config) — Astro Nvim Config Files
* [wasm-game-of-life](https://github.com/lloydlobo/wasm-game-of-life) — Classic Game Of Life built in WebAssembly. Rust + JavaScript...
* [tense-text-transform](https://github.com/lloydlobo/tense-text-transform) — Change the tense of your sentences with this simple app.
* [zoomies](https://github.com/lloydlobo/zoomies)
* [rust-node_use-cases](https://github.com/lloydlobo/rust-node_use-cases) — A practical use-case of using Rust from Node.  Benchmark the...
* [api](https://github.com/lloydlobo/api)
* [quicknote](https://github.com/lloydlobo/quicknote) — Dev Notes with schemas built with Dendron
* [live-api](https://github.com/lloydlobo/live-api) — Demo API to host on RapidAPI
* [sortviz](https://github.com/lloydlobo/sortviz) — Vizualize sorting algorithms in slow motion
* [chat-decentralized](https://github.com/lloydlobo/chat-decentralized) — A decentralized chat room using Gun.js
* [blog](https://github.com/lloydlobo/blog)
* [neura-driver](https://github.com/lloydlobo/neura-driver) — A simple self-driving car application with a neural network ...
* [fem-tip-calculator-app](https://github.com/lloydlobo/fem-tip-calculator-app) — Your challenge is to build out this tip calculator app and g...
* [geomeasure](https://github.com/lloydlobo/geomeasure) — Geomeasure measures distance using GPS and that too without ...
* [pwa-starter](https://github.com/lloydlobo/pwa-starter) — Starter kit to build a quick PWA that aligns with lighthouse...
* [cli-devquiz](https://github.com/lloydlobo/cli-devquiz)
* [odin-recipes](https://github.com/lloydlobo/odin-recipes) — Odin's Recipes is a community-driven food brand providing tr...
* [odin-thors-landing](https://github.com/lloydlobo/odin-thors-landing) — Like Thor's mighty hammer, wield the power of security for y...
* [keybindings](https://github.com/lloydlobo/keybindings)
* [articles](https://github.com/lloydlobo/articles)
"#;
