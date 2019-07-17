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

}

fn parse_args(args: Vec<String>) -> Option<Arguments> {
    
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
