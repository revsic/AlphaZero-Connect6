//! Rust bindings for C++ interface and impl of AlphaZero policy evaluator
//!
//! It provides `extern "C"` based bindings for some utilities to implement policy and `AlphaZero` evaluator.
//!
pub use self::cpp_policy::*;
pub use self::cpp_support::*;
pub use self::cppeval::*;
pub use self::rawobj::*;

pub mod ffi_test;

mod cpp_policy;
mod cpp_support;
mod cppeval;
mod rawobj;
