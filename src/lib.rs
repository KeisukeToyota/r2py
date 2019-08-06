extern crate pyo3;
extern crate reqwest;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use reqwest::header::{RANGE, CONTENT_LENGTH};
use reqwest::Client;

// l = [(self.total_length + i) // self.split_num for i in range(self.split_num)]
// args = [(i, 0 if i == 0 else sum(l[:i]) + 1, sum(l[:i]) + val) for i, val in enumerate(l)]

#[pyfunction]
fn download(url: &str) -> PyResult<()> {
    let client = Client::new();
    let head_resp = client.head(url)
                          .send()
                          .expect("send error...");

    let length = head_resp.headers()
                          .get(CONTENT_LENGTH)
                          .expect("cannot get content-length...");
                          
    let mut res = client.get(url)
                        .header(RANGE, format!("bytes={}-{:?}", 1, length))
                        .send()
                        .expect("hgoe");

    let clients: Vec<Client> = vec![Client::new(); 4];
    let split_num = ((length.to_str().unwrap()).parse::<i32>().unwrap()) / 300000;

    let l: Vec<i32> = (0..split_num).map(|x| ((length.to_str().unwrap()).parse::<i32>().unwrap() + x) / split_num)
                                    .collect();
    let args: Vec<(i32, i32)> = l.iter().enumerate().map(|(i, x)| {
        let s = match i {
            0 => 0,
            _ => (&l[..i]).iter().fold(0, |sum, y| sum + y) + 1
        };
        let e = (&l[..i]).iter().fold(0, |sum, y| sum + y) + x;
        (s, e)
    }).collect();

    println!("{:?}", args);
        
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
