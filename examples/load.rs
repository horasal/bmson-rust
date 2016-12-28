extern crate bmson;

use bmson::*;

use std::io::Read;
use std::fs::File;

use std::env;

fn main() {
    let arg = env::args().collect::<Vec<_>>();
    let mut s = String::new();
    let mut f = File::open(&arg[1]).unwrap();
    f.read_to_string(&mut s);
    println!("{}", Bmson::new(&s).unwrap());
}
