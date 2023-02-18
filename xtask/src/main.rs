//! Code utilized and modified from [matklad/cargo-xtask](https://github.com/matklad/cargo-xtask/blob/master/examples/hello-world/xtask/src/main.rs)

use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

use anyhow::{anyhow, Context};
use man::prelude::*;

type Result<T, E> = anyhow::Result<T, E>;
type DynError = Box<dyn std::error::Error>;

const PKG_NAME: &str = "dashboard";

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", anyhow!(e.to_string()));
        std::process::exit(-1);
    }
}

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

// ```sh
// Append to the given files, do not overwrite:
// echo "example" | tee -a path/to/file
// Print standard input to the terminal, and also pipe it into another program for further processing:
// echo "example" | tee /dev/tty | xargs printf "[%s]"
// Create a directory called "example", count the number of characters in "example" and write "example" to the terminal:
// echo "example" | tee >(xargs mkdir) >(wc -c)
// let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
// ```
fn run_parse_json() -> Result<(), DynError> {
    // Fetches the environment variable `key` from the current process.
    let python3: String = env::var("python3").unwrap_or_else(|_| "python3".to_string());
    let cmd1 = Command::new(python3)
        .current_dir(project_root())
        .args(["scripts/parse.py"])
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to spawn process")?;
    if let Some(err) = cmd1.stderr {
        Err(anyhow!("`python3 scripts/parse.py` failed: {err:?}", err = err))?
    };
    // Read from standard input and write to standard output and files (or commands).
    // Copy standard input to each file, and also to standard output:
    let cmd2 = Command::new("tee")
        .arg("parsed.json")
        .stdin(cmd1.stdout.unwrap())
        .spawn()
        .with_context(|| "Failed to spawn process")?;

    let output = cmd2.wait_with_output().with_context(|| "Failed to wait for process")?;
    let buf: &[u8] = &output.stdout;
    io::stdout().lock().write_all(buf).with_context(|| "Failed to write to stdout")?;

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}

/// Removes a directory at this path, after removing all its contents. Use carefully!
fn run_dist() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(dist_dir());
    fs::create_dir_all(dist_dir())?;
    dist_binary()?;
    dist_manpage()?;

    Ok(())
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}

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

fn dist_manpage() -> Result<(), DynError> {
    let page = Manual::new(PKG_NAME).about("Wave function collapse").render();
    fs::write(dist_dir().join(format!("{PKG_NAME}.man")), page)?;

    Ok(())
}

fn run_dist_doc() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(dir_docs());
    dist_doc_xtask()?;

    Ok(())
}

fn dir_docs() -> PathBuf {
    project_root().join("docs/")
}

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
fn dist_doc_xtask() -> Result<(), DynError> {
    let cargo: String = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status: ExitStatus = Command::new(cargo)
        .current_dir(project_root())
        .args(["doc", "--release", "--no-deps"]) // .args(["doc", "--release", "--no-deps", "--bin", PKG_NAME])
        .status()?;
    if !status.success() {
        Err("error: cargo doc failed")?;
    }

    {
        let copy_from: PathBuf = project_root().join("target/doc");
        let copy_to = dir_docs();
        if Command::new("cp").arg("--version").stdout(Stdio::null()).status().is_ok() {
            eprintln!("info: copying `target/doc` directory to `docs/`");
            let exit_status = Command::new("cp")
                .args(["-r", &copy_from.to_string_lossy(), &copy_to.to_string_lossy()])
                .status()?;
            if !exit_status.success() {
                Err("error: failed to copy to directory with `cp`")?;
            }
        } else {
            eprintln!("error: no `cp` utility found")
        }
    }

    // Create psudo docs/index.html which points to the one in docs/package/index.html
    // Since github pages looks for index.html in the docs/ or root of the folder specified.
    let arg_html = format!("<meta http-equiv=\"refresh\" content=\"0; url={PKG_NAME}\">",);
    // let new_html_index = "target/doc/index.html";
    let new_html_index_path = "docs/index.html";
    let mut f_index_html = fs::File::create(new_html_index_path)?;
    if !f_index_html.metadata()?.is_file() {
        Err("error: failed to create file `{new_html_index}`")?;
    }
    let write_all = f_index_html.write_all(String::from(&arg_html).as_bytes());
    if let Err(err) = write_all {
        Err(format!(
            "error: failed to write `{arg_html:#?}` to file `{new_html_index_path}`: {err:#?}"
        ))?
    };

    Ok(())
}

// // stdout must be configured with `Stdio::piped` in order to use
// // `echo_child.stdout`
// let echo_child = Command::new("echo")
//     .arg("Oh no, a tpyo!")
//     .stdout(Stdio::piped())
//     .spawn()
//     .expect("Failed to start echo process");
//
// // Note that `echo_child` is moved here, but we won't be needing
// // `echo_child` anymore
// let echo_out = echo_child.stdout.expect("Failed to open echo stdout");
//
// let mut sed_child = Command::new("sed")
//     .arg("s/tpyo/typo/")
//     .stdin(Stdio::from(echo_out))
//     .stdout(Stdio::piped())
//     .spawn()
//     .expect("Failed to start sed process");
//
// let output = sed_child.wait_with_output().expect("Failed to wait on sed");
// assert_eq!(b"Oh no, a typo!\n", output.stdout.as_slice());
