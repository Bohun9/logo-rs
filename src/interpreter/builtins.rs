use super::value::*;
use super::DrawCmd;
use rand::Rng;

pub fn get_builtins() -> Vec<(Vec<&'static str>, Value)> {
    let mut builtins = vec![];

    fn forward_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 1);
        match args[0] {
            Value::Number(n) => {
                inter.drawing.push(DrawCmd::Forward(n));
            }
            _ => panic!(""),
        }
        Value::Nothing
    }
    builtins.push((
        vec!["forward", "fd"],
        LangFn {
            arity: 1,
            function: forward_fn,
        },
    ));

    fn back_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 1);
        match args[0] {
            Value::Number(n) => {
                inter.drawing.push(DrawCmd::Back(n));
            }
            _ => panic!(""),
        }
        Value::Nothing
    }
    builtins.push((
        vec!["back", "bk"],
        LangFn {
            arity: 1,
            function: back_fn,
        },
    ));

    fn leftturn_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 1);
        match args[0] {
            Value::Number(n) => {
                inter.drawing.push(DrawCmd::LeftTurn(n));
            }
            _ => panic!(""),
        }
        Value::Nothing
    }
    builtins.push((
        vec!["left", "lt"],
        LangFn {
            arity: 1,
            function: leftturn_fn,
        },
    ));

    fn rightturn_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 1);
        match args[0] {
            Value::Number(n) => {
                inter.drawing.push(DrawCmd::RightTurn(n));
            }
            _ => panic!(""),
        }
        Value::Nothing
    }
    builtins.push((
        vec!["right", "rt"],
        LangFn {
            arity: 1,
            function: rightturn_fn,
        },
    ));

    fn setcolor_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 1);
        match &args[0] {
            Value::String(s) => match &s[..] {
                "black" | "blue" | "green" | "cyan" | "red" | "magenta" | "yellow" | "white"
                | "brown" | "tan" | "aqua" | "salmo" | "purple" | "orange" | "gray" | "violet" => {
                    inter.drawing.push(DrawCmd::SetColor(s.clone()));
                }
                _ => panic!(""),
            },
            _ => panic!(""),
        }
        Value::Nothing
    }
    builtins.push((
        vec!["setcolor"],
        LangFn {
            arity: 1,
            function: setcolor_fn,
        },
    ));

    fn clearscreen_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 0);
        inter.drawing.push(DrawCmd::ClearScreen);
        Value::Nothing
    }
    builtins.push((
        vec!["clearscreen", "cs"],
        LangFn {
            arity: 0,
            function: clearscreen_fn,
        },
    ));

    fn pick_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 1);
        match &args[0] {
            Value::List(list) => {
                if list.len() == 0 {
                    panic!("pick of empty list");
                } else {
                    let mut rng = rand::thread_rng();
                    let index = rng.gen_range(0..list.len());
                    let v = (&list[index]).clone();
                    v
                }
            }
            _ => panic!(""),
        }
    }
    builtins.push((
        vec!["pick"],
        LangFn {
            arity: 1,
            function: pick_fn,
        },
    ));

    fn random_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 1);
        match args[0] {
            Value::Number(n) => {
                let mut rng = rand::thread_rng();
                Value::Number(rng.gen_range(0..(n as i32)) as f64)
            }
            _ => panic!(""),
        }
    }
    builtins.push((
        vec!["random"],
        LangFn {
            arity: 1,
            function: random_fn,
        },
    ));

    fn stop_fn(inter: &mut Interpreter, args: Vec<Value>) -> Value {
        assert_eq!(args.len(), 0);
        Value::Return
    }
    builtins.push((
        vec!["stop"],
        LangFn {
            arity: 0,
            function: stop_fn,
        },
    ));

    builtins
        .into_iter()
        .map(|(n, f)| (n, Value::Function(LogoFn::LangFn(f))))
        .collect()
}
