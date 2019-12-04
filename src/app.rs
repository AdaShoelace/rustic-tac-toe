use std::io;
use crate::{
    state::Game,
    player::Player,
    board_checker::{
        BoardChecker,
        DefaultChecker,
    },
    my_checker::MyChecker,
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
            board_checker: {
                if cfg!(feature="default") {
                    Box::new(DefaultChecker{})
                } else {
                    println!("MyChecker");
                    board_checker
                }
            }
        }
    }

    pub fn run(mut self) {
        while self.game.is_playing {
            println!("{}", self.game.board);
            let last_move = get_move_input(&self.game.current_player);
            self.game.board = self.game.board.add_marker(last_move.0, last_move.1);
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
