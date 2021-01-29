mod chars_iter;
mod numbers;
mod operators;
mod parentheses;
mod util;
mod variables;

use super::errors::LexResult;
use bigdecimal::BigDecimal;
use chars_iter::CharsIterator;
use numbers::NumberTokenLexer;
use operators::{Operator, OperatorTokenLexer};
use parentheses::ParenthesesTokenLexer;
use std::collections::HashMap;
use variables::VariableTokenLexer;
#[derive(Debug)]
pub enum Token {
    Variable(String),
    RightParentheses,
    LeftParentheses,
    Operator(Operator),
    Number(BigDecimal),
}

pub struct Lexer {
    input: CharsIterator,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: CharsIterator::from_str(input),
        }
    }
    pub fn lex_any<T>(&mut self, variables: &HashMap<String, T>) -> LexResult<Token> {
        self.lex_any_parenthesis()
            .or_else(|| self.lex_operator_token())
            .or_else(|| self.lex_number_token())
            .or_else(|| self.lex_variable_token(variables))
    }
}
