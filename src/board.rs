use {
    super::util::{
        Marker,
        Coord,
        Point,
        BOARD_SIZE
    }
};

#[derive(Clone)]
pub struct Board {
    cells: [[Marker; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    pub fn add_marker<T: Into<Coord> + Point>(mut self, coord: T, marker: Marker) -> Self {
        self.cells[coord.x()][coord.y()] = marker;
        self
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Marker::Empty; BOARD_SIZE]; BOARD_SIZE]
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = {
            let mut separator = String::new();
            (0..=4 * BOARD_SIZE).for_each(|_| {
                separator.push_str("-");
            });
            separator
        };

        let board = {
            let mut board = String::new();
            board.push_str(&format!("\t{:3} {:3} {:3}\n", 0, 1, 2));
            board.push_str(&format!("\t{}\n", separator));
            self.cells
                .to_vec()
                .iter()
                .enumerate()
                .for_each(|(i, row)| {
                    board.push_str(&format!("{}\t|", i));
                    row.iter()
                        .for_each(|cell| {
                            board.push_str(&format!(" {} |", cell));
                        });
                    board.push_str(&format!("\n\t{}\n", separator));
                });
            board
        };

        write!(f, "{}", board)
    }
}
