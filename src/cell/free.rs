//! Internal module defining the state of a free cell.

use super::side::Side;

/// Represents the state of a *free* game cell.
///
/// Used as the associated data in the [`Cell::Free`][free_cell] variant.
///
/// A free cell *can* contain a [`Marking`][marking], but the default is an
/// empty cell containing no marking.
/// Furthermore a free cell knows, wether it is lit up by another free cell
/// containing a lamp, and if so, from which [`Side`][side] it is lit up.
/// (Or rather it can *represent* that. It is not responsible
/// for checking or keeping track of that though).
///
/// See [the module level documentation for more][cell_module_at_free_cells].
///
/// [free_cell]: crate::cell::Cell::Free
/// [marking]: crate::cell::Marking
/// [side]: crate::cell::Side
/// [cell_module_at_free_cells]: crate::cell#free-cells
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    /// An optional [`Marking`] on the free cell.
    ///
    /// [marking]: crate::cell::Marking
    marking: Option<Marking>,

    /// Represent wether this cell is lit from a certain side.
    ///
    /// Each field represents wether there is a lamp lighting this
    /// cell from the respective side.
    /// See the [`Side`][side] type for more.
    ///
    /// [side]: crate::cell::Side
    lit_from_a: bool,
    #[allow(clippy::missing_docs_in_private_items)]
    lit_from_b: bool,
    #[allow(clippy::missing_docs_in_private_items)]
    lit_from_c: bool,
    #[allow(clippy::missing_docs_in_private_items)]
    lit_from_d: bool,
    #[allow(clippy::missing_docs_in_private_items)]
    lit_from_e: bool,
    #[allow(clippy::missing_docs_in_private_items)]
    lit_from_f: bool,
}

impl State {
    /// Returns wether this cell is lit up from the given [`Side`][side].
    ///
    /// [side]: crate::cell::Side
    pub fn is_lit_from(&self, side: Side) -> bool {
        match side {
            Side::A => self.lit_from_a,
            Side::B => self.lit_from_b,
            Side::C => self.lit_from_c,
            Side::D => self.lit_from_d,
            Side::E => self.lit_from_e,
            Side::F => self.lit_from_f,
        }
    }

    /// Returns wether this cell has a [`Lamp`][lamp] marking in it or not.
    ///
    /// [lamp]: crate::cell::Marking::Lamp
    pub fn has_lamp(&self) -> bool {
        matches!(self.marking, Some(Marking::Lamp))
    }
}

impl Default for State {
    /// Returns the state of an empty unilluminated free cell.
    fn default() -> Self {
        Self {
            marking: None,
            lit_from_a: false,
            lit_from_b: false,
            lit_from_c: false,
            lit_from_d: false,
            lit_from_e: false,
            lit_from_f: false,
        }
    }
}

/// The possible markings for *free* cells.
///
/// See [the module level documentation for more][cell_module_at_free_cells].
///
/// [free_cell]: crate::cell::Cell::Free
/// [cell_module_at_free_cells]: crate::cell#free-cells
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Marking {
    /// A *lamp* placed in a free cell lights up other free cells in every
    /// direction.
    Lamp,

    /// A *cross* placed in a free cell is used to signify that there
    /// cannot be a lamp in that cell.
    Cross,
}
