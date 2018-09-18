pub use self::support::{create_pypolicy, PyPolicy};

use BOARD_CAPACITY;

use cpython::*; // avoid unused import warning of `PythonObject`, `ToPyObject`
use rand;

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

/// Create random python policy for testing policy AlphaZero
///
/// # Examples
/// ```rust
/// let py_policy = py_policy!();
/// let mut policy = AlphaZero::new(py_policy);
/// ```
#[macro_export]
macro_rules! py_policy {
    () => {{
        let gil = Python::acquire_gil();
        let py = gil.python();
        $crate::macro_def::create_pypolicy(py).unwrap().into_object()
    }}
}

/// Create IoPolicy with stdio
///
/// # Examples
/// ```rust
/// stdio_policy(policy);
/// let result = Agent::new(&mut policy).play();
/// assert!(result.is_ok());
/// ```
#[macro_export]
macro_rules! io_policy_stdio {
    ($policy:ident) => {
        use std;
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let mut $policy = $crate::policy::IoPolicy::new(&mut stdin, &mut stdout);
    }
}