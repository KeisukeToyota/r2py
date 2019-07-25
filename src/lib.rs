extern crate pyo3;
extern crate reqwest;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[pyfunction]
fn download(url: &str) -> PyResult<()> {
    let client = reqwest::Client::new();
    let mut res = client.get(url).send().unwrap();

    let mut body: Vec<u8> = vec![];
    res.read_to_end(&mut body).unwrap();
    let mut f = File::create("foo.jpg").unwrap();
    f.write_all(&body);

    Ok(())
}

#[pymodule]
fn r2py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(download))?;

    Ok(())
}
