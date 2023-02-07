//! # parser
//!
//! ## [`findrepl`]
//!
//! The provided code updates a section of a file by replacing the content between specified start
//! and end markers with a given text. The code opens the file, reads its contents into a buffer,
//! finds the start and end markers, removes the content between them, and inserts the updated text.
//! The updated content is then written back to the file and the function returns a Result.

#![deny(missing_debug_implementations, missing_docs)]

mod error;
mod macros;

pub use crate::{error::*, findrepl::*};

#[allow(dead_code)]
mod printer {
    use std::io::Write;

    use atty::Stream;
    use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

    use crate::error::ParserError;

    pub(crate) fn new() -> Result<(), ParserError> {
        let bufwtr = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut buffer, "green text!")?;
        bufwtr.print(&buffer)?;

        Ok(())
    }

    pub(crate) fn is_stdout_tty(stream: Stream) -> Result<(), ParserError> {
        let bufwtr = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();
        if atty::is(stream) {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            writeln!(&mut buffer, "I'm a terminal")?;
            bufwtr.print(&buffer)?;
        } else {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(&mut buffer, "I'm not a terminal")?;
            bufwtr.print(&buffer)?;
        }

        Ok(())
    }
}

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
        fs,
        io::{Read, Write},
        path::Path,
        sync::Arc,
    };

    use regex::Regex;

    use crate::{comment_block, error::ParserError};

    /// `Marker` is an enumeration of marker values, `Start` and `End`.
    /// These markers are used to indicate the start and end of a [`CommentBlock`] comment block in
    /// some implementation.
    #[derive(Debug, Default, Clone, PartialEq)]
    pub enum Marker {
        /// The end of a comment block section.
        #[default]
        End,
        /// The start of a comment block section.
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
        /// Creates a new [`CommentBlock`].
        pub fn new(section_name: String) -> Self {
            Self {
                section_name: section_name.trim().to_string(),
                marker: (Marker::Start, Marker::End),
            }
        }
        // fn start_marker(&self) -> String { format!("<!--START_SECTION:{}-->", self.section_name)
        // } fn end_marker(&self) -> String { format!("<!--END_SECTION:{}-->", self.section_name) }
    }

    /// `replace` function first opens the file and reads its contents into a string buffer.
    // It then creates a CommentBlock struct with a specified section name and generates the start
    // and end markers from it. The function then finds the positions of the start and end
    // markers in the buffer using the position method. It removes the content between the
    // markers using the clear method, inserts the updated content using the splice method, and
    // updates the file with the new content.
    ///
    /// # Panics
    ///
    /// * File at `path` doesn't exist.
    /// * start or end marker are not found'.
    /// * `path` being passed to the replace function. - Instead of passing input.path which is a
    ///   Path object, you should pass &path, which is the path to the tempfile as a string slice.
    ///
    /// # Errors
    ///
    /// This function will return an error if it panics.
    //
    // PERF: Create regex to find all `section_names` from copied `README.md`.
    pub fn replace(text: &str, block: CommentBlock, path: &Path) -> super::Result<()> {
        // Find the start and end of sections surrounded with comment block.
        let re_start = comment_block!(block.section_name, block.marker.0);
        let re_end = comment_block!(block.section_name, block.marker.1);

        // First copy the existing file into a buffer.
        let mut buf = String::new();
        fs::File::open(path)
            .map_err(|e| ParserError::Io(Arc::new(e)))?
            .read_to_string(&mut buf)
            .map_err(|e| ParserError::Io(Arc::new(e)))?;
        log::debug!("Read and copied file:\n>> {}\n```\n{buf}\n```", path.display());

        // Returns the start and end position of regex section.
        let (n_start, n_end) = get_block_positions(&buf, &re_start, &re_end)
            .map_err(|e| ParserError::RegexError(e.into()))?;

        // Split content into lines and update the section
        let buf_arr: Vec<&str> = buf.lines().collect();
        let mut start = buf_arr[0..=n_start].to_owned();
        let mut end = buf_arr[n_end..].to_owned();
        let mut middle = text.lines().collect();
        // PERF: This can be highly made effecient.
        // Join start, updated middle, and end with new line.
        start.append(&mut middle);
        end.append(&mut vec!["\n"]);
        start.append(&mut end);
        let updated_content: String = start.join("\n");

        // Remove the original file `README.md`
        fs::remove_file(path).map_err(|e| ParserError::Io(Arc::new(e)))?;

        // Create a new file and write the updated content: Write all to new README.md.
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .map_err(|e| ParserError::Io(e.into()))?
            .write_all(updated_content.as_bytes())
            .map_err(|e| ParserError::Io(e.into()))?;

        Ok(())
    }

    /// Returns the line positions of start and end markers for the given section in the buffer.
    ///
    /// ```rust
    /// use parser::*;
    /// fn try_main() {
    ///     let re_start = "<!--START_SECTION:tag_1-->";
    ///     let re_end = "<!--END_SECTION:tag_1-->";
    ///
    ///     let buf: &str = r#"<!--START_SECTION:tag_1-->
    /// * [lorem](https://github.com/username/username) — username's GitHub profile.
    /// * [foobar](https://github.com/username/foobar)
    /// * [bar](https://github.com/username/bar) — Lorem ipsum dolor sit amet, qui minim labore
    ///   adipisicing minim sint cillum sint consectetur cupidatat. ...
    /// <!--END_SECTION:tag_1-->"#;
    ///
    ///     let (start, end) = get_block_positions(buf, &re_start, &re_end).unwrap();
    ///     assert_eq!((start, end), (0, 5));
    /// }
    /// ```
    pub fn get_block_positions(
        buf: &str,
        re_start: &str,
        re_end: &str,
    ) -> super::Result<(usize, usize)> {
        let start = Regex::new(re_start).map_err(ParserError::RegexError)?;
        let end = Regex::new(re_end).map_err(ParserError::RegexError)?;

        let start = start
            .find(buf)
            .ok_or_else(|| regex::Error::Syntax("start marker not found".to_string()))
            .map_err(ParserError::RegexError)?;
        let end = end
            .find(buf)
            .ok_or_else(|| regex::Error::Syntax("end marker not found".to_string()))
            .map_err(ParserError::RegexError)?;
        debug_assert!(start.start() < end.start());

        let start = buf[..start.start()].lines().count();
        let end = buf[..end.start()].lines().count();
        debug_assert!(start < end);

        Ok((start, end))
    }

    #[cfg(test)]
    mod tests {
        use std::{
            fs::File,
            io::{Read, Write},
            path::PathBuf,
        };

        use pretty_assertions::assert_eq;
        use quickcheck::{quickcheck, Arbitrary, Gen};
        use rand::Rng;
        use tempfile::tempdir;

        use super::*;

        #[derive(Debug, PartialEq, Clone, Default)]
        pub struct Input {
            pub text: String,
            pub block: CommentBlock,
        }
        impl Arbitrary for Input {
            fn arbitrary(g: &mut Gen) -> Self {
                Input {
                    text: Arbitrary::arbitrary(g),
                    block: CommentBlock::new(Arbitrary::arbitrary(g)),
                }
            }
        }

        /// Check if the file was updated with new text.
        fn test_if_written(path: &PathBuf, initial_content: &str) -> anyhow::Result<()> {
            let mut f = File::open(path).unwrap();
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            let path = path.to_string_lossy();
            Ok(assert_eq!(
                buf, initial_content,
                "Should write `INITIAL_CONTENT` to tempfile at {path}",
            ))
        }

        // Struct representing a block of text
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct Text {
            content: Box<String>,
        }

        // Implement the `Arbitrary` trait for the `Text` struct String type implements Arbitrary
        // and can be used to generate random strings for the content field of Text.
        impl Arbitrary for Text {
            /// Generate a random string with length between 0 and 100
            fn arbitrary(g: &mut Gen) -> Self {
                let mut rng = rand::thread_rng(); // Create a random number generator
                let len: usize = Gen::new(rng.gen_range(30..100)).size(); // Generate a random length for the content
                let lines: Vec<_> = (0..len).map(|_| String::arbitrary(g)).collect(); // Generate `len` random lines of text
                let content: Box<String> = Box::new(lines.join("\n")); // Join the lines into a single multi-line string

                Text { content }
            }
        }

        //--------------------------------------------------------------------------------

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

        #[test]
        fn quickcheck_get_section_positions() {
            fn prop(text: Text) -> bool {
                let block = CommentBlock::new("tag_1".to_string());
                let quick_fuzz = text.content;
                let buf =
                    format!("<!--START_SECTION:tag_1-->\n{quick_fuzz}\n<!--END_SECTION:tag_1-->");

                let re_start = comment_block!(block.section_name, block.marker.0);
                let re_end = comment_block!(block.section_name, block.marker.1);

                let (start, end) = get_block_positions(&buf, &re_start, &re_end).expect(
                    "Should returns the line positions of start and end markers for the given \
                     buffer.",
                );
                // assert_eq!(Box::new(buf), quick_fuzz);
                start < end
            }

            quickcheck(prop as fn(Text) -> bool);
        }

        #[test]
        fn should_get_section_positions() {
            let block = CommentBlock::new("tag_1".to_string());
            let buf: &str = r#"<!--START_SECTION:tag_1-->
* [lorem](https://github.com/username/username) — username's GitHub profile.
* [foobar](https://github.com/username/foobar)
* [bar](https://github.com/username/bar) — Lorem ipsum dolor sit amet, qui minim labore
  adipisicing minim sint cillum sint consectetur cupidatat. ...
<!--END_SECTION:tag_1-->"#;

            let re_start = comment_block!(block.section_name, block.marker.0);
            let re_end = comment_block!(block.section_name, block.marker.1);
            assert_eq!(re_start, "<!--START_SECTION:tag_1-->");
            assert_eq!(re_end, "<!--END_SECTION:tag_1-->");

            let (start, end) = get_block_positions(buf, &re_start, &re_end).expect(
                "Should returns the line positions of start and end markers for the given buffer.",
            );
            assert!(start < end);
            assert_eq!((start, end), (0, 5));
        }
    }
}
