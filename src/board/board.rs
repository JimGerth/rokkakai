//! Board submodule

use crate::cell::{Cell, Side};

/// The game board
pub struct Board {
    /// The cells 
    cells: Vec<Option<Cell>>,

    /// The number of cell positions in one row of the board.
    width: usize,
}

impl Board {
    fn get_cell(&self, r: usize, c: usize) -> &Option<Cell> {
        self.cells.get(r * self.width + c).unwrap()
    }

    fn get_neighbour(&self, x: usize, y: usize, side: Side) -> Option<Cell> {
        match side {
            Side::A =>
            Side::B =>
            Side::C =>
            Side::D =>
            Side::E =>
            Side::F =>
        }
    }
}
