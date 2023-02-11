use core::fmt;
use std::{collections::HashMap, rc::Rc};

use rand::random;

use crate::{
    mips,
    pattern::{BlockPatt, ExprBuilder, ExprPattern, SimplePattern, TextPatt, TextPattVar},
};

use super::{BlockType, CodeBlock, Expression};

#[derive(Clone, Debug)]
pub struct IfBlock {
    code: Box<dyn Expression>,
    cond: Box<dyn Expression>,
}
impl Expression for IfBlock {
    fn gen_mips(&self) -> String {
        let id = random::<u32>();
        let cond_mips = self.cond.gen_mips();
        let if_mips = mips::pop() + &format!("#branch:\nbeqz $t0, if_false{id}\n");

        cond_mips + &if_mips + "#branch code: \n" + &self.code.gen_mips() + &format!("if_false{id}:\n")
    }
    fn get_name(&self) -> String {
        String::from("if")
    }
}
impl IfBlock {
    fn new(
        code: Box<dyn Expression>,
        cond: Box<dyn Expression>,
    ) -> Self {
        Self {
            code,
            cond
        }
    }
    fn construct(mut params: Vec<Box<dyn Expression>>, _: &mut FrameLayer) -> Box<dyn Expression> {
        assert_eq!(params.len(), 2);
        let code = params.pop().unwrap();
        let cond = params.pop().unwrap();
        Box::new(Self::new(code, cond))
    }
    pub fn get_builder() -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> = vec![
            Box::new(TextPatt("if".to_string())),
            Box::new(BlockPatt(BlockType::Brack)),
            Box::new(BlockPatt(BlockType::Curl)),
        ];
        let constructor: Box<dyn Fn(Vec<Box<dyn Expression>>, &mut FrameLayer) -> Box<dyn Expression>> =
            Box::new(Self::construct);
        ExprBuilder::new(patterns, constructor)
    }
}
//while
#[derive(Clone, Debug)]
pub struct WhileBlock {
    code: Box<dyn Expression>,
    cond: Box<dyn Expression>,
}
impl Expression for WhileBlock {
    fn gen_mips(&self) -> String {
        let id:u32 = random();
        let cond_mips = self.cond.gen_mips();
        let while_mips = mips::pop() + &format!("#branch:\nbeqz $t0, while_end{id}\n");
        format!("while_start{id}:\n#calc cond\n") + &cond_mips + "#eval cond: \n"+ &while_mips  + &self.code.gen_mips() + &format!("j while_start{id}\nwhile_end{id}:\n")
    }
    fn get_name(&self) -> String {
        String::from("while")
    }
}
impl WhileBlock {
    fn new(
        code: Box<dyn Expression>,
        cond: Box<dyn Expression>,
    ) -> Self {
        Self {
            code,
            cond
        }
    }
    fn construct(mut params: Vec<Box<dyn Expression>>, _: &mut FrameLayer) -> Box<dyn Expression> {
        assert_eq!(params.len(), 2);
        let code = params.pop().unwrap();
        let cond = params.pop().unwrap();
        Box::new(Self::new(code, cond))
    }
    pub fn get_builder() -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> = vec![
            Box::new(TextPatt("while".to_string())),
            Box::new(BlockPatt(BlockType::Brack)),
            Box::new(BlockPatt(BlockType::Curl)),
        ];
        let constructor: Box<dyn Fn(Vec<Box<dyn Expression>>, &mut FrameLayer) -> Box<dyn Expression>> =
            Box::new(Self::construct);
        ExprBuilder::new(patterns, constructor)
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
        self.values.0.gen_mips()
            + &self.values.1.gen_mips()
            + &mips::pop_two()
            + &self.mips
            + "\n"
            + &mips::save_t0()
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
        let constructor: Box<dyn Fn(Vec<Box<dyn Expression>>, &mut FrameLayer) -> Box<dyn Expression>> =
            Box::new(move |mut params, _| {
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
        let value = &self.0;
        mips::push_value(value)
    }
    fn get_name(&self) -> String {
        String::from("number")
    }
}
#[derive(Clone, Debug)]
pub struct FrameLayer {
    vars: HashMap<String, usize>,
}
impl FrameLayer {
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
    //generate mips
    pub fn gen_mips(&self) -> String {
        let n = self.vars.len();
        format!("add $t6, $sp, $zero\naddi $sp, $sp, {n}\n")
    }
}
pub struct FrameStack{
    layers: Vec<Rc<FrameLayer>>,
    top: FrameLayer,
}
impl FrameStack{
    pub fn new() -> Self{
        Self{
            layers: vec![],
            top: FrameLayer::new(),
        }
    }
    pub fn push(&self) -> Self{
        let mut layers = self.layers.clone();
        layers.push(Rc::new(self.top.clone()));
        let top = FrameLayer::new();
        Self{layers,top}
    }
    pub fn get_addr(&mut self, name: &str) -> usize{
        for layer in self.layers.iter().rev(){
            if let Some(addr) = layer.vars.get(name){
                return *addr;
            }
        }
        self.top.get_addr(name)
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
        mips::load_var(self.addr)
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
        let constructor: Box<dyn Fn(Vec<Box<dyn Expression>>, &mut FrameLayer) -> Box<dyn Expression>> =
            Box::new(move |mut params, _| {
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
        self.arg.gen_mips() + &self.func.get_call_mips()
    }
    fn get_name(&self) -> String {
        String::from("func")
    }
}
pub struct PrintFn;
impl Function for PrintFn {
    fn get_call_mips(&self) -> String {
        mips::syscall(1)
    }

    fn get_name(&self) -> String {
        String::from("print")
    }
}
//variable declaration
#[derive(Clone, Debug)]
pub struct VarDecl {
    name: String,
    addr: usize,
    value: Box<dyn Expression>,
}
impl VarDecl {
    pub fn new(name: String, addr: usize, value: Box<dyn Expression>) -> Self {
        Self { name, addr, value }
    }
    pub fn get_builder() -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> = vec![
            Box::new(TextPattVar),
            Box::new(TextPatt(String::from("sei"))),
            Box::new(ExprPattern),
        ];
        let constructor: Box<dyn Fn(Vec<Box<dyn Expression>>, &mut FrameLayer) -> Box<dyn Expression>> =
            Box::new(move |mut params, frame| {
                let value = params.pop().unwrap();
                let name = params.pop().unwrap();
                let name = name.get_name();
                let addr = frame.get_addr(&name);
                let var = Box::new(Self::new(name, addr, value));
                var
            });
        ExprBuilder::new(patterns, constructor)
    }
}
impl Expression for VarDecl {
    fn gen_mips(&self) -> String {
        self.value.gen_mips() + &mips::save_var(self.addr)
    }
    fn get_name(&self) -> String {
        String::from("var decl")
    }
}
