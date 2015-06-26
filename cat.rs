use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::stdout;
use std::env::args;

const BUFFER_SIZE: usize = 2048;

fn main() {
    let paths: Vec<String> = args().skip(1).collect();
    if paths.is_empty() {
        panic!("file name not given");
    }

    let mut writer = stdout();
    for path in paths {
        do_cat(&mut writer, &path);
    }
}

fn do_cat(writer: &mut Write, path: &str) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(&file);
    let mut buf = [0; BUFFER_SIZE];
    loop {
        let n = reader.read(&mut buf).unwrap();
        if n == 0 { break; }
        writer.write_all(&buf[..n]).unwrap();
    }
}

