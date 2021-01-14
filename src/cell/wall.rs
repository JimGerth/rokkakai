//! Internal module defining the state of a wall state.

/// Represents the state of a *wall* game cell.
///
/// Used as the associated data in the [`Cell::Wall`][wall_cell] variant.
///
/// See [the module level documentation for more][cell_module_at_wall_cells].
///
/// [wall_cell]: crate::cell::Cell::Wall
/// [cell_module_at_wall_cells]: crate::cell#wall-cells
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    /// The wall cell does not empose a constraint on its neighbours.
    Unconstrained,

    /// The wall cell emposes a [`Constraint`][constraint] on its neighbours.
    ///
    /// [constraint]: crate::cell::Constraint
    Constrained(Constraint),
}

/// The possible constraints a *wall* cell can empose on its neighbours.
///
/// *Yes,* this is an enum with a single variant, which *is* kind of unnecessary.
/// But there are a lot more possible constraints that could make the game really
/// interesting, *like a `Constraint::AtLeast(u8)` or a `Constraint::AtMost(u8)`
/// for example,* which could easily be added as additional variants in later
/// versions this way.
///
/// See [the module level documentation for more][cell_module_at_constraints].
///
/// [cell_module_at_constraints]: crate::cell#constraints
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Constraint {
    /// Neighbouring cells must contain an *exact* amount of lamps.
    ///
    /// As the cells are layed out on an hexagonal grid and lamps *cannot*
    /// light up each other the associated value has to be between
    /// `0` and `3` (inclusive).
    Equal(u8),
}

impl Default for State {
    /// Returns the state of an unconstrained wall cell.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rokkakari::cell::WallState;
    /// assert_eq!(WallState::default(), WallState::Unconstrained);
    /// ```
    fn default() -> Self {
        Self::Unconstrained
    }
}
