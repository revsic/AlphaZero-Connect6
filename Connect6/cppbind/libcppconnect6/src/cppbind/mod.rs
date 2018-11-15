//! Rust bindings for C++ interface and impl of AlphaZero policy evaluator
//!
//! It provides `extern "C"` based bindings for some utilities to implement policy `AlphaZero` eval.
//!
pub use self::cppeval::*;
pub use self::rawobj::*;

mod cppeval;
mod rawobj;