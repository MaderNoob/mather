use super::super::errors::LexResult;
use super::{Lexer, Token};
use std::collections::HashMap;
pub trait VariableTokenLexer {
    fn lex_variable_token<T>(&mut self, variables: &HashMap<String, T>) -> LexResult<Token>;
}
impl VariableTokenLexer for Lexer {
    fn lex_variable_token<T>(&mut self, variables: &HashMap<String, T>) -> LexResult<Token> {
        let mut current_variable = String::new();
        let mut last_matching_variable_length = None;
        loop {
            match self.input.next() {
                Some(c) => {
                    if c.is_alphanumeric() || c == '_' {
                        current_variable.push(c);
                        if variables.contains_key(current_variable.as_str()) {
                            last_matching_variable_length = Some(current_variable.len())
                        }
                    } else {
                        // unconsome the non-variable-name character
                        self.input.index -= 1;
                        return if current_variable.len() == 0 {
                            LexResult::UnexpectedCharacter {
                                index: self.input.index,
                            }
                        } else {
                            match last_matching_variable_length {
                                Some(variable_length) => {
                                    // return the longest matched variable, and don't forget to modify the index accordingly
                                    self.input.index =
                                        self.input.index - current_variable.len() + variable_length;
                                    current_variable.truncate(variable_length);
                                    LexResult::Ok(Token::Variable(current_variable))
                                }
                                None => {
                                    // if no variable matched, assume the the variable that the user meant to use
                                    // was of length 1, and don't forget to modify the index accordingly
                                    self.input.index =
                                        self.input.index - current_variable.len() + 1;
                                    current_variable.truncate(1);
                                    LexResult::UndefinedVariable(current_variable)
                                }
                            }
                        };
                    }
                }
                None => return LexResult::EndOfInput,
            }
        }
    }
}
