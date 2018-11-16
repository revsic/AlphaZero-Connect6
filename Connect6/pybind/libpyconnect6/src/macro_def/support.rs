use super::*;

use rand;

/// Python random policy
py_class!(pub class PyPolicy |py| {
    def __call__(&self, _turn: PyObject, boards: PyObject) -> PyResult<PyObject> {
        let len = boards.cast_into::<PyList>(py)?.len(py);

        let value = (0..len)
            .map(|_| rand::random::<f32>().to_py_object(py).into_object())
            .collect::<Vec<PyObject>>();
        let value = PyList::new(py, value.as_slice()).into_object();

        let policy = (0..len).map(|_| {
            let rand_policy = (0..BOARD_CAPACITY)
                .map(|_| rand::random::<f32>().to_py_object(py).into_object())
                .collect::<Vec<PyObject>>();
            PyList::new(py, rand_policy.as_slice()).into_object()
        }).collect::<Vec<PyObject>>();

        let policy = PyList::new(py, policy.as_slice()).into_object();
        Ok(PyTuple::new(py, &[value, policy]).into_object())
    }
});

/// py_policy generator
#[allow(dead_code)]
pub fn create_pypolicy(py: Python) -> PyResult<PyPolicy> {
    PyPolicy::create_instance(py)
}
