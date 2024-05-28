use crate::state::player::Player;

pub trait Controller {
    fn should_stop(&self, in_search: bool, side: Player, nodes: u128, ply_or_depth: u8) -> bool;
}
