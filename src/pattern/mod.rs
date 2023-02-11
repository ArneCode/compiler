use crate::{
    expression::{
        statements::{FrameLayer, FrameStack, Var},
        BlockType, Expression,
    },
    lexer::token::{Token, TokenType},
};

#[derive(Clone, Debug)]
pub enum TORE<'a> {
    Token(Token<'a>),
    Expr(Box<dyn Expression>),
}
pub trait SimplePattern {
    fn match_tore<'a>(&self, t: &TORE<'a>) -> Option<Option<Box<dyn Expression>>>;
}
//just consumes text
pub struct TextPatt(pub String);
impl SimplePattern for TextPatt {
    fn match_tore<'a>(&self, t: &TORE<'a>) -> Option<Option<Box<dyn Expression>>> {
        if let TORE::Token(t) = t {
            if t.slice == self.0 {
                return Some(None);
            }
        }
        None
    }
}
//returns text
pub struct TextPattVar;
impl SimplePattern for TextPattVar {
    fn match_tore<'a>(&self, t: &TORE<'a>) -> Option<Option<Box<dyn Expression>>> {
        if let TORE::Token(t) = t {
            let var = Var::new(t.slice.to_string(), 0);
            Some(Some(Box::new(var)))
        } else {
            None
        }
    }
}
pub struct BlockPatt(pub BlockType);
impl SimplePattern for BlockPatt {
    fn match_tore<'a>(&self, t: &TORE<'a>) -> Option<Option<Box<dyn Expression>>> {
        if let TORE::Expr(e) = t {
            if e.get_name() == self.0.get_name() {
                return Some(Some(e.clone()));
            }
        }
        None
    }
}
pub struct ExprPattern;
impl SimplePattern for ExprPattern {
    fn match_tore<'a>(&self, t: &TORE<'a>) -> Option<Option<Box<dyn Expression>>> {
        if let TORE::Expr(e) = t {
            return Some(Some(e.clone()));
        } else if let TORE::Token(t) = t {
            if t.token_type == TokenType::Word {
                let name = t.slice.to_string();
                let var: Box<dyn Expression> = Box::new(Var::new(name, 0));
                return Some(Some(var));
            }
        }
        None
    }
}
pub type ExprConstr = Box<dyn Fn(Vec<Box<dyn Expression>>, &mut FrameStack) -> Box<dyn Expression>>;
pub struct ExprBuilder {
    patterns: Vec<Box<dyn SimplePattern>>,
    constructor: ExprConstr,
}
impl ExprBuilder {
    pub fn new(patterns: Vec<Box<dyn SimplePattern>>, constructor: ExprConstr) -> Self {
        Self {
            patterns,
            constructor,
        }
    }
    pub fn parse_occurences<'a>(
        &self,
        mut tokens: Vec<TORE<'a>>,
        frame: &mut FrameStack,
    ) -> Vec<TORE<'a>> {
        let mut i = 0;
        'token_loop: while i + self.patterns.len() <= tokens.len() {
            let mut params = vec![];
            for (off, pattern) in self.patterns.iter().enumerate() {
                let token = &tokens[i + off];
                if let Some(result) = pattern.match_tore(token) {
                    if let Some(result) = result {
                        params.push(result);
                    }
                } else {
                    i += 1;
                    continue 'token_loop;
                }
            }
            let expr = TORE::Expr((self.constructor)(params, frame));
            tokens.splice(i..(i + self.patterns.len()), [expr]);
            i += 1;
        }
        tokens
    }
}
