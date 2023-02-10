use std::rc::Rc;

use expression::statements::TwoSideOp;
use parser::parse;

use crate::expression::statements::{FunctionCall, PrintFn};
mod expression;
mod lexer;
mod parser;
mod pattern;

fn main() {
    let builders = vec![
        TwoSideOp::get_builder("*", ""),
        TwoSideOp::get_builder("+", ""),
        FunctionCall::get_builder(Rc::new(PrintFn)),
    ];
    let s = "print(2)".to_string();
    let ast = parse(s, &builders);
    println!("ast: {:#?}", ast);
}
