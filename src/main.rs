use std::rc::Rc;

use expression::statements::TwoSideOp;
use parser::parse;

use crate::expression::statements::{FunctionCall, PrintFn};
mod expression;
mod lexer;
mod mips;
mod parser;
mod pattern;

fn main() {
    let builders = vec![
        TwoSideOp::get_builder("*", "mult $t0, $t1\nmflo $t0"),
        TwoSideOp::get_builder("+", "add $t0, $t0, $t1"),
        FunctionCall::get_builder(Rc::new(PrintFn)),
    ];
    let s = "print(2+2)".to_string();
    let ast = parse(s, &builders);
    println!("ast: {:#?}", ast);
    println!("mips: {}", ast.gen_mips())
}
