use std::write;

use thiserror::Error;

/// `Error` is an alias for the Error type from the anyhow crate.
/// It is used as the error type for the Result type.
pub type Error = anyhow::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to write to file: {0}")]
    WriteError(#[from] std::io::Error),
    #[error("Failed to replace the text in the file: {0}")]
    ReplaceError(String),
}

impl ParserError {
    pub fn replace_error(text: &str) -> Self {
        ParserError::ReplaceError(text.to_string())
    }
}

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
//         match replace(&input.text, input.block, path.as_path()) {
//             Ok(_) => {}
//             Err(e) => {
//                 println!("Error: {:?}", e);
//                 return false;
//             }
//         }
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
