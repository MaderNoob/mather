use bigdecimal::{BigDecimal, FromPrimitive};
use interpreter::Lexer;
use num_bigint::BigInt;
use std::collections::HashMap;

mod interpreter;
fn main() {
    let mut variables = HashMap::new();
    variables.insert("ab".to_string(), 5);
    variables.insert("a".to_string(), 5);
    let mut lexer = Lexer::new("2.5*(abcs+1+2)");
    loop {
        let result = lexer.lex_any(&variables);
        println!("{:?}", result);
        if !result.is_ok() {
            break;
        }
    }
}
