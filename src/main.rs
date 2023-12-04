#![allow(dead_code)]
#![allow(unused_variables)]

mod drawer;
mod interpreter;
mod parser;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "usage: {} <source> <destination> <img_width> <img_height>",
            args[0]
        );
        process::exit(1);
    }

    let source = &args[1];
    let destination = &args[2];
    let img_width = args[3].parse::<u32>().unwrap();
    let img_height = args[4].parse::<u32>().unwrap();

    let ast = parser::parse_logo_file(source);
    let cmds = interpreter::evaluate(&ast);
    drawer::draw(destination, cmds, img_width, img_height);
}
