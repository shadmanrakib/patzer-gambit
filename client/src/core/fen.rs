use std::{fmt::{self, Display}, vec};

#[derive(Debug)]
pub struct FenNotation {
    raw_value: String,
    piece_placement: String,
    active_color: String,
    castling_availability: String,
    en_passant_target_square: String,
    halfmove_clock: String,
    fullmove_number: String,
}

#[derive(Debug)]
pub enum FenError {
    IncorrectLength,
    MissingPart,
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::IncorrectLength => "Error in FEN string: Must be 6 parts",
            Self::MissingPart => {
                // want to do Missing Part(part_num, message) but rust is being a pain
                // TODO figure out why this isnt working lol
                "Y U NO WORK?!?!??"
                // const PARTS: [&str; 6] = [
                //     "Piece Placement",
                //     "Active Color",
                //     "Castling Availability",
                //     "En Passant Target Square",
                //     "Halfmove Clock",
                //     "Fullmove Number",
                // ];
                // let error_str = format!(
                //     "Error in FEN string: Missing part {}: {}",
                //     PARTS[*part as usize], message
                // );

                // return error_str.as_str();
            }
        };
        write!(f, "{error}")
    }
}

impl FenNotation {
    // black pieces are denoted using lowercase letters
    // white pieces use uppercase letters
    // empty squares are noted using digits 1-8
    // '/' separates ranks
    // ' ' separates the board, active color, castling availability, en passant target square, halfmove clock, and fullmove number fields

    pub fn new(fen: &str) -> FenNotation {
       build_fen(fen)   
    }

    fn update_fen(&mut self, new_fen: &str) {
       *self = build_fen(new_fen)
    }


    pub fn get_board_vec(&self) -> Vec<Vec<String>> {
        let vec: Vec<&str> = self.piece_placement.split("/").collect();
        let mut board_vec: Vec<Vec<String>> = Vec::new();

        for row in vec {
            let mut row_vec: Vec<String> = Vec::new();
            
            for c in row.chars() {
                if c.is_digit(10) {
                    let num = c.to_digit(10).unwrap();
                    row_vec.append(&mut vec![String::from(" "); num as usize]);
                } else {
                    row_vec.push(c.to_string());
                }
            }

            board_vec.push(row_vec);
        }   
        
        return board_vec;
    }


}

fn build_fen(fen: &str) -> FenNotation {
    let fen_parts = split_fen(fen).unwrap();

    let piece_placement = fen_parts[0].to_string();
    let active_color = fen_parts[1].to_string();
    let castling_availability = fen_parts[2].to_string();
    let en_passant_target_square = fen_parts[3].to_string();
    let halfmove_clock = fen_parts[4].to_string();
    let fullmove_number = fen_parts[5].to_string();


    FenNotation {
        raw_value: fen.to_string(),
        piece_placement,
        active_color,
        castling_availability,
        en_passant_target_square,
        halfmove_clock,
        fullmove_number,
    }
}



fn split_fen(fen: &str) -> Result<Vec<&str>, FenError> {
    let mut split_fen: Vec<&str> = fen.split(" ").collect();

    let vec_len = split_fen.len();

    if vec_len != 6 {
        return Err(FenError::IncorrectLength);
    }
    
    if vec_len == 4 {
        split_fen.append(&mut vec!["0", "1"]);
    }

    Ok(split_fen)
}
