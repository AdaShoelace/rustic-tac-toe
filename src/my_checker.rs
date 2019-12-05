use {
    crate::{
        board_checker::{BoardChecker, ScoreState},
        util::Marker,
        player::Player,
        board::Board
    }
};


pub struct MyChecker;

impl BoardChecker for MyChecker {
    fn check(&self, board: &Board, player_x: &Player, player_o: &Player) -> ScoreState {
        unimplemented!()
    }
}
