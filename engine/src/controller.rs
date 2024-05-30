use crate::state::player::Player;

pub trait Controller {
    fn should_stop(
        &self,
        in_search: bool,
        side: Player,
        nodes: u64,
        ply_or_depth: u8,
        ignore_terminated_flag: bool,
    ) -> bool;
}
