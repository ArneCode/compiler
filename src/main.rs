use std::rc::Rc;

use expression::statements::TwoSideOp;
use parser::parse;

use crate::expression::{statements::{FunctionCall, PrintFn, VarDecl, IfBlock, WhileBlock, FuncDecl}, Expression};
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
        FuncDecl::get_builder(),
        FunctionCall::get_builder_var(),
    ];
    let s = "
    x sei 5;
    def a(v){
        x sei 3;
        v sei x + v;
        print(v)
    }
    a(3);
    print(x+2);
    ".to_string();
    //let s = "print(3<2)".to_string();
    let ast = parse(s, &builders);
    let frame = ast.frame.as_ref().unwrap();
    println!("ast: {:#?}", ast);
    println!("addi $sp, $sp, -1000");
    println!("{}", frame.gen_mips());
    println!("{}", ast.gen_mips())
}
