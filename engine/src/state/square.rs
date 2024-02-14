#[derive(Debug, Default, Clone, Copy)]
pub struct Square {
    pub rank: i8,
    pub file: i8,
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
