#![allow(dead_code)]
#![allow(unused_variables)]

mod drawer;
mod interpreter;
mod parser;

use drawer::{draw, DrawCmd};
use interpreter::evaluate;
use parser::parse_logo_file;
use std::fs;

fn main() {
    let ast = parse_logo_file("input.logo").unwrap();
    dbg!(&ast);
    let cmds = evaluate(&ast);
    // dbg!(&cmds);
    draw("image.svg", cmds);

    // let cmds = vec![
    //     DrawCmd::Forward(100.0),
    //     DrawCmd::LeftTurn(90.0),
    //     DrawCmd::Forward(100.0),
    //     DrawCmd::RightTurn(45.0),
    //     DrawCmd::SetColor("red".to_string()),
    //     DrawCmd::Back(100.0),
    // ];
    // draw("image.svg", cmds);
}
