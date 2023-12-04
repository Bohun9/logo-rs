use std::collections::HashMap;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle, Text};
use svg::Document;

const IMG_WIDTH: i32 = 1920;
const IMG_HEIGHT: i32 = 1080;
const CENTER_X: i32 = IMG_WIDTH / 2;
const CENTER_Y: i32 = IMG_HEIGHT / 2;

#[derive(Debug, PartialEq)]
pub enum DrawCmd {
    Forward(f64),
    Back(f64),
    LeftTurn(f64),
    RightTurn(f64),
    SetColor(String),
    ClearScreen,
    PenUp,
    PenDown,
    Label(String),
    SetFontSize(f64),
    SetTurtle(i32),
}

struct Turtle {
    x: f64,
    y: f64,
    color: String,
    rotation: f64,
    pendown: bool,
    font_size: f64,
}

pub fn draw(destination: &str, mut cmds: Vec<DrawCmd>) {
    cmds.insert(0, DrawCmd::ClearScreen);

    let mut document = Document::new()
        .set("width", IMG_WIDTH)
        .set("height", IMG_HEIGHT);

    let mut turtles = HashMap::new();

    fn create_turtle(turtles: &mut HashMap<i32, Turtle>, index: i32) {
        turtles.insert(
            index,
            Turtle {
                x: 0.0,
                y: 0.0,
                color: "black".to_string(),
                rotation: std::f64::consts::PI / 2.0,
                pendown: true,
                font_size: 12.0,
            },
        );
    }

    create_turtle(&mut turtles, 1);

    let mut turtle_idx = 1;

    fn move_forward(x: f64, turtle: &mut Turtle, mut document: Document) -> Document {
        let dx = x * f64::cos(turtle.rotation);
        let dy = x * -f64::sin(turtle.rotation);
        let data = Data::new()
            .move_to((CENTER_X as f64 + turtle.x, CENTER_Y as f64 + turtle.y))
            .line_by((dx, dy));
        let path = Path::new().set("d", data).set("stroke", &turtle.color[..]);
        if turtle.pendown {
            document = document.add(path);
        }
        turtle.x += dx;
        turtle.y += dy;
        document
    }

    for cmd in cmds {
        let turtle = turtles.get_mut(&turtle_idx).unwrap();

        match cmd {
            DrawCmd::Forward(x) => {
                document = move_forward(x, turtle, document);
            }
            DrawCmd::Back(x) => {
                document = move_forward(-x, turtle, document);
            }
            DrawCmd::LeftTurn(d) => {
                turtle.rotation += d * std::f64::consts::PI / 180.0;
            }
            DrawCmd::RightTurn(d) => {
                turtle.rotation -= d * std::f64::consts::PI / 180.0;
            }
            DrawCmd::SetColor(c) => {
                turtle.color = c;
            }
            DrawCmd::PenUp => {
                turtle.pendown = false;
            }
            DrawCmd::PenDown => {
                turtle.pendown = true;
            }
            DrawCmd::Label(s) => {
                document = document.add(
                    Text::new()
                        .set("x", CENTER_X as f64 + turtle.x)
                        .set("y", CENTER_Y as f64 + turtle.y)
                        .set("font-size", turtle.font_size)
                        .set(
                            "transform",
                            format!(
                                "rotate({} {} {})",
                                -turtle.rotation * 180.0 / std::f64::consts::PI,
                                CENTER_X as f64 + turtle.x,
                                CENTER_Y as f64 + turtle.y
                            ),
                        )
                        .add(svg::node::Text::new(&s)),
                )
            }
            DrawCmd::SetFontSize(n) => {
                turtle.font_size = n;
            }
            DrawCmd::ClearScreen => {
                document = document.add(
                    Rectangle::new()
                        .set("x", 0)
                        .set("y", 0)
                        .set("width", IMG_WIDTH)
                        .set("height", IMG_HEIGHT)
                        .set("fill", "white"),
                )
            }
            DrawCmd::SetTurtle(idx) => {
                if let None = turtles.get(&idx) {
                    create_turtle(&mut turtles, idx);
                }
                turtle_idx = idx;
            }
        }
    }

    svg::save(destination, &document).unwrap();
}
