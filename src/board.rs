use {
    super::util::{
        Marker,
        Coord,
        BOARD_SIZE
    }
};

pub struct Board {
    size: usize,
    cells: [[Marker; BOARD_SIZE]; BOARD_SIZE]
}
