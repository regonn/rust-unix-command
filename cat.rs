#![allow(unused_must_use)]
use std::env;
use std::io::{stdout, Write, Read, Result};
use std::fs::File;

fn main() {
    let paths = env::args().skip(1);
    if paths.len() < 1 {
        writeln!(&mut std::io::stderr(), "file name not given\n");
    }
    for path in paths {
        let res = do_cat(path);
        if res.is_err() {
            panic!("{}", res.unwrap_err());
        }
    }
}

const BUFFER_SIZE: usize = 2048;

fn do_cat(path: String) -> Result<()> {
    let stdout = stdout();
    let mut handle = stdout.lock();
    let mut in_buf = [0; BUFFER_SIZE];
    let mut reader = try!(File::open(&std::path::Path::new(&path)));

    loop {
        let n = match reader.read(&mut in_buf[..]) {
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        try!(handle.write(&in_buf[0..n]));
    }
}
