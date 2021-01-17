//! Internal module defining sides of a cell.

/// The sides of a game cell.
///
/// This [crate] is *rotation agnostic*, which is why the sides are not named
/// `Left` and `Right` or `East` and `West`.
/// The reason for that decision is that the hexagonal cells are not
/// symmetric at *90 degree* rotations, but the emerging game board patterns
/// are actually quite different and both desirable in different situations.
///
/// [crate]: crate
///
/// For example displaying the cells with a corner on the top is more suitable
/// for narrow devices like smart phones, while displaying cells with a flat
/// side up is nicer on wide screens like on laptops or tablets.
///
/// ```text
///                   *
///              *         *                                      *    *    *    *
///         *                   *
///     *                           *                          *                    *
///
///     *                           *                       *                          *
///
///     *                           *         vs.         *                              *
///
///     *                           *                       *                          *
///
///     *                           *                          *                    *
///         *                   *
///              *         *                                      *    *    *    *
///                   *
/// ```
///
/// At what rotation cells end up being displayed exactly is *only* up to any
/// game that uses this crate but is not of concern for this crate internally.
/// The only important thing is that the sides get labled *distinctly* and
/// *consistently*, thus `A`, `B`, `C`, `D`, `E` and `F`:
///
/// ```text
///                                                                      A
///          F        *        A
///              *         *                                      *    *    *    *
///         *                   *
///     *                           *                    F     *                    *     B
///
///     *                           *                       *                          *
///
/// E   *                           *   B       vs.       *                              *
///
///     *                           *                       *                          *
///
///     *                           *                    E     *                    *     C
///         *                   *
///              *         *                                      *    *    *    *
///          D        *        C
///                                                                      D
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    #[allow(clippy::missing_docs_in_private_items)]
    A,
    #[allow(clippy::missing_docs_in_private_items)]
    B,
    #[allow(clippy::missing_docs_in_private_items)]
    C,
    #[allow(clippy::missing_docs_in_private_items)]
    D,
    #[allow(clippy::missing_docs_in_private_items)]
    E,
    #[allow(clippy::missing_docs_in_private_items)]
    F,
}
