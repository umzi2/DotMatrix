use pyo3::pyclass;

#[pyclass(eq, eq_int)]
#[derive(PartialEq,Clone,Copy)]
pub enum TypeDot {
    CIRCLE = 0,
    CROSS = 1,
    ELLIPSE = 2,
    LINE = 3,
    INVLINE = 4,
}