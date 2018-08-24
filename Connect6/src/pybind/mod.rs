//! Rust bindings for python interface and impl of AlphaZero policy
//!
//! It provides `rust-cpython` based bindings of some utilities to implement policy `AlphaZero`.
//!
pub use self::pybind_impl::*;

mod pybind_impl;
