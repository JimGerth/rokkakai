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

#[derive(IntoEnumIterator)]
enum Side {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Board {
    fn get_neighbour_position(position: (u32, u32), side: Side) -> (x, y) {
        let (x, y) = position;
        match side {
            Side::A => (if y % 2 == 0 { x } else { x + 1 }, y - 1),
            Side::B => (x + 1, y),
            Side::C => (if y % 2 == 0 { x } else { x + 1 }, y + 1),
            Side::D => (if y % 2 == 0 { x - 1 } else { x }, y + 1),
            Side::E => (x - 1, seld.y),
            Side::F => (if y % 2 == 0 { x - 1 } else { seld.x }, y - 1),
        }
    }

    fn get_neighbour(&self, position: (u32, u32), side: Side) -> Option<&Cell> {
        let (neighbour_x, neighbour_y) = Cell::get_neighbour_position((x, y), side);
        self.cells.get(y)?.get(x)
    }

    fn illuminate(&self, position: (u32, u32), side: Side) {
        match self.get_cell(position) {
            (None, Some(Cell::Blocked)) => (),
            Some(free_cell) => {
                free_cell.add_illuminated_side(side.opposite()); // Could do side and not opposite by convention, should work too.
                illuminate(Self::get_neighbour_position(position), side);
            }
        }
    }

    fn shadow(&self, position: (u32, u32), side: Side) {
        match self.get_cell(position) {
            (None, Some(Cell::Blocked)) => (),
            Some(free_cell) => {
                free_cell.remove_illuminated_side(side.opposite()); // Could do side and not opposite by convention, should work too.
                shadow(Self::get_neighbour_position(position), side);
            }
        }
    }

    fn update(&self, position: (u32, u32)) {
        match self.get_cell(position) {
            (None, Some(Cell::Blocked)) => (),
            Some(free_cell) => {
                for side in Side::into_enum_iter() {
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
