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
        let (first, last) = {
            let mut first = String::new();
            let mut last = String::new();
            (0..=4 * BOARD_SIZE).for_each(|_| {
                first.push_str("-");
                last.push_str("-");
            });
            (first, last)
        };

        let board = {
            let mut board = String::new();

            self.cells
                .to_vec()
                .iter()
                .enumerate()
                .for_each(|(i, row)| {
                    board.push_str("|");
                    row.iter()
                        .for_each(|cell| {
                            board.push_str(&format!(" {} |", cell));
                        });
                    if i != BOARD_SIZE - 1 {
                        board.push_str("\n")
                    }
                });
            board
        };

        write!(f, "{}\n{}\n{}\n", first, board, last)
    }
}
