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
     display_nonblank: bool,
     display_ends:bool,
     file_name: String,
     sqeeze_blank:bool,
}

fn read_file(mut arg:Arguments) -> Result<()>{
    let file = File::open(arg.file_name.as_str())?;
    if arg.display_nonblank {
         for (x,y) in BufReader::new(file).lines()
         .map(|line| line.unwrap())
         .filter(|x| !x.is_empty())
         .enumerate(){
            let args  = arg.clone();
            output(args, x, y)
         }
    } else {
        for (x,y) in BufReader::new(file).lines()
         .map(|line| line.unwrap())
         .enumerate(){
            let args  = arg.clone();
            output(args, x, y)
         }
    }
    Ok(())
}

fn output(arg: Arguments, x:usize, y: String) {
    let mut res = y;
    if arg.display_ends {
        res = res.replace("\n", "$")
    }
    if arg.sqeeze_blank {
        if res.is_empty() {
             res = "$".to_string()
        } else {
             res = res.replace("\n", "$")
        }
    }
    if arg.display_lines {
        println!("{0} {1}", x, res);
    }
    println!("{0} {1}", x, res);
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
                    "-b" => {
                        result.display_nonblank = true;
                    }
                    "-s" => {
                        result.sqeeze_blank = true;
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
