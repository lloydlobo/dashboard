//! # parser
//!
//! ## [`findrepl`]
//!
//! The provided code updates a section of a file by replacing the content between specified start
//! and end markers with a given text. The code opens the file, reads its contents into a buffer,
//! finds the start and end markers, removes the content between them, and inserts the updated text.
//! The updated content is then written back to the file and the function returns a Result.

use std::{path::Path, time::Instant};

use anyhow::anyhow;
use findrepl::CommentBlock;
use log::{
    info,
    LevelFilter::{Debug, Info},
};
use pretty_env_logger::env_logger::builder;

/// `Result<T, E>` is an alias for the Result type from the anyhow crate.
/// It is used as the return type for functions that may fail and return an error.
pub type Result<T, E> = anyhow::Result<T, E>;

/// `Error` is an alias for the Error type from the anyhow crate.
/// It is used as the error type for the Result type.
pub type Error = anyhow::Error;

pub use findrepl::*;

pub mod findrepl {
    //! # findrepl
    //!
    //! The code defines a CommentBlock struct to store a section name and its start and end
    //! markers, which can be generated from the section name.
    //! A comment_block_marker macro is defined to generate a marker string for a section with a
    //! specified marker type.
    //!
    //!
    //! An example of how to use the function try_lib_main is provided, which demonstrates how it
    //! can be used to replace the content between start and end markers in a file with a new
    //! text.
    //!
    //! # Example
    //!
    //! * Input - example.txt
    //! ```markdown
    //! # findreplace
    //!
    //! ## List
    //!
    //! <!--START_SECTION:tag_1-->
    //! * [lorem](https://github.com/username/username) — username's GitHub profile.
    //! * [foobar](https://github.com/username/foobar)
    //! * [bar](https://github.com/username/bar) — Lorem ipsum dolor sit amet, qui minim labore
    //!   adipisicing minim sint cillum sint consectetur cupidatat. ...
    //! <!--END_SECTION:tag_1-->
    //! ```
    //!
    //! ```rust
    //! fn main() {
    //!     let path = Path::new("example.txt");
    //!     let text = "* [new_lorem](https://github.com/new_username/new_username) — new_username's GitHub profile.\n\
    //!                 * [new_foobar](https://github.com/new_username/new_foobar)\n\
    //!                 * [new_bar](https://github.com/new_username/new_bar) — Lorem ipsum dolor sit amet, qui minim labore\n\
    //!                   adipisicing minim sint cillum sint consectetur cupidatat.";
    //!     if let Err(error) = try_lib_main(text, &path) {
    //!         println!("Error: {}", error);
    //!     }
    //! }
    //! ```
    //!
    //! The function try_lib_main will then update the file by replacing the content between the
    //! start and end markers <!--START_SECTION:tag_1--> and <!--END_SECTION:tag_1--> with the
    //! string text. #
    //!
    //! * Output:
    //! ```markdown
    //! # findreplace
    //!
    //! ## List
    //!
    //! <!--START_SECTION:tag_1-->
    //! * [new_lorem](https://github.com/new_username/new_username) — new_username's GitHub
    //!   profile.\n\
    //! * [new_foobar](https://github.com/new_username/new_foobar)\n\
    //! * [new_bar](https://github.com/new_username/new_bar) — Lorem ipsum dolor sit amet, qui minim
    //!   labore\n\ adipisicing minim sint cillum sint consectetur cupidatat."; ...
    //! <!--END_SECTION:tag_1-->
    //! ```

    use std::{
        self,
        fmt::Display,
        fs::{self, File, OpenOptions},
        io::{Read, Write},
        path::Path,
    };

    use anyhow::anyhow;
    use regex::Regex;

    use super::{Error, Result};

    /// The macro `comment_block` generates the start and end marker strings of a comment section in
    /// a Markdown file.
    ///
    /// Generates a string in the format of `<!--MARKER_SECTION:SECTION_NAME-->`
    ///
    /// The macro takes in two arguments: `$section_name` and `$marker`.
    /// The first argument, `$section_name`, specifies the name of the comment section.
    /// The second argument, `$marker`, specifies whether it's the start or end marker of the
    /// comment section.
    ///
    /// # Examples
    ///
    /// ```
    /// use comment_block::comment_block;
    /// let section_name = "tag_1";
    /// let marker = "START";
    /// let result = comment_block!(section_name, marker);
    /// assert_eq!(result, "<!--START_SECTION:tag_1-->");
    /// ```
    ///
    /// ```rust
    /// use comment_block::comment_block;
    /// let start_marker = comment_block!("example", "START");
    /// let end_marker = comment_block!("example", "END");
    /// assert_eq!(start_marker, "<!--START_SECTION:example-->");
    /// assert_eq!(end_marker, "<!--END_SECTION:example-->");
    /// ```
    ///
    /// # Internal Notes for Developers
    ///
    /// This macro defines the start and end markers of a comment section in a Markdown file where:
    /// * The start marker is defined by the string `<!--START_SECTION:` concatenated with the
    ///   `$section_name` argument.
    /// * The end marker is defined by the string `<!--END_SECTION:` concatenated with the
    ///   `$section_name` argument.
    #[macro_export]
    macro_rules! comment_block {
        ($section_name:expr, $marker:expr) => {
            format!("<!--{}_SECTION:{}-->", $marker, $section_name)
        };
    }

    // Certainly! You can make the SECTION parameter dynamic by changing the macro definition to
    // accept an additional argument for the section name. This way, you can pass in the section
    // name at runtime, instead of having it hardcoded in the macro definition. Here's an updated
    // version of the macro: In this updated version, the macro accepts three arguments:
    // $section_name, $marker, and $section. The $section argument is used to dynamically specify
    // the section name in the generated string. The $section_name and $marker arguments work just
    // like in the previous version.
    #[macro_export]
    macro_rules! comment_block_dyn {
        ($section_name:expr, $marker:expr, $section:expr) => {
            format!("<!--{}_{}:{}-->", $marker, $section, $section_name)
        };
    }

    /// `Marker` is an enumeration of marker values, `Start` and `End`.
    /// These markers are used to indicate the start and end of a comment block in some
    /// implementation.
    #[derive(Debug)]
    pub enum Marker {
        End,
        Start,
    }

    impl Display for Marker {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Marker::Start => write!(f, "START"),
                Marker::End => write!(f, "END"),
            }
        }
    }

    /// `CommentBlock` is a struct that holds information about a comment block.
    /// It has two fields: `section_name`, which is a `String` representing the name of the section,
    /// and `marker`, which is a tuple of two Marker values, indicating the start and end
    /// markers of the comment block.
    #[derive(Debug)]
    pub struct CommentBlock {
        pub name: String,
        pub marker: (Marker, Marker),
    }

    /// `impl CommentBlock` is an implementation block for the CommentBlock struct.
    ///
    /// It provides a constructor new for creating new CommentBlock values, and two methods:
    /// `start_marker` and `end_marker`, which return the start and end markers as `String` values.
    impl CommentBlock {
        pub(crate) fn new(section_name: &str) -> Self {
            Self { name: section_name.to_string(), marker: (Marker::Start, Marker::End) }
        }

        fn start_marker(&self) -> String {
            format!("<!--START_SECTION:{}-->", self.name)
        }

        fn end_marker(&self) -> String {
            format!("<!--END_SECTION:{}-->", self.name)
        }
    }

    /// `try_main`
    ///
    /// First copy the existing file into a buffer.
    /// Then find the start and end of sections.
    /// Mutate and replace the content btw them with TO_UPDATE_WITH.
    /// Write all to file.
    //
    // PERF: Create regex to find all `section_names` from copied `README.md`.
    //
    // The try_lib_main function first opens the file and reads its contents into a string buffer.
    // It then creates a CommentBlock struct with a specified section name and generates the start
    // and end markers from it. The function then finds the positions of the start and end
    // markers in the buffer using the position method. It removes the content between the
    // markers using the clear method, inserts the updated content using the splice method, and
    // updates the file with the new content.
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * File at `path` doesn't exist.
    /// * start or end marker are not found'.
    pub fn replace(text: &str, block: CommentBlock, path: &Path) -> Result<(), Error> {
        // Find the start and end of sections surrounded with comment block.
        let re_start = comment_block!(block.name, block.marker.0);
        let re_end = comment_block!(block.name, block.marker.1);

        // First copy the existing file into a buffer.
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        log::info!("Read and copied file:\n>> {}\n```\n{buf}\n```", path.display());

        // Returns the start and end position of regex section
        let (n_start, n_end) = get_section_positions(&buf, &re_start, &re_end)?;
        // let (n_start, n_end) = (get_pos(&buf, &re_start)?, get_pos(&buf, &re_end)?);

        // Split content into lines and update the section
        let buf_arr: Vec<&str> = buf.lines().collect();
        let mut start = buf_arr[0..=n_start].to_owned();
        let mut end = buf_arr[n_end..].to_owned();
        let mut middle = text.lines().collect();
        start.append(&mut middle);
        start.append(&mut end);

        // Join start, updated middle, and end with new line.
        let updated_content: String = start.join("\n");

        // Remove the original file `README.md`
        fs::remove_file(path)?;

        // Create a new file and write the updated content: Write all to new README.md.
        let mut f = OpenOptions::new().create(true).write(true).open(path)?;
        f.write_all(updated_content.as_bytes())?;

        Ok(())
    }

    /// Returns the line positions of start and end markers for the given section in the buffer.
    fn get_section_positions(
        buf: &str,
        re_start: &str,
        re_end: &str,
    ) -> Result<(usize, usize), Error> {
        let start_re = Regex::new(&format!(r"{}", re_start))?;
        let end_re = Regex::new(&format!(r"{}", re_end))?;

        let start = start_re.find(buf).ok_or_else(|| anyhow!("start marker not found"))?;
        let end = end_re.find(buf).ok_or_else(|| anyhow!("end marker not found"))?;

        let n_start = buf[..start.start()].lines().count();
        let n_end = buf[..end.start()].lines().count();

        Ok((n_start, n_end))
    }

    /// Returns the line position of `re` in `buf`.
    fn get_pos(buf: &str, re: &str) -> Result<usize, Error> {
        buf.lines()
            .position(|line| line.contains(&re))
            .ok_or_else(|| anyhow!("start marker not found"))
    }
}

// let mut capture: Vec<(usize, &str)> = Vec::new();
// let ln_start = capture[0].0; let ln_end = capture[1].0;
// Get position of the regex start and end comments.
// for (i, line) in buf.lines().enumerate() {
//     if line.contains(&re_start) { capture.push((i, line)); }
//     if line.contains(&re_end) { capture.push((i, line)); break; }
// }
//
// Mutate and replace the content between them with new content.
// let mut start = buf_arr[0..=capture[0].0].to_owned();
// let mut end = buf_arr[capture[1].0..].to_owned();
// let mid = buf_arr[capture[0].0..capture[1].0].to_owned();

mod tests {
    use super::*;

    const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
    const README_PATH: &str = "README.md";
    const TO_UPDATE_WITH: &str = r#"* [d](...) - ...
 * [e](...) - ...
 * [a](...) - ...
 * [b](...) - ...
 * [a](...) - ...
 * [f](...) - ..."#;

    pub(crate) fn run_main() -> Result<(), Error> {
        builder().filter_level(Info).filter_level(Debug).init();
        let start = Instant::now();
        let section_name = &"tag_1";
        let comment_block = CommentBlock::new(section_name);

        // match crate_lib::try_lib_main(TO_UPDATE_WITH, Path::new(README_PATH)) {
        match findrepl::replace(TO_UPDATE_WITH, comment_block, Path::new(README_PATH)) {
            Ok(_) => info!("Finished successfully in {:#.2?}\n\n", start.elapsed()),
            Err(err) => {
                let err = anyhow!("{err}");
                log::error!("{err}");
                return Err(err);
            }
        };

        Ok(())
    }
}
