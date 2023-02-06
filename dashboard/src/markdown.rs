use std::{
    fs,
    fs::{rename, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    time::Instant,
};

use anyhow::anyhow;
// use comrak::{markdown_to_html, ComrakOptions};
use lazy_static::lazy_static;

// use regex::Regex;
use crate::{gh::GitRepoListItem, AppError, Result};

#[macro_export]
macro_rules! comment_block {
    ($section_name:expr, $marker:expr) => {
        format!("<!--{}_SECTION:{}-->", $marker, $section_name)
    };
}

/// Word count limit for description.
pub const DESC_WC: usize = 60;

lazy_static! {
    static ref NEW_CONTENT: String = "* [Google](https://www.google.com) - Search engine\n\
                          * [GitHub](https://github.com) - Web-based hosting service\n\
                          * [Rust Programming Language](https://www.rust-lang.org) - System programming language".to_owned();
}

pub struct RegexMd;

impl RegexMd {
    /// Find a pattern in a file.
    ///
    /// rustc: can't capture dynamic environment in a fn item
    /// use the `|| { ... }` closure form instead
    ///
    /// Note that all of the unwraps are actually OK for this regex because the only way for
    /// the regex to match is if all of the capture groups match. Not true in general though!
    pub fn find_in_text(to_search: &str, section_tag: &str) {
        let re_start = comment_block!(section_tag, "START");
        let re_end = comment_block!(section_tag, "END");

        for (i, line) in to_search.lines().enumerate() {
            log::debug!("i: {}. {}", i, line);
            if line.contains(&re_start) {
                log::debug!("Found start: `{}`", line);
            }
            if line.contains(&re_end) {
                log::debug!("Found end: `{}`", line);
                log::debug!("Exiting");
                break;
            }
        }

        // let start_re = Regex::new(&re_start).unwrap();
        // let end_re = Regex::new(&re_end).unwrap();
        //
        // for caps in start_re.captures_iter(to_search) {
        //     println!("{:?}", caps);
        // }
        // for caps in end_re.captures_iter(to_search) {
        //     println!("{:?}", caps);
        // }
    }

    fn replace() {
        // regex::Replacer::replace_append(&mut self, caps, dst)
    }
}

/// Indicates if visitor is inside markdown start and end section blocks.
enum SectionState {
    OutsideSection,
    InSection,
}

/// This way, the function returns a Result which can be checked by the caller for success or
/// failure. The error type is Box<dyn std::error::Error>, which allows for the caller to handle
/// any type of error that may occur during the execution of the function. The code is more
/// concise and easier to read, with the use of a macro to simplify the comment block strings.
// new_lines.push_str("\n");
// new_file.push_str(NEW_CONTENT.as_str()); new_file.push_str("\n");
// new_file.push_str(&end_section); new_file.push_str("\n");
pub(crate) fn update_markdown_file(
    file_path: &Path,
    section_tag: &str,
    new_content: &str,
) -> Result<(), AppError> {
    let mut section_state = SectionState::OutsideSection;
    let mut new_lines: Vec<String> = Vec::new();

    let mut file: File =
        OpenOptions::new().read(true).write(true).create(true).open(file_path).unwrap();
    let reader = BufReader::new(&file);
    log::debug!("{}", &new_content);

    for line in reader.lines() {
        let line: String = line.unwrap();
        if line.contains(&comment_block!(section_tag, "START")) {
            section_state = SectionState::InSection;
            new_lines.push(line.to_owned());
            new_lines.push(new_content.to_owned());
        } else if line.contains(&comment_block!(section_tag, "END")) {
            section_state = SectionState::OutsideSection;
            new_lines.push(line.to_owned());
        } else if matches!(section_state, SectionState::OutsideSection) {
            new_lines.push(line.to_owned());
        }
    }

    let backup_file_path = format!("{}.bak", file_path.display());
    if Path::new(file_path).exists() {
        rename(file_path, &backup_file_path)
            .map_err(|e| {
                anyhow!(
                    // Error if: * The user lacks permissions to view contents
                    "Failed to rename file `{}` to `{}`: `{e:#?}`",
                    file_path.display(),
                    backup_file_path,
                )
            })
            .unwrap();
        log::info!("Renamed file `{}` to `{backup_file_path}`", file_path.display(),);
    }

    // let mut file = File::create(file_path).unwrap();
    for line in new_lines {
        print!("{}", line);
        writeln!(file, "{}", line).unwrap();
    }

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

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_update_markdown_file() {
        pretty_env_logger::env_logger::builder().filter_level(log::LevelFilter::Info).build();
        let file_path = Path::new("test.md");
        let section_name = "dashboard";
        let new_content = "This is some new content.";
        let expected_content = "This is some new content.\n";

        update_markdown_file(file_path, section_name, new_content).unwrap();
        let file = fs::read_to_string(file_path).unwrap();
        assert_eq!(file, expected_content);

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_update_md_file() {
        pretty_env_logger::env_logger::builder().filter_level(log::LevelFilter::Info).build();
        let file_path = Path::new("test.md");
        let section_name = "dashboard";
        let new_content = "* [Google](https://www.google.com) - Search engine\n\
                          * [GitHub](https://github.com) - Web-based hosting service\n\
                          * [Rust Programming Language](https://www.rust-lang.org) - System programming language";
        let expected_content = "* [Google](https://www.google.com) - Search engine\n\
                          * [GitHub](https://github.com) - Web-based hosting service\n\
                          * [Rust Programming Language](https://www.rust-lang.org) - System programming language\n";

        update_markdown_file(file_path, section_name, new_content).unwrap();
        let file = fs::read_to_string(file_path).unwrap();
        assert_eq!(file, expected_content);

        fs::remove_file(file_path).unwrap();
    }
}

//------------------------------------------------------------------------------

fn bench() {
    let file_path = Path::new("test.md");
    let section_name = "dashboard";
    let new_content = "* [Google](https://www.google.com) - Search engine\n\
                      * [GitHub](https://github.com) - Web-based hosting service\n\
                      * [Rust Programming Language](https://www.rust-lang.org) - System programming language";

    if Path::new(file_path).exists() {
        fs::remove_file(file_path).unwrap();
    }

    let start = Instant::now();
    update_markdown_file(file_path, section_name, new_content).unwrap();
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);

    fs::remove_file(file_path).unwrap();
}
