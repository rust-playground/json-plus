//! # json_plus
//!
//! This crate provides JSON helper functions beyond Serialization & Deserialization such as `diff`, `merge`, `...`
//!
//! ```rust
//! use json_plus::diff;
//! use serde_json::json;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let old = json!({"key":"old value", "arr":[]});
//!     let new = json!({"key":"new value", "arr":[]});
//!
//!     let diffed = diff(&old, &new).unwrap();
//!     println!("{}", diffed.to_string());
//!     Ok(())
//! }
//! ```

mod diff;
mod merge;
mod strip;

pub use diff::diff;
pub use merge::merge;
pub use strip::{strip, Strip};
