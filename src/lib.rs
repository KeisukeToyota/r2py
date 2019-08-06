extern crate pyo3;
extern crate reqwest;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

#[pyfunction]
fn download(url: &str) -> PyResult<()> {
    let mut res = reqwest::get(url).expect("request failed");
        
    let url_parse: Vec<&str> = url.split('/').collect();

    let file_name = match url_parse.last() {
        Some(name) => name,
        None => panic!("Cannot get file name...")
    };

    let path = Path::new(OsStr::new(file_name));
    let mut file = File::create(path).expect("create failed");
    &res.copy_to(&mut file).expect("");

    Ok(())
}

#[pyfunction]
fn fibonacci(n: usize) -> usize {
    if n < 2 {
      return n; 
    }
    fibonacci(n-1) + fibonacci(n-2) 
}

#[pymodule]
fn r2py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(download))?;
    m.add_wrapped(wrap_pyfunction!(fibonacci))?;

    Ok(())
}
