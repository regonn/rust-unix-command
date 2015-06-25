use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::stdout;

const BUFFER_SIZE: usize = 2048;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let paths = &args[1..];

    if paths.len() < 1 {
        panic!("file name not given");
    }
    for path in paths {
        do_cat(&path);
    }
}

fn do_cat(path: &str) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(&file);

    let mut buf = [0; BUFFER_SIZE];
    let mut writer = stdout();
    loop {
        let n = reader.read(&mut buf).unwrap();
        if n == 0 { break; }
        writer.write_all(&buf).unwrap();
    }
}

