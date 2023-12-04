use std::collections::HashMap;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle, Text};
use svg::Document;

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

pub fn draw(destination: &str, mut cmds: Vec<DrawCmd>, img_width: u32, img_height: u32) {
    let center_x = img_width / 2;
    let center_y = img_height / 2;

    cmds.insert(0, DrawCmd::ClearScreen);

    let mut document = Document::new()
        .set("width", img_width)
        .set("height", img_height);

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

    let mut turtle = turtles.get_mut(&1).unwrap();

    let move_forward = |u: f64, turtle: &mut Turtle, mut document: Document| {
        let dx = u * f64::cos(turtle.rotation);
        let dy = u * -f64::sin(turtle.rotation);
        let data = Data::new()
            .move_to((center_x as f64 + turtle.x, center_y as f64 + turtle.y))
            .line_by((dx, dy));
        let path = Path::new().set("d", data).set("stroke", &turtle.color[..]);
        if turtle.pendown {
            document = document.add(path);
        }
        turtle.x += dx;
        turtle.y += dy;
        document
    };

    for cmd in cmds {
        match cmd {
            DrawCmd::Forward(u) => {
                document = move_forward(u, turtle, document);
            }
            DrawCmd::Back(u) => {
                document = move_forward(-u, turtle, document);
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
                        .set("x", center_x as f64 + turtle.x)
                        .set("y", center_y as f64 + turtle.y)
                        .set("font-size", turtle.font_size)
                        .set(
                            "transform",
                            format!(
                                "rotate({} {} {})",
                                -turtle.rotation * 180.0 / std::f64::consts::PI,
                                center_x as f64 + turtle.x,
                                center_y as f64 + turtle.y
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
                        .set("width", img_width)
                        .set("height", img_height)
                        .set("fill", "white"),
                )
            }
            DrawCmd::SetTurtle(idx) => {
                if let None = turtles.get(&idx) {
                    create_turtle(&mut turtles, idx);
                }
                turtle = turtles.get_mut(&idx).unwrap();
            }
        }
    }

    svg::save(destination, &document).unwrap();
}
