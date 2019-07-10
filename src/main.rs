use std::io::Read;
use std::io::BufRead;
use std::env;

struct cat {

}

impl cat {
    fn version(self) -> &'static str {
        return "0.0.1"
    }

    fn read(self, name: &str) {

    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
