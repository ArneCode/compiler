use std::rc::Rc;

use expression::statements::TwoSideOp;
use parser::parse;

use crate::expression::statements::{FunctionCall, PrintFn, VarDecl, IfBlock, WhileBlock};
mod expression;
mod lexer;
mod mips;
mod parser;
mod pattern;

fn main() {
    let builders = vec![
        TwoSideOp::get_builder("*", "mult $t0, $t1\nmflo $t0"),
        TwoSideOp::get_builder("+", "add $t0, $t0, $t1"),
        TwoSideOp::get_builder("<", "slt $t0, $t1, $t0"),
        VarDecl::get_builder(),
        IfBlock::get_builder(),
        WhileBlock::get_builder(),
        FunctionCall::get_builder(Rc::new(PrintFn)),
    ];
    let s = "i sei 0;while(i<10){i sei i+1;print(i)}".to_string();
    //let s = "print(3<2)".to_string();
    let (ast,frame) = parse(s, &builders);
    println!("ast: {:#?}", ast);
    println!("addi $sp, $sp, -1000");
    println!("{}", frame.gen_mips());
    println!("{}", ast.gen_mips())
}
