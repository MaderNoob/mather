use super::Lexer;
use crate::interpreter::errors::LexResult;

pub trait LexerUtil {
    fn lex_any_string_greedy(&mut self, strings: &[&str]) -> LexResult<usize>;
}
impl LexerUtil for Lexer {
    fn lex_any_string_greedy(&mut self, strings: &[&str]) -> LexResult<usize> {
        let start_index = self.input.index;
        let mut current_largest_string_found = None;
        let mut string_chars_iterators: Vec<_> =
            strings.iter().map(|s| (true, s.chars())).collect();
        let error = loop {
            match self.input.next() {
                None => {
                    // check if also reached the end of any of the strings
                    for (string_index, (keep_searching, chars)) in
                        string_chars_iterators.iter_mut().enumerate()
                    {
                        if *keep_searching {
                            if chars.next().is_none() {
                                current_largest_string_found = Some(string_index)
                            }
                        }
                    }
                    return if let Some(index) = current_largest_string_found {
                        LexResult::Ok(index)
                    } else {
                        LexResult::EndOfInput
                    };
                }
                Some(c) => {
                    let mut found_char_in_any_string = false;
                    let mut reached_end_of_all_strings = true;
                    for (string_index, (keep_searching, chars)) in
                        string_chars_iterators.iter_mut().enumerate()
                    {
                        if *keep_searching {
                            match chars.next() {
                                None => {
                                    // no need to keep searching because reached the end of the string
                                    *keep_searching = false;
                                    current_largest_string_found = Some(string_index)
                                }
                                Some(desired_char) => {
                                    reached_end_of_all_strings = false;
                                    if c == desired_char {
                                        found_char_in_any_string = true;
                                    } else {
                                        *keep_searching = false;
                                    }
                                }
                            }
                        }
                    }
                    if reached_end_of_all_strings {
                        // unconsume the last consumed character, since if reached end of all string this char
                        // was not even used
                        self.input.index -= 1;
                        return if let Some(index) = current_largest_string_found {
                            LexResult::Ok(index)
                        } else {
                            LexResult::UnexpectedCharacter {
                                index: self.input.index,
                            }
                        };
                    } else if !found_char_in_any_string {
                        // unconsume the non matching character
                        self.input.index -= 1;
                        return if let Some(index) = current_largest_string_found {
                            LexResult::Ok(index)
                        } else {
                            LexResult::UnexpectedCharacter {
                                index: self.input.index,
                            }
                        };
                    }
                }
            }
        };
    }
}
