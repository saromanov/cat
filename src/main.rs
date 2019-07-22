use std::io::Read;
use std::io::BufRead;
use std::env;
use std::fs;
use std::io::{BufReader, Write};
use std::mem;

#[derive(Default)]
struct Arguments {
     stream:bool,
     show: bool,
}

fn read_file(path:&str){
    let mut f = BufReader::new(fs::File::open(path).unwrap());
    let mut b: [u8; 4] = unsafe { mem::uninitialized() };
    for _ in 0 .. 100_000_000 {
        f.read_exact(&mut b).unwrap();
    }
}
fn parse_args(args: Vec<String>) -> Option<Arguments> {
    let mut result = Arguments{..Default::default()};
    let size = args.len();
    match size {
        1 => result.stream = true,
        2 => {
            match args[1] {
                "-v" => version(),
                _ => result.show = true
            }
        }
        _ => return None
    }
    Some(result)
}

fn version() -> &'static str {
    "0.0.1"
}

fn main() {
    let result = parse_args(env::args().collect());
    match result {
        Some(data) => {
            if data.show {
                read_file()
            }
        }
        None => println!("Unable to parse arguments")
    }
}
