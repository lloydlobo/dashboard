//! `error` is a Rust module for error handling.
//!
//! It defines two error enums: `ParserError` and `PrinterError`.
//!
//! * The `ParserError` enum represents errors that may occur during parsing and has variants such
//!   as: `Io` for I/O errors, `LogicBug` for logic errors, `RegexError` for regex errors, and
//!   `PrinterError` and `AnyhowError` for errors from other libraries.
//! * The `PrinterError` enum represents errors that may occur during printing and has variants such
//!   as `TermcolorError`, `Io`, `BufferError`, and `InvalidColor`.
//! * The `ErrorColor` enum represents different colors that can be used.
//!
//! The module implements conversion trait implementations to convert between various error types.
//!
//! # Examples
//!
//! Example usage of the `ParserError` enum:
//!
//! ```rust
//! use parser::ParserError;
//!
//! fn parse_input() -> Result<(), ParserError> {
//!     // ...some parsing code
//!     return Err(ParserError::LogicBug("error in logic".to_owned()));
//! }
//!
//! fn main() {
//!     if let Err(e) = parse_input() {
//!         let err = format!("Parsing failed: {:?}", e);
//!         assert_eq!(err, r#"Parsing failed: LogicBug("error in logic")"#);
//!     }
//! }
//! ```
//!
//! Example usage of the `PrinterError` enum:
//!
//! ```rust
//! ```

// #[cfg(doctest)]
// use doc_comment::doctest;
// #[cfg(doctest)]
// doctest!("../README.md");

use std::{convert::Into, io, sync::Arc, write};

use regex::Error as RegexError;

/// `Result<T, E>` is an alias for `anyhow::Result` with [`ParserError`] as the error type.
/// It is used as the return type for functions that may fail and return an error.
pub type Result<T> = anyhow::Result<T, ParserError>;

/// The `ParserError` enum represents the different errors that can occur while parsing some input.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    /// An error occurred while performing an I/O operation
    ///
    /// Instead of cloning the `std::io::Error`, we can store the error within the `ParserError` as
    /// an `Arc` (Atomic Reference Counted) smart pointer. This allows for multiple references to
    /// the same error to be stored in different places without having to clone it.
    #[error("I/O error: {0}")]
    Io(#[from] Arc<io::Error>),
    /// An error occurred in the code logic
    #[error("Error in logic: {0}")]
    LogicBug(String),
    /// An error occurred in the regex engine
    #[error("Regex error")]
    RegexError(#[from] regex::Error),
    /// An error occurred in the printer
    #[error("Printer error")]
    PrinterError(#[from] PrinterError),
    /// An error occurred using the anyhow library
    #[error("Anyhow error")]
    AnyhowError(#[from] anyhow::Error),
    /// An error occurred using the crossbeam library
    #[error("Crossbeam scope return result error: {0}")]
    ChannelError(String),
}

impl ParserError {
    /// Returns `true` if the parser error is [`ChannelError`].
    ///
    /// [`ChannelError`]: ParserError::ChannelError
    #[must_use]
    pub fn is_channel_error(&self) -> bool {
        matches!(self, Self::ChannelError(..))
    }
}
// impl FromResidual<result::Result<Infallible, Box<dyn Any + Send>>> for ParserError {
//     fn from_residual(residual: result::Result<Infallible, Box<dyn Any + Send>>) -> Self {
//         todo!()
//     }
// }

/// `PrinterError` enum represents the different errors that can occur while printing some output.
//
// It seems that in the original code, the variant ParserError::Io is constructed with an
// Arc<std::io::Error>, which is an atomic reference counted smart pointer to an
// std::io::Error. Whether or not to use an Arc here depends on the use case.
// If the ParserError type is shared across multiple threads and ParserError::Io is going to be
// passed from one thread to another, then it is necessary to use an Arc to ensure that the
// error is thread-safe.
// However, if ParserError::Io is only used within a single thread, it may be more efficient to
// simply use an std::io::Error without an Arc.
// Additionally, it is worth considering whether the Arc is actually necessary. If the
// std::io::Error type is going to be immediately processed and the error message is never going
// to be accessed again, then using an Arc would be unnecessary overhead.
#[derive(Debug, thiserror::Error)]
pub enum PrinterError {
    /// An error occurred while using the termcolor library
    #[error("Termcolor error")]
    TermcolorError(#[from] termcolor::ColorChoiceParseError),
    /// An error occurred while performing an I/O operation
    #[error("File I/O error")]
    Io(Arc<io::Error>),
    /// An error occurred while using the termcolor buffer
    #[error("Buffer error")]
    BufferError(#[from] termcolor::ParseColorError),
    /// An error occurred with an invalid color
    #[error("InvalidColor error")]
    InvalidColor(ErrorColor),
}

/// The ErrorColor enum represents the different colors that can be used.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum ErrorColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

/// Converts a ParserError from a RegexError.
impl From<ParserError> for RegexError {
    fn from(val: ParserError) -> Self {
        match val {
            ParserError::RegexError(error) => error,
            _ => RegexError::Syntax(String::from("ParserError")),
        }
    }
}

/// Converts an io::Error into a `PrinterError`.
impl From<io::Error> for PrinterError {
    fn from(error: io::Error) -> Self {
        PrinterError::Io(Arc::new(error))
    }
}

/// Converts an io::Error into a `ParserError`.
impl From<io::Error> for ParserError {
    fn from(error: io::Error) -> Self {
        ParserError::Io(Arc::new(error))
    }
}

/// # Example
///
/// ```rust,ignore
/// let result: Result<T, regex::Error> = // some code that returns a Result
/// let parser_error_result = result.map_err(convert_to_parser_error);
/// ```
#[allow(dead_code)]
fn convert_to_parser_error(regex_error: regex::Error) -> ParserError {
    ParserError::RegexError(regex_error)
}

/// This allows any type that implements the Into<ParserError>
/// trait to be used as the error type in the Result type.
//
// fn handle_error<T>(result: Result<T, regex::Error>) -> Result<T, ParserError>
#[allow(dead_code)]
fn handle_error<T>(result: anyhow::Result<T, regex::Error>) -> Result<T>
where
    T: std::fmt::Debug,
    regex::Error: Into<ParserError>,
{
    result.map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_log_logicbug() {
        fn parse_input() -> Result<()> {
            // ...some parsing code
            Err(ParserError::LogicBug("error in logic".to_owned()))
        }
        if let Err(e) = parse_input() {
            let err = format!("Parsing failed: {e:?}");
            assert_eq!(err, r#"Parsing failed: LogicBug("error in logic")"#);
        }
    }

    #[test]
    fn should_log_printererror() {
        fn parse_input() -> Result<()> {
            // ...some parsing code
            Err(ParserError::PrinterError(PrinterError::InvalidColor(ErrorColor::Red)))
        }
        if let Err(e) = parse_input() {
            let err = format!("Parsing failed: {e:?}");
            assert_eq!(err, r#"Parsing failed: PrinterError(InvalidColor(Red))"#);
        }
    }
}
