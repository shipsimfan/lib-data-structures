//! Useful data structures library

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod id;
mod packed_map;
mod slot_map;

pub use id::*;
pub use packed_map::*;
pub use slot_map::*;
