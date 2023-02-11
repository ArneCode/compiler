use crate::{
    expression::{
        statements::{Number, FrameLayer, Var},
        BlockType, CodeBlock, Expression,
    },
    lexer::{
        lex,
        token::{Token, TokenType},
    },
    pattern::{ExprBuilder, TORE},
};
fn find_matching_bracket<'a>(
    tokens_or_expr: &Vec<TORE<'a>>,
    (open, close): (&str, &str),
    brack_start: usize,
) -> usize {
    //check if token at brack_start really is the open bracket
    assert!(
        matches!(
            &tokens_or_expr[brack_start],
            TORE::Token(
                Token {
                    token_type:_,
                    slice
                })
            if slice == &open
        ),
        "find_matching_brack called with another token than {} at brack_start",
        open
    );
    let mut level = 0;
    for (i, token_or_expr) in tokens_or_expr.iter().enumerate() {
        if let TORE::Token(token) = token_or_expr {
            match token.slice {
                s if s == open => level += 1,
                s if s == close => {
                    level -= 1;
                    if level == 0 {
                        return i;
                    }
                }
                _ => (),
            }
        }
    }
    panic!("unmatched opening bracket");
}
fn parse_braces<'a>(
    mut tokens_or_expr: Vec<TORE<'a>>,
    builders: &Vec<ExprBuilder>,
    frame: &mut FrameLayer,
) -> Vec<TORE<'a>> {
    //parsing brackets and functions
    let mut i = 0;
    while i < tokens_or_expr.len() {
        if let TORE::Token(token) = tokens_or_expr[i].clone() {
            if token.slice == "{" {
                let brack_end = find_matching_bracket(&tokens_or_expr, ("{", "}"), i);
                let nodes = tokens_or_expr[i + 1..brack_end].to_vec();
                //parsing the tokens into an expression
                let lines = parse_tokens(nodes.to_vec(), builders, frame);
                let block = Box::new(CodeBlock::new(lines, BlockType::Curl));
                //replacing the tokens with the expression, delete the brackets as well
                tokens_or_expr.splice(i..=brack_end, vec![TORE::Expr(block)]);
            }
        }
        i += 1;
    }
    tokens_or_expr
}
fn parse_brackets<'a>(
    mut tokens_or_expr: Vec<TORE<'a>>,
    builders: &Vec<ExprBuilder>,
    frame: &mut FrameLayer,
) -> Vec<TORE<'a>> {
    //parsing brackets and functions
    let mut i = 0;
    while i < tokens_or_expr.len() {
        if let TORE::Token(token) = tokens_or_expr[i].clone() {
            if token.slice == "(" {
                let brack_end = find_matching_bracket(&tokens_or_expr, ("(", ")"), i);
                let nodes = tokens_or_expr[i + 1..brack_end].to_vec();
                //parsing the tokens into an expression
                let lines = parse_tokens(nodes.to_vec(), builders, frame);
                let block = Box::new(CodeBlock::new(lines, BlockType::Brack));
                //replacing the tokens with the expression, delete the brackets as well
                tokens_or_expr.splice(i..=brack_end, vec![TORE::Expr(block)]);
            }
        }
        i += 1;
    }
    tokens_or_expr
}
fn parse_tokens(
    mut tokens: Vec<TORE>,
    builders: &Vec<ExprBuilder>,
    frame: &mut FrameLayer,
) -> Vec<Box<dyn Expression>> {
    tokens = parse_braces(tokens, builders, frame);
    tokens = parse_brackets(tokens, builders, frame);
    for builder in builders {
        tokens = builder.parse_occurences(tokens, frame);
    }
    println!("{:#?}", tokens);
    //make lines
    let lines = tokens
        .into_iter()
        .filter_map(|token| match token {
            TORE::Token(t) => {
                if t.slice == ";" {
                    None
                } else if t.token_type == TokenType::Word {
                    let name = t.slice.to_string();
                    let addr = frame.get_addr(&name);
                    let var: Box<dyn Expression> = Box::new(Var::new(name, addr));
                    Some(var)
                } else {
                    panic!("unexpected token: {}", t.slice)
                }
            }
            TORE::Expr(e) => Some(e),
        })
        .collect::<Vec<_>>();
    lines
}
pub fn parse_nums(tokens: Vec<TORE>) -> Vec<TORE> {
    tokens
        .into_iter()
        .map(|t| match t {
            TORE::Token(Token {
                token_type: TokenType::Number,
                slice,
            }) => TORE::Expr(Box::new(Number(slice.to_string()))),
            _ => t,
        })
        .collect()
}
pub fn parse(code: String, builders: &Vec<ExprBuilder>) -> (Box<dyn Expression>, FrameLayer) {
    let tokens = lex(&code);
    let tokens_or_expr = tokens.into_iter().map(|t| TORE::Token(t)).collect();
    let tokens_or_expr = parse_nums(tokens_or_expr);
    println!("tokens: {:#?}", tokens_or_expr);
    let mut frame = FrameLayer::new();
    let lines = parse_tokens(tokens_or_expr, builders, &mut frame);
    let code = CodeBlock::new(lines, BlockType::Curl);
    (Box::new(code),frame)
}
