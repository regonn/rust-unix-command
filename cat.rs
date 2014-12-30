#![allow(unused_must_use)]
use std::os;
use std::io::{File};
use std::io::stdio::{stdout_raw, stderr};

fn main() {
    let mut args = os::args();
    args.remove(0);
    let mut stderr = stderr();

    if args.len() < 1 {
        stderr.write_str("file name not given\n");
    }
    for path in args.iter() {
        do_cat(path);
    }
}

const BUFFER_SIZE: uint = 2048;

fn do_cat(path: &String) {
    let mut writer = stdout_raw();
    let mut in_buf = [0, .. BUFFER_SIZE];
    let mut reader = File::open(&std::path::Path::new(path));

    loop {
        match reader.read(&mut in_buf) {
            Ok(n) if n != 0 => {
                writer.write(in_buf.slice_to(n)).unwrap();
            },
            _ => break
        }
    }
}
