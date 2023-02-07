//! `macros` is a Rust module for generating macros.
//!
//! It defines the following macros:
//!
//! * [`comment_block`]
//! * [`comment_block_dyn`]
//!
//! The macro `comment_block` generates the start and end marker strings of a comment
//! section in a Markdown file.
//!
//! Macro `comment_block_dyn` make the `SECTION` parameter dynamic by changing the macro
//! definition to accept an additional argument for the section name. pass in the section name at
//! runtime, instead of having it hardcoded in the macro definition.

/// The macro `comment_block` generates the start and end marker strings of a comment
/// section in a Markdown file.
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
/// This macro defines the start and end markers of a comment section in a Markdown file
/// where:
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

/// Macro `comment_block_dyn` macro accepts three arguments:
/// $section_name, $marker, and $section. The $section argument is used to dynamically
/// specify the section name in the generated string. The $section_name and $marker
/// arguments work just like in the previous version.
// Make the SECTION parameter dynamic by changing the macro definition to accept an additional
// argument for the section name. pass in the section name at runtime, instead of having it
// hardcoded in the macro definition.
#[macro_export]
macro_rules! comment_block_dyn {
    ($section_name:expr, $marker:expr, $section:expr) => {
        format!("<!--{}_{}:{}-->", $marker, $section, $section_name)
    };
}
