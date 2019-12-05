use {
    crate::{
        board_checker::{BoardChecker, ScoreState},
        player::Player,
        board::Board
    }
};


pub struct MyChecker;

impl BoardChecker for MyChecker {
    fn check(&self, _board: &Board, _player_x: &Player, _player_o: &Player) -> ScoreState {
        unimplemented!()
    }
}
