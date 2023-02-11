use core::fmt;

use self::statements::FrameStack;

pub mod statements;
pub trait Expression: ExpressionClone + fmt::Debug {
    fn gen_mips(&self) -> String;
    fn get_name(&self) -> String;
    fn as_block(&self) -> Option<&CodeBlock> {
        None
    }
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
    pub frame: Option<FrameStack>,
}

impl CodeBlock {
    pub fn new(lines: Vec<Box<dyn Expression>>, block_type: BlockType, frame: Option<FrameStack>) -> Self {
        Self { block_type, lines, frame }
    }
}

impl Expression for CodeBlock {
    fn gen_mips(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.gen_mips())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_name(&self) -> String {
        self.block_type.get_name()
    }
    fn as_block(&self) -> Option<&CodeBlock> {
        Some(self)
    }
}
