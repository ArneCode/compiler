pub mod token;
use token::*;
// TODO: make this a peekable (https://doc.rust-lang.org/std/iter/struct.Peekable.html), so that it is easier to understand
struct CharStream {
    c_iter: std::vec::IntoIter<char>,
    curr_c: Option<char>,
    skip_next: bool,
}
impl Iterator for CharStream {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.skip_next {
            self.curr_c = self.c_iter.next();
        } else {
            self.skip_next = false;
        }
        self.curr_c
    }
}
impl CharStream {
    fn new(s: &str) -> Self {
        CharStream {
            c_iter: s.chars().collect::<Vec<char>>().into_iter(),
            curr_c: None,
            skip_next: false,
        }
    }
    fn count_bytes_while<F>(&mut self, mut cond: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut maybe_c = self.next(); //possibly a character
        let mut n_bytes = 0;
        while let Some(c) = maybe_c {
            if !cond(c) {
                break;
            }
            n_bytes += c.len_utf8();
            maybe_c = self.next();
        }
        self.skip_next = true;
        n_bytes
    }
}
pub fn lex(s: &str) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(s.len() / 2 + 2);
    let mut stream = CharStream::new(s);
    let mut i_bytes = 0;
    while let Some(c) = stream.next() {
        let mut token_length = c.len_utf8();
        let mut token_type = TokenType::Single;
        match c {
            'a'..='z' | 'A'..='Z' => {
                token_length +=
                    stream.count_bytes_while(|c| matches!(c,'0'..='9'|'a'..='z'|'A'..='Z'));
                token_type = TokenType::Word;
            }
            '0'..='9' => {
                //maybe refactor to seperate function?
                token_length += stream.count_bytes_while(|c| matches!(c, '0'..='9'));
                if let Some('.') = stream.next() {
                    let after_dot = stream.count_bytes_while(|c| matches!(c, '0'..='9'));
                    if after_dot == 0 {
                        //dot isn't supposed to be part of number, wether this represents an error get's handled by the parser
                        let slice_num = &s[i_bytes..i_bytes + token_length];
                        i_bytes += token_length;
                        let token_num = Token::new(TokenType::Number, slice_num);
                        tokens.push(token_num);
                        let slice_dot = &s[i_bytes..i_bytes + 1];
                        i_bytes += 1;
                        let token_dot = Token::new(TokenType::Single, slice_dot);
                        tokens.push(token_dot);
                        continue;
                    }
                    token_length += 1 + after_dot;
                } else {
                    stream.skip_next = true;
                }
                token_type = TokenType::Number;
            }
            c if c.is_whitespace() => {
                i_bytes += 1;
                continue;
            }
            _ => (),
        }
        let slice = &s[i_bytes..i_bytes + token_length];
        i_bytes += token_length;
        let token = Token::new(token_type, slice);
        tokens.push(token);
    }
    tokens
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenizer_simple() {
        let s = "3.5*abc".to_string();
        let result = lex(&s);
        assert_eq!(
            result,
            vec![
                Token::new(TokenType::Number, "3.5"),
                Token::new(TokenType::Single, "*"),
                Token::new(TokenType::Word, "abc")
            ]
        );
    }
    #[test]
    fn ignores_whitespace() {
        let s1 = String::from("a+b*(5-7)");
        let s2 = String::from(" a + b         \n \t    *           (5 -7) ");
        assert_eq!(lex(&s1), lex(&s2));
    }
}
