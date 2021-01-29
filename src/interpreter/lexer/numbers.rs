use super::super::errors::LexResult;
use super::{util::LexerUtil, Lexer, Token};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

const PREFIX_LITERALS: &[&str] = &["", "d", "0x", "h", "o", "b"];
trait NumberLexer {
    fn lex_number_base_prefix(&mut self) -> u32;
    fn lex_number_with_base(&mut self, base: u32) -> LexResult<BigDecimal>;
}

impl NumberLexer for Lexer {
    fn lex_number_base_prefix(&mut self) -> u32 {
        match self.lex_any_string_greedy(PREFIX_LITERALS).unwrap() {
            0 | 1 => 10,
            2 | 3 => 16,
            4 => 8,
            5 => 2,
            _ => panic!(),
        }
    }
    fn lex_number_with_base(&mut self, base: u32) -> LexResult<BigDecimal> {
        let mut current_int_number: Option<BigInt> = None;
        loop {
            match self.input.next() {
                None => {
                    return match current_int_number {
                        None => LexResult::EndOfInput,
                        Some(num) => LexResult::Ok(BigDecimal::from(num)),
                    }
                }
                Some(c) => {
                    if let Some(digit) = c.to_digit(base) {
                        match &mut current_int_number {
                            Some(num) => {
                                *num *= base;
                                *num += digit;
                            }
                            None => current_int_number = Some(BigInt::from(digit)),
                        }
                    } else if c == '.' {
                        break;
                    } else {
                        // unconsume the last character which is not a digit and not a '.'
                        self.input.index -= 1;
                        return match current_int_number {
                            Some(num) => LexResult::Ok(BigDecimal::from(num)),
                            None => LexResult::UnexpectedCharacter {
                                index: self.input.index,
                            },
                        };
                    }
                }
            }
        }
        let mut current_decimal_number = BigDecimal::from(current_int_number.unwrap());
        let mut current_decimal_exponent = 1i64;
        loop {
            match self.input.next() {
                None => return LexResult::Ok(current_decimal_number),
                Some(c) => {
                    if let Some(digit) = c.to_digit(base) {
                        current_decimal_number +=
                            BigDecimal::new(BigInt::from(digit), current_decimal_exponent);
                        current_decimal_exponent += 1;
                    } else {
                        // unconsume the last character which is not a digit
                        self.input.index -= 1;
                        return LexResult::Ok(current_decimal_number);
                    }
                }
            }
        }
    }
}

pub trait NumberTokenLexer {
    fn lex_number_token(&mut self) -> LexResult<Token>;
}
impl NumberTokenLexer for Lexer {
    fn lex_number_token(&mut self) -> LexResult<Token> {
        let base = self.lex_number_base_prefix();
        self.lex_number_with_base(base)
            .map(|num| Token::Number(num))
    }
}
