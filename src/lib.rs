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
//! # Compatibility
//!
//! This library aims to support archives used in the following games:
//!
//! * The Elder Scrolls III: Morrowind
//! * The Elder Scrolls IV: Oblivion
//! * The Elder Scrolls V: Skyrim
//! * The Elder Scrolls V: Skyrim Special Edition
//! * The Elder Scrolls V: Skyrim Virtual Reality
//! * Fallout 3
//! * Fallout 4
//!
//! This library does not aim to be forwards-compatible.
//!
//! [creation-engine-wiki]: https://en.wikipedia.org/wiki/Creation_Engine
//! [gamebryo-wiki]: https://en.wikipedia.org/wiki/Gamebryo

#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(anonymous_parameters)]
#![warn(box_pointers)]
#![warn(deprecated_in_future)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(keyword_idents)]
#![warn(macro_use_extern_crate)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(private_doc_tests)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unstable_features)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_labels)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![deny(warnings)]
#![forbid(unsafe_code)]

mod archive;
mod error;
mod parser;

pub use archive::{Archive, Version as ArchiveVersion};
pub use error::{Error, Kind as ErrorKind, Result};
