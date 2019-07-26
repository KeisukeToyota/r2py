extern crate pyo3;
extern crate reqwest;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::ffi::OsStr;
use std::fs::File;
use std::io::copy;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[pyfunction]
fn download(url: &str) -> PyResult<()> {
    let mut res = reqwest::get(url).expect("request failed");
    {
        let resp = &res;
        let url_parse: Vec<&str> = {
            *resp.url().path().split('/').collect()
        };

        let file_name = match url_parse.last() {
            Some(l) => l,
            None => "hoge"
        };
        let path = Path::new(OsStr::new(file_name));
        let mut file = File::create(path).expect("create failed");
        *resp.copy_to(&mut file).expect("");
    }


//    copy(&mut res, &mut file).expect("");
//    println!("{:?}", file_name);
    Ok(())
}

#[pymodule]
fn r2py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(download))?;

    Ok(())
}
