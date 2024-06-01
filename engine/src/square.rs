#[derive(Debug, Default, Clone, Copy)]
pub struct Square {
    pub rank: i8,
    pub file: i8,
}

impl Square {
    pub fn rank(pos: i8) -> i8 {
        pos / 8
    }
    pub fn file(pos: i8) -> i8 {
        pos % 8
    }
    pub fn rank_and_file(pos: i8) -> (i8, i8){
        let rank = pos / 8;
        let file = pos % 8;
        (rank, file)
    }
    pub fn index(rank: i8, file: i8) -> i8 {
        rank * 8 + file
    }
  
    pub fn stringify(&self) -> String {
        let rank = (self.rank as u8 + '1' as u8) as char;
        let file = (self.file as u8 + 'a' as u8) as char;
        format!("{file}{rank}")
    }
    pub fn parse_string(input: String) -> Result<Square, String> {
        if input.len() != 2 {
            return Err("INVALID".into());
        }

        let rank = (input.chars().nth(1).unwrap() as u32 - '1' as u32) as i8;
        let file = (input.chars().nth(0).unwrap() as u32 - 'a' as u32) as i8;

        if 0 <= rank && rank <= 7 && 0 <= file && file <= 7 {
            return Ok(Square { rank, file });
        }

        return Err("INVALID".into());
    }
}

impl From<i8> for Square {
    fn from(pos: i8) -> Self {
        Square {
            rank: pos / 8,
            file: pos % 8,
        }
    }
}

impl Into<i8> for Square {
    fn into(self) -> i8 {
        8 * self.rank + self.file
    }
}
