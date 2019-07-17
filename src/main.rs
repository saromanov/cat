use std::io::Read;
use std::io::BufRead;
use std::env;
use std::

struct Cat {

}

impl Cat {
    fn version(self) -> &'static str {
        "0.0.1"
    }

    fn read(self, name: &str) {
        
    }
}

struct Arguments {
     stream:bool
     show: bool
}

fn parse_args(args: Vec<String>) -> Option<Arguments> {
    let mut args = Arguments
    if args.len() == 0 {
        args.stream = true
    }
    
    Ok(args)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
