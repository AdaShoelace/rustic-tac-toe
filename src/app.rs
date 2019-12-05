use std::io;
use crate::{
    state::Game,
    player::Player,
    board_checker::{
        BoardChecker,
        ScoreState
    },
    util::{
        Marker,
        Coord,
        BOARD_SIZE
    }
};

pub struct App {
    player_x: Player,
    player_o: Player,
    game: Game,
    board_checker: Box<dyn BoardChecker>
}


impl App {
    pub fn new(player_x: Player, player_o: Player, board_checker: Box<dyn BoardChecker>) -> Self {
        Self {
            game: Game::new(player_x.clone()),
            player_x,
            player_o,
            board_checker
        }
    }

    pub fn run(mut self) {
        loop {
            println!("{}", self.game.board);
            match self.board_checker.check(&self.game.board, &self.player_x, &self.player_o) {
                ScoreState::Won(player) => {
                    println!("Player: {} won!", player.name);
                    break;
                },
                ScoreState::Tie => {
                    println!("The game ended in a tie!");
                    break;
                },
                ScoreState::Continue => ()
            }
            let last_move = get_move_input(&self.game.current_player);
            match self.game.board.clone().add_marker(last_move.0, last_move.1) {
                (board, true) => {
                    self.game.board = board;
                    if self.game.current_player == self.player_x {
                        self.game.current_player = self.player_o.clone()
                    } else {
                        self.game.current_player = self.player_x.clone()
                    }
                },
                (_, false) => {
                    println!("Illegal move, coordinate already populated");
                }
            }
        }
    }
}


fn get_move_input(current_player: &Player) -> (Coord, Marker){
    loop {
        println!("Please enter coordinates separated with a comma eg 0,1");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                match input.trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>().as_slice() {
                    [first, second] if first.parse::<usize>().is_ok() && second.parse::<usize>().is_ok() => {
                        let (first, second) = (first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap());
                        if !(0..BOARD_SIZE).contains(&first) || !(0..BOARD_SIZE).contains(&second) {
                            println!("Coordniate out of bounds: {},{}", first, second);
                            continue
                        }
                        return (Coord::new(first, second), current_player.marker)
                    },
                    x => {
                        println!("Unsupported input: {:?}", x)
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
