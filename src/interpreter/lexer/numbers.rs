enum NumberPrefix{
    Decimal,
    Hex,
    Binary,
    Octal,
}
const PREFIX_LITERALS:[&str]=["0x","h","o","b","B"]
pub trait NumberLexer{
    fn lex_prefix(&mut self)->NumberPrefix{

        match self
    }