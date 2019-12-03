pub const BOARD_SIZE: usize = 3;

pub fn set_board_size() -> usize {
    use std::io;
    println!("hello");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    input.parse::<usize>().unwrap()
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Coord(usize, usize);

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

impl Into<Coord> for (usize, usize) {
    fn into(self) -> Coord {
        Coord::new(self.0, self.1)
    }
}

#[derive(Clone, Copy)]
pub enum Marker {
    X,
    O,
}
