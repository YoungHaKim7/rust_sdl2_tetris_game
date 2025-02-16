use rand::{
    prelude::{SliceRandom, ThreadRng},
    rng,
};

use piece::{Kind as PieceKind, Piece};
mod piece;

type Coordinate = cgmath::Vector2<usize>;
type Offset = cgmath::Vector2<isize>;

pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::blank(),
            bag: Vec::new(),
            rng: rng(),
            cursor: None,
        }
    }

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng)
    }

    fn place_cursor() {
        // Assert that the piece does not overlap filled cells
    }
}

struct Board([bool; Self::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn in_bounds(Coordinate { x, y }: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }
}
