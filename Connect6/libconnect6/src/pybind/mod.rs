//! Rust bindings for python interface and impl of AlphaZero policy evaluator
//!
//! It provides `rust-cpython` based bindings for some utilities to implement policy `AlphaZero`.
//!
pub use self::pybind_impl::*;
pub use self::pyeval::*;

mod pybind_impl;
mod pyeval;
