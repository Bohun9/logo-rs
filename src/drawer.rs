use svg::node::element::path::Data;
use svg::node::element::{Circle, Path, Rectangle};
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
}

struct Turtle {
    x: f64,
    y: f64,
    color: String,
    rotation: f64,
}

pub fn draw(destination: &str, cmds: Vec<DrawCmd>) {
    let mut document = Document::new()
        .set("width", IMG_WIDTH)
        .set("height", IMG_HEIGHT)
        .add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", IMG_WIDTH)
                .set("height", IMG_HEIGHT)
                .set("fill", "white"),
        )
        .add(
            Circle::new()
                .set("cx", CENTER_X)
                .set("cy", CENTER_Y)
                .set("r", 2)
                .set("fill", "red"),
        );
    let mut turtle = Turtle {
        x: 0.0,
        y: 0.0,
        color: "black".to_string(),
        rotation: std::f64::consts::PI / 2.0,
    };

    fn move_forward(x: f64, turtle: &mut Turtle, mut document: Document) -> Document {
        let dx = x * f64::cos(turtle.rotation);
        let dy = x * -f64::sin(turtle.rotation);
        let data = Data::new()
            .move_to((CENTER_X as f64 + turtle.x, CENTER_Y as f64 + turtle.y))
            .line_by((dx, dy));
        let path = Path::new().set("d", data).set("stroke", &turtle.color[..]);
        document = document.add(path);
        turtle.x += dx;
        turtle.y += dy;
        document
    }

    for cmd in cmds {
        match cmd {
            DrawCmd::Forward(x) => {
                document = move_forward(x, &mut turtle, document);
            }
            DrawCmd::Back(x) => {
                document = move_forward(-x, &mut turtle, document);
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
            _ => {}
        }
    }

    svg::save(destination, &document).unwrap();

    println!(
        "turtle: ({} {}), rotation: {}",
        turtle.x, turtle.y, turtle.rotation
    );
}
