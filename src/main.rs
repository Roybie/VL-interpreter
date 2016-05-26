pub mod ast;
pub mod commands;
pub mod interpreter;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub use ast::{Ast, Parser};

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide vl source file\n\nUsage: \"vl sourcefile.vl\"");
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    //TODO error checking here
    file.read_to_string(&mut s).unwrap();

    //println!("{}", s);

    //println!("{:?}", ast::parse(s));

    let program = ast::parse(s);
    interpreter::run(program);

}

