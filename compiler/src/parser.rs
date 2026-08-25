use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ast::*;

#[derive(Parser)]
#[grammar = "openmatl.pest"]
pub struct OpenMatLParser;

pub fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::number => {
            let val = pair.as_str().parse::<f64>().unwrap();
            Expr::Number(val)
        }
        Rule::string_literal => {
            let val = pair.as_str();
            let content = val[1..val.len() - 1].to_string(); // Strip quotes
            Expr::StringLiteral(content)
        }
        Rule::ident => Expr::Identifier(pair.as_str().to_string()),
        Rule::array => {
            let mut elements = Vec::new();
            for inner_pair in pair.into_inner() {
                elements.push(parse_expr(inner_pair));
            }
            Expr::Array(elements)
        }
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let arg = parse_expr(inner.next().unwrap());
            Expr::FunctionCall {
                name,
                arg: Box::new(arg),
            }
        }
        Rule::slice_access => {
            let mut inner = pair.into_inner();
            let target = inner.next().unwrap().as_str().to_string();
            let start = parse_expr(inner.next().unwrap());
            let end = inner.next().map(|p| Box::new(parse_expr(p)));
            Expr::SliceAccess {
                target,
                start: Box::new(start),
                end,
            }
        }
        Rule::expr => {
            let mut inner = pair.into_inner();
            let mut lhs = parse_expr(inner.next().unwrap());
            
            while let Some(op_pair) = inner.next() {
                let op = match op_pair.as_rule() {
                    Rule::add => Operator::Add,
                    Rule::sub => Operator::Sub,
                    Rule::mul => Operator::Mul,
                    Rule::div => Operator::Div,
                    Rule::pow => Operator::Pow,
                    Rule::dot => Operator::Dot,
                    Rule::eq  => Operator::Eq,
                    Rule::neq => Operator::Neq,
                    Rule::gt  => Operator::Gt,
                    Rule::lt  => Operator::Lt,
                    Rule::gte => Operator::Gte,
                    Rule::lte => Operator::Lte,
                    _ => unreachable!("Unknown operator"),
                };
                let rhs = parse_expr(inner.next().unwrap());
                lhs = Expr::BinaryOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                };
            }
            lhs
        }
        _ => unreachable!("Unexpected rule in expression: {:?}", pair.as_rule()),
    }
}

pub fn parse_block(pair: Pair<Rule>) -> Vec<Statement> {
    let mut stmts = Vec::new();
    for stmt_pair in pair.into_inner() {
        stmts.push(parse_statement(stmt_pair));
    }
    stmts
}

pub fn parse_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let target = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap());
            Statement::Assignment { target, expr }
        }
        Rule::expr => Statement::Expression(parse_expr(pair)),
        Rule::if_stmt => {
            let mut inner = pair.into_inner();
            let condition = parse_expr(inner.next().unwrap());
            let true_block = parse_block(inner.next().unwrap());
            let false_block = inner.next().map(parse_block);
            Statement::IfElse { condition, true_block, false_block }
        }
        Rule::fn_stmt => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let mut params = Vec::new();
            let mut body_pair = None;
            for p in inner {
                if p.as_rule() == Rule::ident {
                    params.push(p.as_str().to_string());
                } else if p.as_rule() == Rule::block {
                    body_pair = Some(p);
                }
            }
            let body = parse_block(body_pair.unwrap());
            Statement::FunctionDef { name, params, body }
        }
        Rule::return_stmt => {
            let expr = parse_expr(pair.into_inner().next().unwrap());
            Statement::Return(expr)
        }
        _ => unreachable!("Unexpected rule in statement: {:?}", pair.as_rule()),
    }
}

pub fn parse_program(input: &str) -> Result<Program, pest::error::Error<Rule>> {
    let mut program_pairs = OpenMatLParser::parse(Rule::program, input)?;
    
    let mut statements = Vec::new();
    if let Some(program_pair) = program_pairs.next() {
        for pair in program_pair.into_inner() {
            if pair.as_rule() != Rule::EOI {
                statements.push(parse_statement(pair));
            }
        }
    }
    
    Ok(Program { statements })
}
