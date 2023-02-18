//! Code utilized and modified from [matklad/cargo-xtask](https://github.com/matklad/cargo-xtask/blob/master/examples/hello-world/xtask/src/main.rs)

use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

use anyhow::{anyhow, Context};
use man::prelude::*;

////////////////////////////////////////////////////////////////////////////////

const PKG_NAME: &str = "dashboard";

type Result<T, E> = anyhow::Result<T, E>;
type DynError = Box<dyn std::error::Error>;

////////////////////////////////////////////////////////////////////////////////

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", anyhow!(e.to_string()));
        std::process::exit(-1);
    }
}

////////////////////////////////////////////////////////////////////////////////

fn run() -> Result<(), DynError> {
    let task: Option<String> = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => run_dist()?,
        Some("doc") => run_dist_doc()?,
        Some("parse-json") => run_parse_json()?,
        _ => print_help(),
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn print_help() {
    eprintln!(
        r#"xtask 0.1.0
A cargo-xtask automation tool

USAGE:
    cargo xtask [COMMAND]...

ARGS:
    dist            builds application and man pages
    doc             builds rustdoc documentation
    parse-json      parse crate dashboard output to custom json
"#
    )
}

////////////////////////////////////////////////////////////////////////////////

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}

fn dir_docs() -> PathBuf {
    project_root().join("docs/")
}

////////////////////////////////////////////////////////////////////////////////

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}

fn dist_manpage() -> Result<(), DynError> {
    let page = Manual::new(PKG_NAME).about("Dashboard to display all git projects.").render();
    fs::write(dist_dir().join(format!("{PKG_NAME}.man")), page)?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

/// Removes a directory at this path, after removing all its contents. Use carefully!
fn run_dist() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(dist_dir());
    fs::create_dir_all(dist_dir())?;

    dist_binary()?;
    dist_manpage()?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn run_dist_doc() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(dir_docs());
    dist_doc_xtask()?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

/// Runs the `parse.py` script and pipes its output to a file.
///
/// # Errors
///
/// Returns an error if the `python3` command or `tee` command fails to execute,
/// or if there is an error waiting for the output.
fn run_parse_json() -> Result<(), DynError> {
    // Fetches the environment variable `python3` from the current process, or uses "python3"
    // as a fallback value if the variable is not set.
    let python3: String = env::var("python3").unwrap_or_else(|_| "python3".to_string());
    let cmd1 = Command::new(python3)
        .current_dir(project_root())
        .args(["scripts/parse.py"])
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Spawning `python3 scripts/parse.py` failed")?;

    // Runs the `python3 scripts/parse.py` command and pipes (with `tee`) its output to a file.
    // * `tee` - Read from standard input and write to standard output and files (or commands).
    let cmd2 = Command::new("tee")
        .args(["parsed.json"])
        .stdin(cmd1.stdout.unwrap())
        .spawn()
        .with_context(|| "Failed to spawn process")?;

    // Waits for the output of the `tee` command and writes it to stdout.
    io::stdout()
        .lock()
        .write_all(&cmd2.wait_with_output().with_context(|| "Failed to wait for process")?.stdout)
        .with_context(|| "Failed to write to stdout")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn dist_binary() -> Result<(), DynError> {
    let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status: ExitStatus =
        Command::new(cargo).current_dir(project_root()).args(["build", "--release"]).status()?;

    if !status.success() {
        Err("cargo build failed")?
    }

    let dst: PathBuf = project_root().join(format!("target/release/{PKG_NAME}").as_str());

    fs::copy(&dst, dist_dir().join(PKG_NAME))?;

    match Command::new("strip").arg("--version").stdout(Stdio::null()).status().is_ok() {
        true => {
            eprintln!("stripping the binary");
            let status: ExitStatus = Command::new("strip").arg(&dst).status()?;
            if !status.success() {
                Err("strip failed")?;
            }
        }
        false => {
            eprintln!("no `strip` utility found");
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

/// # Equivalent shell script
///
/// [See reference](https://dev.to/deicuously/prepare-your-rust-api-docs-for-github-pages-2n5i)
///
/// ```bash
/// rm -rf ./docs
/// cargo doc --no-deps
/// echo "<meta http-equiv=\"refresh\" content=\"0; url=PKG_NAME\">" > target/doc/index.html
/// cp -r target/doc ./docs
/// ```
// rustc: `if` and `else` have incompatible types
// expected tuple `(&str, &str, &str)` found tuple `(&str, &str)`
// let copy_command =
// if cfg!(target_os = "windows") { ("cmd", "/C", "xcopy") } else { ("cp", "-r") };
// let copy_status = Command::new(copy_command.0) .arg(copy_command.1) .arg(copy_command.2)
// .arg(&copy_from) .arg(&copy_to) .status();
fn dist_doc_xtask() -> Result<(), DynError> {
    let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status: ExitStatus = Command::new(cargo)
        .current_dir(project_root())
        .args(["doc", "--release", "--no-deps"]) // .args(["doc", "--release", "--no-deps", "--bin", PKG_NAME])
        .status()
        .with_context(|| "Failed to build documentation")?;
    if !status.success() {
        Err("error: cargo doc failed")?;
    }

    let copy_from = project_root().join("target/doc");
    let copy_to = dir_docs();
    if let Ok(exit_status) = Command::new("cp").arg("--version").stdout(Stdio::null()).status() {
        if exit_status.success() {
            Command::new("cp")
                .args(["-r", &copy_from.to_string_lossy(), &copy_to.to_string_lossy()])
                .status()
                .with_context(|| "Failed to build documentation")?;
        } else {
            eprintln!("error: no `cp` utility found");
        }
    } else {
        return Err(anyhow!("error: failed to copy to directory with `cp`").into());
    }

    create_index_html_docs()?;

    Ok(())
}

/// Create psudo docs/index.html which points to the one in docs/package/index.html
/// Since github pages looks for index.html in the docs/ or root of the folder specified.
fn create_index_html_docs() -> Result<(), DynError> {
    let arg_html = format!("<meta http-equiv=\"refresh\" content=\"0; url={PKG_NAME}\">",);
    let new_html_index_path = "docs/index.html";

    let mut f_index_html = fs::File::create(new_html_index_path)?;
    if !f_index_html.metadata()?.is_file() {
        Err("error: failed to create file `{new_html_index}`")?
    }

    if let Err(err) = f_index_html.write_all(String::from(&arg_html).as_bytes()) {
        Err(format!(
            "error: failed to write `{arg_html:#?}` to file `{new_html_index_path}`: {err:#?}"
        ))?
    };

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
