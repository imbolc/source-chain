//! # source-chain
//!
//! Formats StdError with it's source chain
//!
//! ```rust
//! #[derive(Debug, thiserror::Error)]
//! enum Error {
//!     #[error("unknown file {1}")]
//!     UnknownFile(#[source] std::io::Error, &'static str),
//! }
//!
//! fn file_error() -> Result<String, Error> {
//!     let filename = "unknown-file.txt";
//!     std::fs::read_to_string(filename).map_err(|e| Error::UnknownFile(e, filename))
//! }
//!
//! let err = file_error().unwrap_err();
//! assert_eq!(
//!     source_chain::to_string(&err),
//!     "unknown file unknown-file.txt\nCaused by:\n\tNo such file or directory (os error 2)"
//! );
//!
//! let dyn_err: Box<dyn std::error::Error> = Box::new(err);
//! assert_eq!(
//!     // notice dereferencing
//!     source_chain::to_string(&*dyn_err),
//!     "unknown file unknown-file.txt\nCaused by:\n\tNo such file or directory (os error 2)"
//! );
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

/// A helper function to format an error with its source chain.
///
/// This function works with both `&Error` and `Box<dyn Error>`. When passing a boxed error,
/// make sure to dereference it using `&*e`.
pub fn to_string(e: &(dyn std::error::Error + 'static)) -> String {
    use std::fmt::Write as _;

    let mut s = e.to_string();
    let mut current = e.source();
    if current.is_some() {
        s.push_str("\nCaused by:");
    }
    while let Some(cause) = current {
        write!(s, "\n\t{}", cause).ok();
        current = cause.source();
    }
    s
}
