use std::time::SystemTime;

use xorshift::{Rand, Rng, SeedableRng, SplitMix64, Xoroshiro128};

use crate::state::{boards::BitBoard, game::GameState, pieces::Piece, player::Player};

pub struct ZobristHasher {
    pub pieces: [[[u64; 64]; 7]; 2],
    pub enpassant: [u64; 65],
    pub castling: [u64; 16],
    pub side_to_move: u64,
}

impl ZobristHasher {
    pub fn init() -> ZobristHasher {
        let seed: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut sm: SplitMix64 = SeedableRng::from_seed(seed);
        let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

        let mut pieces = [[[0_u64; 64]; 7]; 2];
        let mut enpassant = [0_u64; 65];
        let mut castling = [0_u64; 16];
        let side_to_move = rng.next_u64();

        // piece keys
        for sq in 0..64 {
            let empty_key = 0;
            pieces[0][0][sq] = empty_key;
            pieces[1][0][sq] = empty_key;

            for piece in 1..7 {
                for side in 0..2 {
                    pieces[side][piece][sq] = rng.next_u64();
                }
            }
        }

        // enpassant
        for sq in 0..64 {
            enpassant[sq] = rng.next_u64();
        }

        // castling
        for permission in 0..16 {
            castling[permission] = rng.next_u64();
        }

        ZobristHasher {
            pieces,
            enpassant,
            castling,
            side_to_move,
        }
    }
}

impl ZobristHasher {
    pub fn hash(&self, game: &GameState) -> u64 {
        let mut hash = 0;

        hash ^= self.enpassant[game.enpassant_square.trailing_zeros() as usize];
        hash ^= self.castling[game.castle_permissions as usize];

        if game.side_to_move == Player::White {
            hash ^= self.side_to_move;
        }

        // piece keys
        for sq in 0..64 {
            if game.bitboards.pos_to_piece[sq] != Piece::Empty {
                let piece = game.bitboards.pos_to_piece[sq] as usize;
                let side = if game.bitboards.pos_to_player[Player::White as usize].get(sq as i8) {
                    Player::White as usize
                } else {
                    Player::Black as usize
                };
                hash ^= self.pieces[side as usize][piece][sq];
            }
        }

        hash
    }
}