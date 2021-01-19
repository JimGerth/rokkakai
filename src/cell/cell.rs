//! Internal module defining the `Cell` type.

use super::free::State as FreeState;
use super::wall::State as WallState;

/// The [`Cell`][cell] type represents all possible states of a
/// single Rokkakari game cell.
///
/// See [the module level documentation for more][cell_module].
///
/// [cell]: crate::cell::Cell
/// [cell_module]: crate::cell#cells
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    /// A *free* game cell.
    /// All the free cell specific state is represented by [`FreeState`][free_state].
    ///
    /// [free_state]: crate::cell::FreeState
    Free(FreeState),

    /// A *wall* game cell.
    /// All the wall cell specific state is represented by [`WallState`][wall_state].
    ///
    /// [wall_state]: crate::cell::WallState
    Wall(WallState),
}

impl Cell {
    /// Make this cell a *free* cell (with [default][free_state_default] state).
    ///
    /// This *overrides* any previous state hold by this cell!
    ///
    /// [free_state_default]: crate::cell::FreeState::default
    ///
    /// # Examples
    ///
    /// ```
    /// # use rokkakari::cell::Cell;
    /// let mut cell = Cell::default();
    ///
    /// cell.make_free();
    ///
    /// assert!(matches!(cell, Cell::Free(_)));
    /// ```
    pub fn make_free(&mut self) {
        *self = Self::Free(FreeState::default());
    }

    /// Make this cell a *wall* cell (with [default][wall_state_default] state).
    ///
    /// This *overrides* any previous state hold by this cell!
    ///
    /// [wall_state_default]: crate::cell::WallState::default
    ///
    /// # Examples
    ///
    /// ```
    /// # use rokkakari::cell::Cell;
    /// let mut cell = Cell::default();
    ///
    /// cell.make_wall();
    ///
    /// assert!(matches!(cell, Cell::Wall(_)));
    /// ```
    pub fn make_wall(&mut self) {
        *self = Self::Wall(WallState::default());
    }
}

impl Default for Cell {
    /// Returns an empty free cell.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rokkakari::cell::{Cell, FreeState};
    /// assert_eq!(Cell::default(), Cell::Free(FreeState::default()));
    /// ```
    fn default() -> Self {
        Self::Free(FreeState::default())
    }
}
