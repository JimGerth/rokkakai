//! Exposes the Rokkakari game board logic and interactivity.
//!
//!   (0,0)   (1,0)   (2,0)   (3,0)   (4,0)
//!
//!
//!       (0,1)   (1,1)   (2,1)   (3,1)   (4,1)                   F        A
//!
//!
//!   (0,2)   (1,2)   (2,2)   (3,2)   (4,2)                   E      self      B
//!
//!
//!       (0,3)   (1,3)   (2,3)   (3,3)   (4,4)                   D        C
//!
//!
//!   (0,4)   (1,4)   (2,4)   (3,4)   (4,4)

use crate::cell;

struct Board {
    cells: Vec<Vec<Option<cell::Cell>>>,
    input_mode: InputMode,
}

enum InputMode {
    Light,
    Cross,
    Clear,
}

#[derive(Clone, Copy)]
enum Side {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Side {
    fn all() -> Vec<Side> {
        vec![Self::A, Self::B, Self::C, Self::D, Self::E, Self::F]
    }

    fn opposite(&self) -> Side {
        match self {
            Self::A => Self::D,
            Self::B => Self::E,
            Self::C => Self::F,
            Self::D => Self::A,
            Self::E => Self::B,
            Self::F => Self::C,
        }
    }
}

impl Board {
    ///
    ///       F    A
    ///
    ///    E   self   B
    ///
    ///       D    C
    ///
    fn get_neighbour_position(position: (u32, u32), side: Side) -> (u32, u32) {
        let (x, y) = position;
        match side {
            Side::A => (if y % 2 == 0 { x } else { x + 1 }, y - 1),
            Side::B => (x + 1, y),
            Side::C => (if y % 2 == 0 { x } else { x + 1 }, y + 1),
            Side::D => (if y % 2 == 0 { x - 1 } else { x }, y + 1),
            Side::E => (x - 1, y),
            Side::F => (if y % 2 == 0 { x - 1 } else { x }, y - 1),
        }
    }

    fn get_neighbour(&self, position: (u32, u32), side: Side) -> Option<&cell::Cell> {
        self.get_cell(Self::get_neighbour_position(position, side))
    }

    /// Returns a reference to the cell at the given game board position.
    ///
    /// Returns None if the position is out of bounds of the current game
    /// or if there is no cell at that position of the game board.
    fn get_cell(&self, position: (u32, u32)) -> Option<&cell::Cell> {
        let (x, y) = position;
        self.cells.get(y as usize)?.get(x as usize)?.as_ref()
    }

    /// Illuminates all cells in the direction of the given side.
    ///
    /// The light beam ends at a blocked cell and the border of the game board.
    fn illuminate(&self, position: (u32, u32), side: Side) {
        match self.get_cell(position) {
            None | Some(cell::Cell::Blocked(_)) => (),
            Some(free_cell) => {
                free_cell.add_illuminated_side(side.opposite()); // Could do side and not opposite by convention, should work too.
                self.illuminate(Self::get_neighbour_position(position, side), side);
            }
        }
    }

    /// Shadows all cells in the direction of the given side.
    ///
    /// The evaluation ends at a blocked cell and the border of the game board.
    fn shadow(&self, position: (u32, u32), side: Side) {
        match self.get_cell(position) {
            None | Some(cell::Cell::Blocked(_)) => (),
            Some(free_cell) => {
                free_cell.remove_illuminated_side(side.opposite()); // Could do side and not opposite by convention, should work too.
                self.shadow(Self::get_neighbour_position(position, side), side);
            }
        }
    }
    
    /// Updates the game board depending on the state of the cell at the given position.
    ///
    /// If the given cell is a light source, this updates all cells that go out in straight
    /// lines from that cell to be illuminated.
    ///
    /// If the given cell is not a light source this makes sure no cell on the board is left
    /// illuminated by this cell.
    fn update(&self, position: (u32, u32)) {
        match self.get_cell(position) {
            None | Some(cell::Cell::Blocked(_)) => (),
            Some(free_cell) => {
                for side in Side::all() {
                    if free_cell.lamp {
                        self.illuminate(Self::get_neighbour_position(position, side), side);
                    } else {
                        self.shadow(Self::get_neighbour_position(position, side), side);
                    }
                }
            }
        }
    }
}
