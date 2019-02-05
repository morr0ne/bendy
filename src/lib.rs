//! Encodes and decodes bencoded structures.
//!
//! The decoder is explicitly designed to be zero-copy as much as possible, and to not
//! accept any sort of invalid encoding in any mode (including non-canonical encodings)
//!
//! The encoder is likewise designed to ensure that it only produces valid structures.
pub mod decoder;
pub mod encoder {
    pub use super::encoding::*;
}

mod encoding;
mod state_tracker;
mod token;

pub use token::Error;
