use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rand::random;

use crate::{
    mips,
    pattern::{
        BlockPatt, ExprBuilder, ExprConstr, ExprPattern, SimplePattern, TextPatt, TextPattVar,
    },
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

        cond_mips
            + &if_mips
            + "#branch code: \n"
            + &self.code.gen_mips()
            + &format!("if_false{id}:\n")
    }
    fn get_name(&self) -> String {
        String::from("if")
    }
}
impl IfBlock {
    fn new(code: Box<dyn Expression>, cond: Box<dyn Expression>) -> Self {
        Self { code, cond }
    }
    fn construct(mut params: Vec<Box<dyn Expression>>, _: &mut FrameStack) -> Box<dyn Expression> {
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
        let constructor: ExprConstr = Box::new(Self::construct);
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
        let id: u32 = random();
        let cond_mips = self.cond.gen_mips();
        let while_mips = mips::pop() + &format!("#branch:\nbeqz $t0, while_end{id}\n");
        format!("while_start{id}:\n#calc cond\n")
            + &cond_mips
            + "#eval cond: \n"
            + &while_mips
            + &self.code.gen_mips()
            + &format!("j while_start{id}\nwhile_end{id}:\n")
    }
    fn get_name(&self) -> String {
        String::from("while")
    }
}
impl WhileBlock {
    fn new(code: Box<dyn Expression>, cond: Box<dyn Expression>) -> Self {
        Self { code, cond }
    }
    fn construct(mut params: Vec<Box<dyn Expression>>, _: &mut FrameStack) -> Box<dyn Expression> {
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
        let constructor: ExprConstr = Box::new(Self::construct);
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
        let constructor: ExprConstr = Box::new(move |mut params, _| {
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
    pub fn get_addr(&mut self, name: &str, alt_addr: usize) -> usize {
        if let Some(addr) = self.vars.get(name) {
            *addr
        } else {
            let addr = alt_addr;
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
#[derive(Clone, Debug)]
pub struct FrameStack {
    layers: Vec<Rc<FrameLayer>>,
    top: FrameLayer,
    n_vars: Rc<RefCell<usize>>,
}
impl FrameStack {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            top: FrameLayer::new(),
            n_vars: Rc::new(RefCell::new(0)),
        }
    }
    pub fn push(&self) -> Self {
        let mut layers = self.layers.clone();
        layers.push(Rc::new(self.top.clone()));
        let top = FrameLayer::new();
        Self {
            layers,
            top,
            n_vars: self.n_vars.clone(),
        }
    }
    pub fn get_addr(&mut self, name: &str) -> usize {
        for layer in self.layers.iter().rev() {
            if let Some(addr) = layer.vars.get(name) {
                return *addr;
            }
        }
        *self.n_vars.borrow_mut() += 1;
        self.top.get_addr(name, *self.n_vars.borrow() - 1)
    }
    pub fn gen_mips(&self) -> String {
        let n = *self.n_vars.borrow();
        format!("add $t6, $sp, $zero\naddi $sp, $sp, {n}\n")
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
        mips::load_var(self.addr as i32)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub trait Function {
    fn get_call_mips(&self) -> String;
    fn get_name(&self) -> String;
}
#[derive(Clone, Debug)]
struct UnknownFn(String);
impl Function for UnknownFn {
    fn get_call_mips(&self) -> String {
        format!("jal {}\n", self.0)
    }
    fn get_name(&self) -> String {
        self.0.clone()
    }
}
#[derive(Clone)]
pub struct FunctionCall {
    func: Rc<dyn Function>,
    args: Vec<Box<dyn Expression>>,
}
impl FunctionCall {
    fn new(func: Rc<dyn Function>, args: Vec<Box<dyn Expression>>) -> Self {
        Self { func, args }
    }

    pub fn get_builder(func: Rc<dyn Function>) -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> = vec![
            Box::new(TextPatt(func.get_name())),
            Box::new(BlockPatt(BlockType::Brack)),
        ];
        let constructor: ExprConstr = Box::new(move |mut params, _| {
            assert_eq!(params.len(), 1);
            let args = params.pop().unwrap();
            let args = args.as_block().unwrap();
            let args = args.lines.clone();
            let func = Box::new(Self::new(func.clone(), args));
            func
        });
        ExprBuilder::new(patterns, constructor)
    }
    pub fn get_builder_var() -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> =
            vec![Box::new(TextPattVar), Box::new(BlockPatt(BlockType::Brack))];
        let constructor: ExprConstr = Box::new(move |mut params, _| {
            assert_eq!(params.len(), 2);
            let args = params.pop().unwrap();
            let args = args.as_block().unwrap();
            let args = args.lines.clone();
            let name = params.pop().unwrap();
            let name = name.get_name();
            let func = Box::new(Self::new(Rc::new(UnknownFn(name)), args));
            func
        });
        ExprBuilder::new(patterns, constructor)
    }
}
impl fmt::Debug for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "func call: {}({:#?})", self.get_name(), self.args)
    }
}
impl Expression for FunctionCall {
    fn gen_mips(&self) -> String {
        self.args
            .iter()
            .map(|arg| arg.gen_mips())
            .collect::<Vec<_>>()
            .join("\n")
            + &self.func.get_call_mips()
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
//func declaration
#[derive(Clone, Debug)]
pub struct FuncDecl {
    name: String,
    body: Box<dyn Expression>,
    args: Vec<usize>,
    frame: FrameStack,
}
impl FuncDecl {
    pub fn new(
        name: String,
        body: Box<dyn Expression>,
        args: Vec<usize>,
        frame: FrameStack,
    ) -> Self {
        Self {
            name,
            body,
            args,
            frame,
        }
    }
    pub fn get_builder() -> ExprBuilder {
        let patterns: Vec<Box<dyn SimplePattern>> = vec![
            Box::new(TextPatt(String::from("def"))),
            Box::new(TextPattVar),
            Box::new(BlockPatt(BlockType::Brack)),
            Box::new(BlockPatt(BlockType::Curl)),
        ];
        let constructor: Box<
            dyn Fn(Vec<Box<dyn Expression>>, &mut FrameStack) -> Box<dyn Expression>,
        > = Box::new(move |mut params, frame| {
            let brack_param = params.pop().unwrap();
            let body = brack_param.as_block().unwrap();
            let args = params.pop().unwrap();
            let args = args.as_block().unwrap();
            let name = params.pop().unwrap();
            let mut frame = body.frame.as_ref().unwrap().clone();
            let args = args
                .lines
                .iter()
                .map(|var| frame.get_addr(&var.get_name()))
                .collect();
            let name = name.get_name();
            let func = Box::new(Self::new(name, brack_param, args, frame));
            func
        });
        ExprBuilder::new(patterns, constructor)
    }
}
impl Expression for FuncDecl {
    fn gen_mips(&self) -> String {
        format!("j {}_end\n{}:\n", self.name, self.name)
            + &String::from("add $t4, $t6, $zero\n") //save old base pointer
            + &String::from("add $t5, $sp, $zero\n") //save old stack pointer
            + &self.frame.gen_mips()
            + &self.args.iter().rev().enumerate().map(|(i, addr)| mips::load_var(-(i as i32+1))+&mips::save_var(*addr)).collect::<Vec<_>>().join("\n")
            + &self.body.gen_mips()
            + &mips::pop()
            + &String::from("add $sp, $t5, $zero\n")
            + &String::from("add $t6, $t4, $zero\n")
            + &mips::save_t0()
            + &String::from("jr $ra\n")
            + &format!("{}_end:", self.name)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl Function for FuncDecl {
    fn get_call_mips(&self) -> String {
        format!("jal {}\n", self.name)
    }

    fn get_name(&self) -> String {
        self.name.clone()
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
        let constructor: Box<
            dyn Fn(Vec<Box<dyn Expression>>, &mut FrameStack) -> Box<dyn Expression>,
        > = Box::new(move |mut params, frame| {
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
