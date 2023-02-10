#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Number,
    Word,
    Single,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub slice: &'a str,
}
impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, slice: &'a str) -> Token<'a> {
        Token { token_type, slice }
    }
}
