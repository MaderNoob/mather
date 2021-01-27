mod numbers;

use std::str::Chars;

use bigdecimal::{BigDecimal, Zero};
use num_bigint::BigInt;

use super::cached_iter::CachedIterator;
pub enum Token{
    Variable(String),
    RightParentheses,
    LeftParentheses,
    Operator(String),
    Number(BigDecimal),
}

pub struct Lexer<'a>{
    input:CachedIterator<char,Chars<'a>>,
}

impl<'a> Lexer<'a>{
    
}