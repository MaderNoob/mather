use super::super::errors::LexResult;
use super::{util::LexerUtil, Lexer, Token};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
}

// note that the order here must match the order of the Operator enum's definition so that TryFromPrimitive will work
// properly when given the string index returned from the lex_any_string_greedy on the operator literals slice
const OPERATOR_LITERALS: &[&str] = &["+", "-", "*", "/", "="];
pub trait OperatorTokenLexer {
    fn lex_operator_token(&mut self) -> LexResult<Token>;
}

impl OperatorTokenLexer for Lexer {
    fn lex_operator_token(&mut self) -> LexResult<Token> {
        self.lex_any_string_greedy(OPERATOR_LITERALS)
            .map(|op_index| Token::Operator(Operator::try_from(op_index).unwrap()))
    }
}
