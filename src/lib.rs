use pyo3::exceptions::PyFileNotFoundError;
use pyo3::prelude::*;
use std::{collections::HashMap, fs::File, io::Read};

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
    let res = File::open(filename);
    match res {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            if content.contains(&name) {
                Ok("You are registered".to_string())
            } else {
                Ok("Sorry, you are not in the list".to_string())
            }
        }
        Err(err) => Err(PyFileNotFoundError::new_err(err)),
    }
}

/// Count number of attendees in the list
#[pyfunction]
fn count_attendees(att_list: Vec<String>) -> PyResult<usize> {
    Ok(att_list.len())
}

/// Calculate average travel budget over {attendee -> budget} dictionary
#[pyfunction]
fn travel_avg(budget_dict: HashMap<String, f32>) -> PyResult<f32> {
    let mut sum: f32 = 0.0;
    let mut count = 0;
    for (_, budget) in budget_dict {
        sum += budget;
        count += 1;
    }
    Ok(sum / (count as f32))
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_attendees, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    Ok(())
}
