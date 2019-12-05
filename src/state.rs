use {
    super::player::Player,
    super::board::Board,
};

#[derive(Clone)]
pub struct Game {
    pub current_player: Player,
    pub board: Board
}

impl Game {
    pub fn new(first_player: Player) -> Self {
        Self {
            current_player: first_player,
            board: Default::default()
        }
    }
}
