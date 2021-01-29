use super::super::errors::LexResult;
use super::{util::LexerUtil, Lexer, Token};
pub trait ParenthesesTokenLexer {
    fn lex_any_parenthesis(&mut self) -> LexResult<Token>;
}

impl ParenthesesTokenLexer for Lexer {
    fn lex_any_parenthesis(&mut self) -> LexResult<Token> {
        match self.input.next() {
            Some(c) => {
                if c == '(' {
                    LexResult::Ok(Token::LeftParentheses)
                } else if c == ')' {
                    LexResult::Ok(Token::RightParentheses)
                } else {
                    self.input.index -= 1;
                    LexResult::UnexpectedCharacter {
                        index: self.input.index,
                    }
                }
            }
            None => LexResult::EndOfInput,
        }
    }
}
