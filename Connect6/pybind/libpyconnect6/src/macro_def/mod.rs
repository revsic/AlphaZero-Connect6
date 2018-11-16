//! Macro definition
pub use self::support::{create_pypolicy, PyPolicy};

use connect6::BOARD_CAPACITY;
use cpython::*; // avoid unused import warning of `PythonObject`, `ToPyObject`

mod support;

/// Panic with given error string if given Result is Err
#[macro_export]
macro_rules! must {
    ($e:expr, $err:expr) => {
        match $e {
            Ok(obj) => obj,
            Err(e) => panic!("{} : {:?}", $err, e),
        }
    };
}

/// Create random python policy for testing AlphaZero
///
/// # Examples
/// ```rust
/// # extern crate cpython;
/// # extern crate connect6;
/// # #[macro_use] extern crate pyconnect6;
/// # use connect6::policy::AlphaZero;
/// # use pyconnect6::pybind::PyEval;
/// let py_policy = py_policy!();
/// let mut policy = AlphaZero::new(py_policy);
/// ```
#[macro_export]
macro_rules! py_policy {
    () => {{
        use cpython::{Python, PythonObject};
        use $crate::pybind::PyEval;

        let gil = Python::acquire_gil();
        let py = gil.python();
        let obj = $crate::macro_def::create_pypolicy(py)
            .unwrap()
            .into_object();
        Box::new(PyEval::new(obj))
    }};
}
