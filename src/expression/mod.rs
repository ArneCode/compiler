use core::fmt;

pub mod statements;
pub trait Expression: ExpressionClone + fmt::Debug {
    fn gen_mips(&self) -> String;
    fn get_name(&self) -> String;
}
pub trait ExpressionClone {
    fn clone_box(&self) -> Box<dyn Expression>;
}
impl<T> ExpressionClone for T
where
    T: Expression + 'static + Clone,
{
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Box<dyn Expression> {
        self.clone_box()
    }
}
#[derive(Clone, Debug)]
pub enum BlockType {
    Curl,
    Brack,
}
impl BlockType {
    pub fn get_name(&self) -> String {
        String::from(match self {
            BlockType::Curl => "curl",
            BlockType::Brack => "brack",
        })
    }
}
#[derive(Clone, Debug)]
pub struct CodeBlock {
    block_type: BlockType,
    lines: Vec<Box<dyn Expression>>,
}

impl CodeBlock {
    pub fn new(lines: Vec<Box<dyn Expression>>, block_type: BlockType) -> Self {
        Self { block_type, lines }
    }
}

impl Expression for CodeBlock {
    fn gen_mips(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        self.block_type.get_name()
    }
}
