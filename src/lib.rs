extern crate pyo3;
extern crate reqwest;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::thread;
use reqwest::header::{RANGE, CONTENT_LENGTH};
use reqwest::Client;
use std::borrow::Cow;


struct Arg<'a> {
    index: usize,
    range: Cow<'a, str>,
}

// WIP

#[pyfunction]
fn download(url: &'static str) -> PyResult<()> {
    let head_client = Client::new();
    let head_resp = head_client.head(url)
                          .send()
                          .expect("send error...");

    let length = head_resp.headers()
                          .get(CONTENT_LENGTH)
                          .expect("cannot get content-length...");

    let mut res = head_client.get(url)
                        .header(RANGE, format!("bytes={}-{:?}", 0, length))
                        .send()
                        .expect("hgoe");

    // let clients: Vec<Client> = vec![Client::new(); 4];
    let split_num = ((length.to_str().unwrap()).parse::<i32>().unwrap()) / 300000;

    let hoge: Vec<i32> = (0..split_num).map(|x| ((length.to_str().unwrap()).parse::<i32>().unwrap() + x) / split_num)
                                    .collect();
    let args: Vec<Arg> = (&hoge).into_iter().enumerate().map(|(i, x)| {
        let s = match i {
            0 => 0,
            _ => (&hoge[..i]).iter().fold(0, |sum, y| sum + y) + 1
        };
        let e = (&hoge[..i]).iter().fold(0, |sum, y| sum + y) + x;
        let tmp = format!("bytes={}-{}", s, e);
        Arg { index: i, range: Cow::Owned(tmp) }
    }).collect();

    // let (l, r) = args.split_at(args.len() / 2);
    // let (ll, lr) = &l.split_at(&l.len() / 2);
    // let (rl, rr) = &r.split_at(&r.len() / 2);

    // for &arguments in [ll, lr, rl, rr].iter() {
    //     thread::spawn(move || {
    //         for arg in arguments.iter() {
    //             let client = Client::new();
    //             let mut res = client.get(url).header(RANGE, format!("{}", arg.range)).send().expect("hoge");
    //             let tmp = format!("{}.tmp", arg.index);
    //             let path = Path::new(&tmp);
    //             let mut file = File::create(path).expect("create failed");
    //             &res.copy_to(&mut file).expect("hoge");
    //         }
    //     });
    // }

    // thread::spawn(move || {
    //     for arg in ll {
    //         let client = Client::new();
    //         let mut res = client.get(url).header(RANGE, format!("{}", arg.range)).send().expect("hoge");
    //         let tmp = format!("{}.tmp", arg.index);
    //         let path = Path::new(&tmp);
    //         let mut file = File::create(path).expect("create failed");
    //         &res.copy_to(&mut file).expect("hoge");
    //     }
    // });

    // thread::spawn(move || {
    //     for arg in lr {
    //         let client = Client::new();
    //         let mut res = client.get(url).header(RANGE, format!("{}", arg.range)).send().expect("hoge");
    //         let tmp = format!("{}.tmp", arg.index);
    //         let path = Path::new(&tmp);
    //         let mut file = File::create(path).expect("create failed");
    //         &res.copy_to(&mut file).expect("hoge");
    //     }
    // });

    // thread::spawn(move || {
    //     for arg in rl {
    //         let client = Client::new();
    //         let mut res = client.get(url).header(RANGE, format!("{}", arg.range)).send().expect("hoge");
    //         let tmp = format!("{}.tmp", arg.index);
    //         let path = Path::new(&tmp);
    //         let mut file = File::create(path).expect("create failed");
    //         &res.copy_to(&mut file).expect("hoge");
    //     }
    // });

    // thread::spawn(move || {
    //     for arg in rr {
    //         let client = Client::new();
    //         let mut res = client.get(url).header(RANGE, format!("{}", arg.range)).send().expect("hoge");
    //         let tmp = format!("{}.tmp", arg.index);
    //         let path = Path::new(&tmp);
    //         let mut file = File::create(path).expect("create failed");
    //         &res.copy_to(&mut file).expect("hoge");
    //     }
    // });

    // for (&client, &arg) in clients.iter().zip(split_args.iter()) {
    //     thread::spawn(move || {
    //         for &a in arg {
    //             let mut res = client.get(url).header(RANGE, a.1).send().expect("hoge");
    //             let path = Path::new(&format!("{}.tmp", a.0));
    //             let mut file = File::create(path).expect("create failed");
    //             &res.copy_to(&mut file).expect("hoge");
    //         }
    //     });
    // }

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
