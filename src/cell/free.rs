//! Internal module defining the state of a free cell.

use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize, PartialEq, Serialize)]
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

/// The possible markings for *free* cells.
///
/// See [the module level documentation for more][cell_module_at_free_cells].
///
/// [free_cell]: crate::cell::Cell::Free
/// [cell_module_at_free_cells]: crate::cell#free-cells
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Marking {
    /// A *lamp* placed in a free cell lights up other free cells in every
    /// direction.
    Lamp,

    /// A *cross* placed in a free cell is used to signify that there
    /// cannot be a lamp in that cell.
    Cross,
}
