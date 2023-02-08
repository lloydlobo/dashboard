//! `util` contains common utility functions and items. YAGNI!
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

#![allow(dead_code)]

use std::path::Path;

use regex::Regex;

use crate::app::AppError;

/// `replace_file_extension`
///
/// In this function, we first create a Path from the file_path. Then, we get the file stem of
/// the path using file_stem() and convert it to a &str using to_str(). Finally, we return a
/// new String created using the format! macro that consists of the stem, a dot (.), and the
/// new_extension.
pub(crate) fn replace_file_extension(file_path: &str, new_extension: &str) -> String {
    let path = Path::new(file_path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let new_file_path = format!("{stem}.{new_extension}");
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
    let re = Regex::new(r"(?i)\.[^./]+$").map_err(AppError::RegexError).unwrap();
    re.replace(file_path, &format!(".{new_extension}")).to_string()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use rand::{
        distributions::{Alphanumeric, DistString},
        Rng,
    }; // use claim::assert_err; use fake::{faker::internet::en::SafeEmail, Fake};

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct OnlyLowercaseLetters(String);

    impl Arbitrary for OnlyLowercaseLetters {
        fn arbitrary(g: &mut Gen) -> Self {
            let len_min = 3;
            let mut rng = &mut &mut rand::thread_rng();
            let len = rng.gen_range(len_min..g.size().min(len_min + 1));
            let string = Alphanumeric.sample_string(&mut rng, len);
            assert_eq!(string.len(), len);
            OnlyLowercaseLetters(string)
        }
    }

    #[test]
    fn should_generate_random_data() {
        fn prop(text: OnlyLowercaseLetters) -> bool {
            let got = format!("filename.{}", text.0);
            let got = replace_extension_regex(&got, "md");
            let expect = "filename.md";
            assert_eq!(got, expect);
            got.contains(expect)
        }

        quickcheck(prop as fn(OnlyLowercaseLetters) -> bool);
    }
}
