//! A library for manipulating Bethesda Softworks Archive (`.bsa`, `.ba2`) files.
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! bsa = { git = "https://github.com/CJKay/bsa.git" }
//! ```
//!
//! # Overview
//!
//! `bsa` is a Rust crate for reading and writing Bethesda Softworks Archive (`.bsa`, `.ba2`) files.
//! These files are used by several Bethesda-developed games to package in-game assets like sounds,
//! meshes and textures.
//!
//! Generally speaking, any Bethesda game using [the Gamebryo engine][gamebryo-wiki] or a derivative
//! of it, such as the [Creation Engine][creation-engine-wiki], uses this archive file format to
//! package assets.
//!
//! [creation-engine-wiki]: https://en.wikipedia.org/wiki/Creation_Engine
//! [gamebryo-wiki]: https://en.wikipedia.org/wiki/Gamebryo

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod archive;
mod error;
mod parser;

pub use archive::Archive;
pub use error::{Error, Kind as ErrorKind, Result};
