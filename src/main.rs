use std::io::Read;
use std::io::BufRead;
use std::env;
use std::fs;
use std::io::{BufReader, Write, stdin, stdout,Result};
use std::mem;
use std::io;
use std::fs::File;

#[derive(Default,Clone)]
struct Arguments {
     stream:bool,
     show: bool,
     display_lines: bool,
     non_blank: bool,
     display_ends:bool,
     file_name: String
}

fn read_file(mut arg:Arguments) -> Result<()>{
    let file = File::open(arg.file_name.as_str())?;
    for (x,y) in BufReader::new(file).lines().enumerate(){
        let args  = arg.clone();
        output(args, x, y?)
    }
    Ok(())
}

fn output(arg: Arguments, x:usize, y: String) {
    let mut res = y;
    if arg.display_ends {
        res = res.replace("\n", "$")
    }
    if arg.display_lines {
        println!("{0} {1}", x, res);
    }
    println!("{0}", res);
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
                    "-e" => {
                        result.display_ends = true;
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
            if data.stream {
                while true {
                    let mut s=String::new();
                    let _=stdout().flush();
                    stdin().read_line(&mut s).expect("Did not enter a correct string");
                    if let Some('\n')=s.chars().next_back() {
                        s.pop();
                    }
                    if let Some('\r')=s.chars().next_back() {
                        s.pop();
                    }
                    println!("{}", s)
                }
            }
            read_file(data);
        }
        None => println!("Unable to parse arguments")
    }
}
