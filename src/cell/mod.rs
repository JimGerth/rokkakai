//! Represent Rokkakari game cell states.
//!
//! This module contains various types and functions to represent and manipulate the state
//! of game cells.
//!
//! # Cells
//!
//! A [`Cell`] is a single hexagon and the smallest logical unit in a Rokkakari game.
//! Cells are arranged in an hexagonal grid making up the game board.
//! Read more about [the Rokkakari game board][game_board] and
//! [the game in general][rokkakari].
//!
//! Generally speaking there are two kinds of cells: ***free*** cells and ***wall*** cells.
//!
//! [cell]: crate::cell::Cell
//! [game_board]: crate
//! [rokkakari]: crate
//!
//! ## Free Cells
//!
//! - ...are the ones typically painted *white* in regular [Akari].
//! - ...have to be lit up to solve the puzzle.
//! - ...***can*** contain ***either*** a [`Lamp`][lamp] that will light up other cells,
//! - ***or*** a [`Cross`][cross] to represent that they cannot contain a lamp.
//!
//! [akari]: https://en.wikipedia.org/wiki/Light_Up_(puzzle)
//! [lamp]: crate::cell::Marking::Lamp
//! [cross]: crate::cell::Marking::Cross
//!
//! ## Wall Cells
//!
//! - ...are the ones typically painted *black* in regular [Akari].
//! - ... do not need to be *- or rather cannot be -* lit up to solve the puzzle.
//! - ...contain *neither* lamps *nor* crosses.
//! - ...block off the light coming from a lamp in a free cell for other free cells.
//! - ...***can*** constrain the number of lamps *directly* adjacent to them.
//!
//! [akari]: https://en.wikipedia.org/wiki/Light_Up_(puzzle)
//!
//! ### Constraints
//!
//! As mentioned before wall cells can constrain the number of lamps in adjacent
//! cells.
//! Each cell has 6 direct neighbours *(unless it is located at the edge of the game board).*
//! If a wall cell imposes a [`Constraint`][constraint] on these neighbours, the sum of
//! all lamps collectively contained in those neighbouring cells has to equal the constraint
//! number.
//!
//! Because lamps cannot light up each other the number of lamps can only be constrained
//! to a number between `0` and `3` (inclusive).
//!
//! In regular [Akari] this is represented by the number being painted in the wall cell.
//!
//! [constraint]: crate::cell::Constraint
//! [akari]: https://en.wikipedia.org/wiki/Light_Up_(puzzle)

// Exported Types
pub use self::cell::Cell;
pub use self::free::{Marking, State as FreeState};
pub use self::side::Side;
pub use self::wall::{Constraint, State as WallState};

// Internal Modules
#[allow(clippy::module_inception)]
mod cell;
mod free;
mod side;
mod wall;
