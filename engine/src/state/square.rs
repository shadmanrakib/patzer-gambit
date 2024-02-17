#[derive(Debug, Default, Clone, Copy)]
pub struct Square {
    pub rank: i8,
    pub file: i8,
}

impl Square {
    pub fn stringify(&self) -> String {
        let rank = (self.rank as u8 + '1' as u8) as char;
        let file = (self.file as u8 + 'a' as u8) as char;
        format!("{file}{rank}")
    }
}

impl From<i8> for Square {
    fn from(pos: i8) -> Self {
        Square{rank: pos / 8, file: pos % 8}
    }
}

impl Into<i8> for Square {
    fn into(self) -> i8 {
        8 * self.rank + self.file
    }
}
