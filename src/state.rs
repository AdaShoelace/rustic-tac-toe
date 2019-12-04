use {
    super::player::Player,
    super::board::Board,
};

#[derive(Clone)]
pub struct Game {
    pub is_playing: bool,
    pub current_player: Player,
    pub board: Board
}

impl Game {
    pub fn new(first_player: Player) -> Self {
        Self {
            is_playing: false,
            current_player: first_player,
            board: Default::default()
        }
    }
}
