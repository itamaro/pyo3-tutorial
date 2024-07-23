use pyo3::prelude::*;
use std::{fs::File, io::Read};

#[pyfunction]
#[pyo3(signature = (name, conference="the conference".to_string()))]
fn say_hello(name: String, conference: String) -> PyResult<String> {
    Ok(format!("Hello {}, welcome to {}", name, conference))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Give
#[pyfunction]
fn check_reg(filename: String, name: String) -> PyResult<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    if content.contains(&name) {
        Ok("You are registered".to_string())
    } else {
        Ok("Sorry, you are not in the list".to_string())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    Ok(())
}
