//! Mutable global variables for Rust. No unsafe code in
//! this crate. Really slow and gross, but better than
//! nothing.
//!
//! Currently only sync global variables are provided.
//! Future enhancements may include thread_local variables.

pub mod sync;
