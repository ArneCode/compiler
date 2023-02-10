use core::fmt;
use std::{collections::HashMap, rc::Rc};

use crate::pattern::{BlockPatt, ExprBuilder, ExprPattern, SimplePattern, TextPatt};

use super::{BlockType, CodeBlock, Expression};

#[derive(Clone, Debug)]
struct IfBlock {
    code: Box<dyn Expression>,
    cond_l: Box<dyn Expression>,
    cond_r: Box<dyn Expression>,
}
impl Expression for IfBlock {
    fn gen_mips(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        String::from("if")
    }
}
impl IfBlock {
    fn new(
        code: Box<dyn Expression>,
        cond_l: Box<dyn Expression>,
        cond_r: Box<dyn Expression>,
    ) -> Self {
        Self {
            code,
            cond_l,
            cond_r,
        }
    }
    fn construct(mut params: Vec<Box<dyn Expression>>) -> Box<dyn Expression> {
        assert_eq!(params.len(), 3);
        let cond_r = params.pop().unwrap();
        let cond_l = params.pop().unwrap();
        let code = params.pop().unwrap();
        Box::new(Self::new(code, cond_l, cond_r))
    }
}
#[derive(Clone, Debug)]
pub struct TwoSideOp {
    values: (Box<dyn Expression>, Box<dyn Expression>),
    sign: String,
    mips: String,
}

impl Expression for TwoSideOp {
    fn gen_mips(&self) -> String {
        self.values.0.gen_mips() + &self.values.1.gen_mips() + &self.mips
    }

    fn get_name(&self) -> String {
        self.sign.clone()
    }
}
impl TwoSideOp {
    pub fn new(
        values: (Box<dyn Expression>, Box<dyn Expression>),
        sign: String,
        mips: String,
    ) -> Self {
        Self { values, sign, mips }
    }
    pub fn get_builder(sign: &str, mips: &str) -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> = vec![
            Box::new(ExprPattern),
            Box::new(TextPatt(sign.to_string())),
            Box::new(ExprPattern),
        ];
        let sign = sign.to_string();
        let mips = mips.to_string();
        let constructor: Box<dyn Fn(Vec<Box<dyn Expression>>) -> Box<dyn Expression>> =
            Box::new(move |mut params| {
                let b = params.pop().unwrap();
                let a = params.pop().unwrap();
                Box::new(Self {
                    values: (a, b),
                    sign: sign.clone(),
                    mips: mips.clone(),
                })
            });

        ExprBuilder::new(patterns, constructor)
    }
}
#[derive(Clone, Debug)]
pub struct Number(pub String);
impl Expression for Number {
    fn gen_mips(&self) -> String {
        todo!()
    }
    fn get_name(&self) -> String {
        String::from("number")
    }
}
pub struct StackFrame {
    vars: HashMap<String, usize>,
}
impl StackFrame {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    pub fn get_addr(&mut self, name: &str) -> usize {
        if let Some(addr) = self.vars.get(name) {
            *addr
        } else {
            let addr = self.vars.len();
            self.vars.insert(name.to_string(), addr);
            addr
        }
    }
}
#[derive(Clone, Debug)]
pub struct Var {
    name: String,
    addr: usize,
}

impl Var {
    pub fn new(name: String, addr: usize) -> Self {
        Self { name, addr }
    }
}

impl Expression for Var {
    fn gen_mips(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
pub trait Function {
    fn get_call_mips(&self) -> String;
    fn get_name(&self) -> String;
}
#[derive(Clone)]
pub struct FunctionCall {
    func: Rc<dyn Function>,
    arg: Box<dyn Expression>,
}
impl FunctionCall {
    fn new(func: Rc<dyn Function>, arg: Box<dyn Expression>) -> Self {
        Self { func, arg }
    }

    pub fn get_builder(func: Rc<dyn Function>) -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> = vec![
            Box::new(TextPatt(func.get_name())),
            Box::new(BlockPatt(BlockType::Brack)),
        ];
        let constructor: Box<dyn Fn(Vec<Box<dyn Expression>>) -> Box<dyn Expression>> =
            Box::new(move |mut params| {
                assert_eq!(params.len(), 1);
                let arg = params.pop().unwrap();
                let func = Box::new(Self::new(func.clone(), arg));
                func
            });
        ExprBuilder::new(patterns, constructor)
    }
}
impl fmt::Debug for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "func call: {}({:#?})", self.get_name(), self.arg)
    }
}
impl Expression for FunctionCall {
    fn gen_mips(&self) -> String {
        self.func.get_call_mips()
    }
    fn get_name(&self) -> String {
        String::from("func")
    }
}
pub struct PrintFn;
impl Function for PrintFn {
    fn get_call_mips(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        String::from("print")
    }
}
