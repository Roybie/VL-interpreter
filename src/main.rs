pub mod ast;
pub mod commands;
pub mod interpreter;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub use ast::{Ast, Parser};

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut string = false;
    let mut s = String::new();
    let mut program:Ast;
    if args.len() < 2 {
        panic!("Please provide vl source file\n\nUsage: \"vl sourcefile.vl\"");
    }

    let stdout = std::io::stdout();
    let mut writer = stdout.lock();

    if &args[1] == "-r" {
        //repl mode!
        println!("VL v.0.3alpha");
        println!("ctrl-c to exit");
        let mut input:String;
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let inp = input.trim().to_owned() + "\n";
                    program = ast::parse(inp);
                    interpreter::run(program, &mut writer);
                },
                _ => (),
            }
        }
    }
    if args.len() == 3 {
        if &args[1] != "-s" {
            panic!("Invalid arguments");
        }
        string = true;
        s = args[2].to_owned() + "\n";
    }

    if !string {
        let path = Path::new(&args[1]);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        //TODO error checking here
        file.read_to_string(&mut s).unwrap();
    }

    //println!("{}", s);
    program = ast::parse(s);
    //println!("{:?}", program);
    interpreter::run(program, &mut writer);

}

