use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "logo.pest"]
struct LogoParser;

#[derive(Debug, PartialEq, Clone)]
pub enum Binop {
    And,
    Or,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    String(String),
    Number(f64),
    Variable(String),
    List(Vec<AstNode>),
    Binop {
        lhs: Box<AstNode>,
        op: Binop,
        rhs: Box<AstNode>,
    },
    Call {
        f: Box<AstNode>,
        args: Vec<Box<AstNode>>,
    },
    If {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    Loop {
        repeat: Box<AstNode>,
        body: Box<AstNode>,
    },
    ProcDef {
        proc_name: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
    Block(Vec<Box<AstNode>>),
}

pub fn parse_logo_file(file: &str) -> Result<AstNode, Error<Rule>> {
    let source = fs::read_to_string(file).unwrap();
    parse_logo_source(&source)
}

pub fn parse_logo_source(source: &str) -> Result<AstNode, Error<Rule>> {
    let mut pest_terms = LogoParser::parse(Rule::program, source)?;
    let program = pest_terms.next().unwrap();
    let eoi = pest_terms.next().unwrap();

    assert!(pest_terms.next().is_none());
    assert_eq!(program.as_rule(), Rule::block);
    assert_eq!(eoi.as_rule(), Rule::EOI);

    fn parse_term(term: Pair<Rule>) -> AstNode {
        fn string_to_binop(s: &str) -> Binop {
            match s {
                "and" => Binop::And,
                "or" => Binop::Or,
                "<" => Binop::Less,
                "<=" => Binop::LessEqual,
                ">" => Binop::Greater,
                ">=" => Binop::GreaterEqual,
                "==" => Binop::GreaterEqual,
                "+" => Binop::Add,
                "-" => Binop::Sub,
                "*" => Binop::Mul,
                "/" => Binop::Div,
                _ => unreachable!(),
            }
        }

        fn parse_binop(term: Pair<Rule>) -> AstNode {
            let mut subterms = term.into_inner();
            let lhs = subterms.next().unwrap();
            let op = subterms.next();
            if op.is_none() {
                parse_term(lhs)
            } else {
                let op = op.unwrap();
                let rhs = subterms.next().unwrap();
                AstNode::Binop {
                    lhs: Box::new(parse_term(lhs)),
                    op: string_to_binop(op.as_str()),
                    rhs: Box::new(parse_term(rhs)),
                }
            }
        }

        match term.as_rule() {
            Rule::number => AstNode::Number(term.as_str().parse().unwrap()),
            Rule::word => AstNode::String(term.as_str().to_string()),
            Rule::string => AstNode::String(term.as_str()[1..].to_string()),
            Rule::identifier | Rule::fn_identifier => AstNode::Variable(term.as_str().to_string()),
            Rule::variable => AstNode::Variable(term.as_str()[1..].to_string()),
            Rule::logic | Rule::comp | Rule::add | Rule::mult => parse_binop(term),
            Rule::list => AstNode::List(term.into_inner().map(|t| parse_term(t)).collect()),
            Rule::call | Rule::fn_call => {
                let mut ts = term.into_inner();
                let f = Box::new(parse_term(ts.next().unwrap()));
                let args: Vec<Box<AstNode>> = ts.map(|t| Box::new(parse_term(t))).collect();
                AstNode::Call { f, args }
            }
            Rule::cond => {
                let mut ts = term.into_inner();
                let condition = Box::new(parse_term(ts.next().unwrap()));
                let body = Box::new(parse_term(ts.next().unwrap()));
                AstNode::If { condition, body }
            }
            Rule::repeat => {
                let mut ts = term.into_inner();
                let repeat = Box::new(parse_term(ts.next().unwrap()));
                let body = Box::new(parse_term(ts.next().unwrap()));
                AstNode::Loop { repeat, body }
            }
            Rule::proc_def => {
                let mut ts = term.into_inner();
                let proc_name = ts.next().unwrap().as_str().to_string();
                let mut rest: Vec<Pair<Rule>> = ts.collect();
                let body = if let Some(b) = rest.pop() {
                    Box::new(parse_term(b))
                } else {
                    unreachable!("procedure should have a body!");
                };
                let params = rest
                    .into_iter()
                    .map(|t| t.as_str()[1..].to_string())
                    .collect();
                AstNode::ProcDef {
                    proc_name,
                    params,
                    body,
                }
            }
            Rule::block => {
                AstNode::Block(term.into_inner().map(|t| Box::new(parse_term(t))).collect())
            }
            Rule::WHITESPACE
            | Rule::EOI
            | Rule::keyword
            | Rule::expr
            | Rule::statement
            | Rule::logic_op
            | Rule::comp_op
            | Rule::add_op
            | Rule::mult_op
            | Rule::primary
            | Rule::program => {
                dbg!(&term);
                unreachable!()
            }
        }
    }

    Ok(parse_term(program))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arith_expr() {
        let source = "print 1 + 2";
        let ast = parse_logo_source(source);
        assert!(ast.is_ok());
        let ast = ast.unwrap();
        assert_eq!(
            ast,
            AstNode::Block(vec![Box::new(AstNode::Call {
                f: Box::new(AstNode::Variable("print".to_string())),
                args: vec![Box::new(AstNode::Binop {
                    lhs: Box::new(AstNode::Number(1.0)),
                    op: Binop::Add,
                    rhs: Box::new(AstNode::Number(2.0))
                })]
            })])
        )
    }

    #[test]
    fn operator_priority() {
        let source = "print 1 + 2 * 3";
        let ast = parse_logo_source(source);
        assert!(ast.is_ok());
        let ast = ast.unwrap();
        assert_eq!(
            ast,
            AstNode::Block(vec![Box::new(AstNode::Call {
                f: Box::new(AstNode::Variable("print".to_string())),
                args: vec![Box::new(AstNode::Binop {
                    lhs: Box::new(AstNode::Number(1.0)),
                    op: Binop::Add,
                    rhs: Box::new(AstNode::Binop {
                        lhs: Box::new(AstNode::Number(2.0)),
                        op: Binop::Mul,
                        rhs: Box::new(AstNode::Number(3.0)),
                    })
                })]
            })])
        )
    }
}
