use crate::state::pieces::Piece;

use super::killer::SimpleMove;

/**
 * Transposition Table
 */
#[derive(PartialEq, Debug)]
pub enum NodeType {
    Pv,  //
    Cut, // fail-high nodes, are nodes in which a beta-cutoff was performed. The score returned is a lower bound (might be greater) on the exact score of the node
    All, // fail-low, otherwise known as fail-low nodes, are nodes in which no move's score exceeded alpha
    None,
}

#[derive(Debug)]
pub struct TTEntry {
    pub key: u64, // zobrist hash
    pub depth: u8,
    pub score: i32,
    pub best_move: SimpleMove,
    pub node: NodeType,
}

pub struct TTable {
    pub table: Vec<TTEntry>,
    // pub addressing_bits: usize,
    // pub addressing_mask: usize,
    pub size: usize,
}

impl TTable {
    pub fn init(size: usize) -> TTable {
        // minimum 1 maximum 31
        // let addressing_mask = (1 << addressing_bits) - 1;
        // let size = 2_usize.pow(addressing_bits as u32);
        let mut table = Vec::with_capacity(size);
        for _ in 0..size {
            table.push(TTEntry {
                key: 0, // zobrist hash
                depth: 0,
                score: 0,
                best_move: SimpleMove {
                    from: 0,
                    to: 0,
                    promotion: Piece::Empty,
                },
                node: NodeType::None,
            });
        }

        println!("Created Table Size {size}");

        TTable {
            table,
            size,
            // addressing_bits,
            // addressing_mask,
        }
    }
    pub fn probe(&self, key: u64, alpha: i32, beta: i32) -> Option<(SimpleMove, i32, u8)> {
        // let entry = &self.table[key as usize & self.addressing_mask];
        let index = key as usize % self.size;
        // let index = key as usize & self.addressing_mask;
        let entry = &self.table[index];
        // println!("{:?} {} {}", entry, key, entry.key);

        if entry.key == key {
            if entry.node == NodeType::Pv {
                return Some((entry.best_move.clone(), entry.score, entry.depth));
            }
            if entry.node == NodeType::Cut && entry.score >= beta {
                return Some((entry.best_move.clone(), entry.score, entry.depth));
            }
            if entry.node == NodeType::All && entry.score < alpha {
                return Some((entry.best_move.clone(), entry.score, entry.depth));
            }
        }
        return None;
    }
    pub fn record(
        &mut self,
        key: u64,
        best_move: SimpleMove,
        score: i32,
        depth: u8,
        node: NodeType,
    ) {
        let index = key as usize % self.size;
        // let index = key as usize & self.addressing_mask;

        self.table[index].key = key;
        self.table[index].best_move = best_move;
        self.table[index].score = score;
        self.table[index].depth = depth;
        self.table[index].node = node;

        // println!("Inserted: {:?}", self.table[index]);
    }
}
