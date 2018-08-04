extern crate cpython;

#[cfg(test)]
mod tests;

use cpython::*;

fn pyobj_to_vec<'a, T: FromPyObject<'a>>(py: Python, obj: PyObject) -> Option<Vec<T>> {
    let pyseq = match obj.cast_into::<PySequence>(py) {
        Ok(seq) => seq,
        Err(_) => return None,
    };

    let pyiter = match pyseq.iter(py) {
        Ok(iter) => iter,
        Err(_) => return None,
    };

    let mapped = pyiter
        .filter(|x| x.is_ok())
        .filter_map(|x| x.unwrap().extract::<T>(py).ok())
        .collect::<Vec<T>>();
    Some(mapped)
}