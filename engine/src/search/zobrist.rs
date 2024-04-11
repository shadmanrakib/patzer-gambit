use std::time::SystemTime;

use xorshift::{Rand, Rng, SeedableRng, SplitMix64, Xoroshiro128};

use crate::state::{boards::BitBoard, game::GameState};

pub struct ZobristRandomKeys {
    pub pieces: [[[u64; 64]; 7]; 2],
    pub enpassant: [u64; 64],
    pub castling: [u64; 16],
    pub side_to_move: [u64; 2],
}

impl ZobristRandomKeys {
    pub fn init() -> ZobristRandomKeys {
        let seed: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut sm: SplitMix64 = SeedableRng::from_seed(seed);
        let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

        let mut pieces = [[[0_u64; 64]; 7]; 2];
        let mut enpassant = [0_u64; 64];
        let mut castling = [0_u64; 16];
        let side_to_move = [0, rng.next_u64()];

        // piece keys
        for sq in 0..64 {
            let empty_key = rng.next_u64();
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

        ZobristRandomKeys {
            pieces,
            enpassant,
            castling,
            side_to_move,
        }
    }
}

pub fn calculate_zobrist_hash(game: &GameState, keys: &ZobristRandomKeys) -> u64 {
    let mut hash = 0;

    let hash_enpassant_sq = std::cmp::min(game.enpassant_square.trailing_zeros(), 63) as usize;
    hash ^= keys.enpassant[hash_enpassant_sq];

    hash ^= keys.castling[game.castle_permissions as usize];

    hash ^= keys.side_to_move[game.side_to_move as usize];

    // piece keys
    for sq in 0..64 {
        let piece = game.bitboards.pos_to_piece[sq] as usize;
        let side = if game.bitboards.pos_to_player[0].get(sq as i8) {
            0
        } else {
            1
        };
        hash ^= keys.pieces[side as usize][piece][sq];
    }

    hash
}
