#![allow(dead_code)]
#![allow(unused_variables)]

mod drawer;
mod interpreter;
mod parser;

use drawer::draw;
use interpreter::evaluate;
use parser::parse_logo_file;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <source> <destination>", args[0]);
        process::exit(1);
    }

    let source = &args[1];
    let destination = &args[2];

    let ast = parse_logo_file(source).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let cmds = evaluate(&ast);
    draw(destination, cmds);
}
