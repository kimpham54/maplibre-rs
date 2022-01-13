//! Errors which can happen in various parts of the library.

#[derive(Debug)]
pub enum Error {
    Network(String),
    File(String),
}
