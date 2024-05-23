use std::ops::Index;
use std::rc::Rc;
use super::StringReader;

impl Index<usize> for StringReader {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        if self.head + index < self.chars.len() {
            &self.chars[self.head + index]
        } else {
            &'\0'
        }
    }
}

impl StringReader {
    pub fn from_str(str: &str) -> Self {
        Self {
            chars: Rc::new(str.chars().collect()),
            head: 0
        }
    }

    pub fn from_string(str: String) -> Self {
        Self {
            chars: Rc::new(str.chars().collect()),
            head: 0,
        }
    }

    pub fn move_head(&self, amount: isize) -> Option<Self> {
        if amount > 0 {
            let mut n = self.clone();
            n.head += amount as usize;
            if n.head > n.chars.len() {
                None
            } else {
                Some(n)
            }
        } else if amount < 0 {
            let mut n = self.clone();
            let d = -amount as usize;
            if d > n.head {
                None
            } else {
                n.head -= d;
                Some(n)
            }
        } else {
            Some(self.clone())
        }
    }

    pub fn skip_whitespaces(self) -> Self {
        let mut i = self.head;
        while i < self.chars.len() && self.chars[i].is_whitespace() {
            i += 1;
        }
        if i != self.head {
            Self {
                chars: self.chars.clone(),
                head: i
            }
        } else {
            self
        }
    }

    pub fn finished(self) -> bool {
        self.head >= self.chars.len()
    }
}