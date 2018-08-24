extern crate cpython;
extern crate rand;

pub use self::support::{create_pypolicy, PyPolicy};

use super::BOARD_CAPACITY;
use cpython::*; // avoid unused import warning of `PythonObject`, `ToPyObject`

mod support;

/// Panic with given error string if given Result is Err
///
/// # Examples
/// ```rust
/// let pyseq = must!(obj.cast_into::<PySequence>(py), "couldn't cast obj into PySequence");
/// ```
#[macro_export]
macro_rules! must {
    ($e:expr, $err:expr) => {
        match $e {
            Ok(obj) => obj,
            Err(e) => panic!("{} : {:?}", $err, e),
        }
    }
}

#[macro_export]
macro_rules! py_policy {
    () => {{
        let gil = Python::acquire_gil();
        let py = gil.python();
        $crate::macro_def::create_pypolicy(py).unwrap().into_object()
    }}
}