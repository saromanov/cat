use std::io::Read;
use std::io::BufRead;
use std::env;

struct Cat;

impl Cat {
    fn version(self) -> &'static str {
        "0.0.1"
    }

    fn read(self, name: &str) {
        
    }
}
#[derive(Default)]
struct Arguments {
     stream:bool,
     show: bool
}

fn parse_args(args: Vec<String>) -> Option<Arguments> {
    let mut result = Arguments{..Default::default()};
    if args.len() == 0 {
        result.stream = true
    }
    
   Some(result)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
