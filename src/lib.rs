mod screentone;

use ndarray::Ix2;
use numpy::{IntoPyArray, PyArray};
use pyo3::prelude::*;
use crate::screentone::create_dot_mask::create_mask;
use crate::screentone::r#enum::TypeDot;

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(signature = (mask_size, dot_size, angle=None, dot_type=None))]
fn screentone_mask(
    mask_size: [usize; 2],
    dot_size: usize,
    angle: Option<f32>,
    dot_type: Option<TypeDot>,
    py: Python<'_>,
) -> Bound<'_, PyArray<f32, Ix2>> {
    let angle = angle.unwrap_or(0.0);
    let mask = py.allow_threads(|| {
        create_mask(mask_size, dot_size,  angle, dot_type)
    });
    mask.into_pyarray(py)

}

/// A Python module implemented in Rust.
#[pymodule]
fn dot_matrix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(screentone_mask, m)?)?;
    m.add_class::<TypeDot>()?;
    Ok(())
}
