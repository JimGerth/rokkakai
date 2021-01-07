//! A collection of types and methods for cells of a rokkakari game.
//!
//!

pub enum Cell {
    Blocked(blocked::BlockedCell),
    Free(free::FreeCell),
}

pub mod blocked {

    use std::convert::TryFrom;

    pub enum BlockedCell {
        Unconstrained,
        Constrained(i32),
    }

    impl Default for BlockedCell {
        fn default() -> Self {
            Self::Unconstrained
        }
    }

    impl From<i32> for BlockedCell {
        fn from(constraint: i32) -> Self {
            Self::Constrained(constraint)
        }
    }

    impl From<Option<i32>> for BlockedCell {
        fn from(option: Option<i32>) -> Self {
            match option {
                Some(constraint) => Self::Constrained(constraint),
                None => Self::Unconstrained,
            }
        }
    }
}

pub mod free {

    pub struct FreeCell {
        marking: Option<Marking>,
        illuminated_from: Illumination,
        // neighbours // theoretically only these need to be linked up their neighbours, as light doesnt travel through blocked cells.
    }

    impl FreeCell {
        pub fn new() -> Self {
            Self {
                marking: None,
                illuminated_from: Illumination::none(),
            }
        }
    }

    struct Illumination {
        up: bool,
        up_right: bool,
        down_right: bool,
        down: bool,
        down_left: bool,
        up_left: bool,
    }

    impl Illumination {
        fn none() -> Illumination {
            Illumination {
                up: false,
                up_right: false,
                down_right: false,
                down: false,
                down_left: false,
                up_left: false,
            }
        }
    }

    pub enum Marking {
        Lamp,
        Cross,
    }
}
