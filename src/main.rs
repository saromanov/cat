use std::io::Read;
use std::io::BufRead;
use std::env;
use std::fs;
use std::io::{BufReader, Write, stdin, stdout};
use std::mem;
use std::io;

#[derive(Default)]
struct Arguments {
     stream:bool,
     show: bool,
     display_lines: bool,
     file_name: String
}

fn read_file(arg:Arguments){
  let contents = fs::read_to_string(arg.file_name.as_str())
        .expect("Something went wrong reading the file");
  println!("{:?}", contents);
}

fn input_data() {
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
}
fn parse_args(args: Vec<String>) -> Option<Arguments> {
    let mut result = Arguments{..Default::default()};
    let size = args.len();
    match size {
        1 => result.stream = true,
        _ => {
            for d in args {
                match d.as_str() {
                    "-v" => {
                        version();
                    }
                    "-n" => {
                        result.display_lines = true;
                    }
                     _ => {
                        result.show = true;
                        result.file_name = d.to_string();
                    }
                };
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
                read_file(data)
            }
            if data.stream {

            }
        }
        None => println!("Unable to parse arguments")
    }
}
