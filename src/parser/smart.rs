use pest::Parser;
use pest_derive::Parser;

use crate::parser::{Expr, Program, Statement};

#[derive(Parser)]
#[grammar = "grammars/smart.pest"]
pub struct SheepParser;

type ParserResult = Result<Program, pest::error::Error<Rule>>;

impl SheepParser {
    pub fn parse_program(input: &str) -> ParserResult{
        let pairs = SheepParser::parse(Rule::program, input)?;
        let mut statements = Vec::new();
        let mut main_expr = None;
        
        for pair in pairs {
            match pair.as_rule() {
                Rule::program => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::item => {
                                let item = inner_pair.into_inner().next().unwrap();
                                match item.as_rule() {
                                    Rule::let_binding => {
                                        statements.push(parse_let_binding(item));
                                    }
                                    Rule::expr => {
                                        main_expr = Some(parse_expr(item));
                                    }
                                    _ => unreachable!()
                                }
                            }
                            Rule::EOI => {}
                            _ => unreachable!()
                        }
                    }
                }
                _ => unreachable!()
            }
        }
        
        Ok(Program { statements, main_expr })
    }
}

fn parse_let_binding(pair: pest::iterators::Pair<Rule>) -> Statement {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let value = parse_expr(inner.next().unwrap());
    Statement::LetBinding { name, value }
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::identifier => Expr::Var(pair.as_str().to_string()),
        Rule::builtin => Expr::Builtin(pair.as_str().to_string()),
        Rule::lambda => {
            let mut inner = pair.into_inner();
            let param = inner.next().unwrap().as_str().to_string();
            let body = Box::new(parse_expr(inner.next().unwrap()));
            Expr::Lambda { param, body }
        }
        Rule::application => {
            let mut inner = pair.into_inner();
            let mut expr = parse_expr(inner.next().unwrap());
            
            for arg_pair in inner {
                let arg = Box::new(parse_expr(arg_pair));
                expr = Expr::App { 
                    func: Box::new(expr), 
                    arg 
                };
            }
            expr
        }
        Rule::atom => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        Rule::expr => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        _ => unreachable!("Unexpected rule: {:?}", pair.as_rule())
    }
}
