pub struct CharsIterator {
    chars: Vec<char>,
    pub index: usize,
}
impl CharsIterator {
    pub fn from_str(s: &str) -> CharsIterator {
        CharsIterator {
            chars: s.chars().collect(),
            index: 0,
        }
    }
}
impl Iterator for CharsIterator {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.get(self.index) {
            Some(&c) => {
                self.index += 1;
                Some(c)
            }
            None => None,
        }
    }
}
