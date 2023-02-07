//! # parser
//!
//! ## [`findrepl`]
//!
//! The provided code updates a section of a file by replacing the content between specified start
//! and end markers with a given text. The code opens the file, reads its contents into a buffer,
//! finds the start and end markers, removes the content between them, and inserts the updated text.
//! The updated content is then written back to the file and the function returns a Result.

pub mod error;

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
        path::{Path, PathBuf},
    };

    use anyhow::anyhow;
    use regex::Regex;

    use super::Result;
    use crate::error::{Error, ParserError};

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

    // impl<'a> Default for Input<'a> {
    //     fn default() -> Self {
    //         Self {
    //             text: Default::default(),
    //             block: Default::default(),
    //             path: Path::new("README.md"),
    //         }
    //     }
    // }

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
    #[derive(Debug, Default, Clone, PartialEq)]
    pub enum Marker {
        #[default]
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
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct CommentBlock {
        section_name: String,
        marker: (Marker, Marker),
    }

    /// `impl CommentBlock` is an implementation block for the CommentBlock struct.
    ///
    /// It provides a constructor new for creating new CommentBlock values, and two methods:
    /// `start_marker` and `end_marker`, which return the start and end markers as `String` values.
    impl CommentBlock {
        pub fn new(section_name: String) -> Self {
            Self {
                section_name: section_name.trim().to_string(),
                marker: (Marker::Start, Marker::End),
            }
        }

        fn start_marker(&self) -> String {
            format!("<!--START_SECTION:{}-->", self.section_name)
        }

        fn end_marker(&self) -> String {
            format!("<!--END_SECTION:{}-->", self.section_name)
        }

        // pub(crate) fn arbitrary(g: &mut quickcheck::Gen) -> CommentBlock {
        //     todo!()
        // }
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
    /// * `path` being passed to the replace function. - Instead of passing input.path which is a
    ///   Path object, you should pass &path, which is the path to the tempfile as a string slice.
    pub fn replace(text: &str, block: CommentBlock, path: &Path) -> Result<(), ParserError> {
        // Find the start and end of sections surrounded with comment block.
        let re_start = comment_block!(block.section_name, block.marker.0);
        let re_end = comment_block!(block.section_name, block.marker.1);

        // First copy the existing file into a buffer.
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        log::info!("Read and copied file:\n>> {}\n```\n{buf}\n```", path.display());

        // Returns the start and end position of regex section
        let (n_start, n_end) = match get_section_positions(&buf, &re_start, &re_end) {
            Ok(it) => it,
            Err(err) => return Err(ParserError::ReplaceError(anyhow!(err).to_string())),
        };
        // let (n_start, n_end) = (get_pos(&buf, &re_start)?, get_pos(&buf, &re_end)?);

        // Split content into lines and update the section
        let buf_arr: Vec<&str> = buf.lines().collect();
        let mut start = buf_arr[0..=n_start].to_owned();
        let mut end = buf_arr[n_end..].to_owned();
        let mut middle = text.lines().collect();
        start.append(&mut middle);
        end.append(&mut vec!["\n"]);
        start.append(&mut end);

        // Join start, updated middle, and end with new line.
        let updated_content: String = start.join("\n");

        // Remove the original file `README.md`
        fs::remove_file(path)?;

        // Create a new file and write the updated content: Write all to new README.md.
        let mut f = OpenOptions::new().create(true).write(true).open(path)?;

        if let Err(e) = f.write_all(updated_content.as_bytes()) {
            return Err(ParserError::WriteError(e));
        }

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

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{Read, Write},
        path::{Path, PathBuf},
    };

    use pretty_assertions::assert_eq;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use tempfile::tempdir;

    use super::*;

    const INITIAL_CONTENT: &str = r#"# README Test

This is a dashboard to display all users projects.

<!--START_SECTION:tag_1-->
<!--END_SECTION:tag_1-->

# LICENSE

Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."#;

    const TO_UPDATE_WITH: &str = r#"* [d](...) - ...
 * [e](...) - ...
 * [a](...) - ...
 * [b](...) - ...
 * [a](...) - ...
 * [f](...) - ..."#;

    #[derive(Debug, PartialEq, Clone, Default)]
    pub struct Input {
        pub text: String,
        pub block: CommentBlock,
    }
    impl Arbitrary for Input {
        /// Return an arbitrary value.
        ///
        /// Gen represents a PRNG. It is the source of randomness from which QuickCheck will
        /// generate values. An instance of `Gen` is passed to every invocation of
        /// `Arbitrary::arbitrary`, which permits callers to use lower level RNG routines to
        /// generate values.
        fn arbitrary(g: &mut Gen) -> Self {
            Input {
                text: Arbitrary::arbitrary(g),
                block: CommentBlock::new(Arbitrary::arbitrary(g)),
            }
        }
    }

    //--------------------------------------------------------------------------------

    #[test]
    fn should_replace() {
        let input = Input {
            text: TO_UPDATE_WITH.to_string(),
            block: CommentBlock::new("tag_1".to_string()),
            // path: Path::new("README.md").to_path_buf(),
        };

        // Initialize temp files.
        let dir = tempdir().unwrap();
        let path = dir.path().join("README.md");

        // Write INITIAL_CONTENT to tempfile.
        let mut f = File::create(&path).unwrap();
        f.write_all(INITIAL_CONTENT.as_bytes()).unwrap();
        test_if_written(&path, INITIAL_CONTENT)
            .expect("Should write and match the initial content");

        let result = replace(&input.text, input.block, &path);
        assert_eq!(result.is_ok(), true, "Should replace text with `parser::findrepl`");

        // Check if the file was updated with new text.
        let mut f = File::open(&path).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!(buf.contains(&input.text), true);
    }

    // This test will generate random Input values & call the prop function with each value. If the
    // function returns false for any value, quickcheck will stop & print the failing input value.
    // #[test]
    // fn quickcheck_should_replace() {
    //     fn prop(input: Input) -> bool {
    //         // Initialize temp files.
    //         let dir = tempdir().unwrap();
    //         let path: PathBuf = dir.path().join("README.md");
    //         let mut f = File::create(&path).unwrap();
    //
    //         // Write INITIAL_CONTENT to tempfile.
    //         f.write_all(INITIAL_CONTENT.as_bytes()).unwrap();
    //         test_if_written(&path, INITIAL_CONTENT)
    //             .expect("Should write and match the initial content");
    //
    //         let result = replace(&input.text, input.block, path.as_path());
    //         assert!(result.is_ok());
    //
    //         // Check if the file was updated with new text.
    //         let mut f = File::open(&path).unwrap();
    //         let mut buf = String::new();
    //         f.read_to_string(&mut buf).unwrap();
    //         buf.contains(&input.text)
    //     }
    //
    //     quickcheck(prop as fn(Input) -> bool);
    // }

    /// Check if the file was updated with new text.
    fn test_if_written(path: &PathBuf, initial_content: &str) -> anyhow::Result<()> {
        let mut f = File::open(path).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        let path = path.to_string_lossy();
        Ok(
            assert_eq!(
                buf, initial_content,
                "Should write `INITIAL_CONTENT` to tempfile at {path}",
            ),
        )
    }
}
