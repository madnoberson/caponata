#![doc = include_str!("../README.md")]

#[cfg(feature = "animation")]
pub mod animation;
pub mod text;

#[cfg(feature = "animation")]
pub use animation::*;
pub use text::*;
