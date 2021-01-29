#[derive(Debug)]
pub enum LexResult<T> {
    Ok(T),
    UnexpectedCharacter { index: usize },
    UndefinedVariable(String),
    EndOfInput,
}

impl<T> LexResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            LexResult::Ok(v) => v,
            LexResult::UnexpectedCharacter { index } => panic!(
                "called `LexResult::unwrap()` on an `UnexpectedCharacter` value with index {:?}",
                index
            ),
            LexResult::UndefinedVariable(name) => panic!(
                "called `LexResult::unwrap()` on an `UndefinedVariable` value: {:?}",
                name
            ),
            LexResult::EndOfInput => {
                panic!("called `LexResult::unwrap()` on an `EndOfInput` value")
            }
        }
    }
    pub fn map<O, F: FnOnce(T) -> O>(self, f: F) -> LexResult<O> {
        match self {
            LexResult::Ok(v) => LexResult::Ok(f(v)),
            LexResult::UnexpectedCharacter { index } => LexResult::UnexpectedCharacter { index },
            LexResult::UndefinedVariable(name) => LexResult::UndefinedVariable(name),
            LexResult::EndOfInput => LexResult::EndOfInput,
        }
    }
    pub fn or_else<F: FnOnce() -> LexResult<T>>(self, op: F) -> LexResult<T> {
        match self {
            LexResult::Ok(v) => LexResult::Ok(v),
            LexResult::EndOfInput => LexResult::EndOfInput,
            LexResult::UndefinedVariable(name) => LexResult::UndefinedVariable(name),
            LexResult::UnexpectedCharacter { .. } => match op() {
                LexResult::Ok(v) => LexResult::Ok(v),
                LexResult::EndOfInput => LexResult::EndOfInput,
                LexResult::UndefinedVariable(name) => LexResult::UndefinedVariable(name),
                // return the original error
                LexResult::UnexpectedCharacter { .. } => self,
            },
        }
    }
    pub fn is_ok(&self) -> bool {
        match self {
            LexResult::Ok(_) => true,
            _ => false,
        }
    }
}
