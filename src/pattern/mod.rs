use crate::{
    expression::{BlockType, Expression},
    lexer::token::Token,
};

#[derive(Clone, Debug)]
pub enum TORE<'a> {
    Token(Token<'a>),
    Expr(Box<dyn Expression>),
}
pub trait SimplePattern {
    fn match_tore<'a>(&self, t: &TORE<'a>) -> Option<Option<Box<dyn Expression>>>;
}
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
        } else {
            //operator or similar
            None
        }
    }
}
pub struct ExprBuilder {
    patterns: Vec<Box<dyn SimplePattern>>,
    constructor: Box<dyn Fn(Vec<Box<dyn Expression>>) -> Box<dyn Expression>>,
}

impl ExprBuilder {
    pub fn new(
        patterns: Vec<Box<dyn SimplePattern>>,
        constructor: Box<dyn Fn(Vec<Box<dyn Expression>>) -> Box<dyn Expression>>,
    ) -> Self {
        Self {
            patterns,
            constructor,
        }
    }
    pub fn parse_occurences<'a>(&self, mut tokens: Vec<TORE<'a>>) -> Vec<TORE<'a>> {
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
            let expr = TORE::Expr((self.constructor)(params));
            tokens.splice(i..(i + self.patterns.len()), [expr]);
            i += 1;
        }
        tokens
    }
}
