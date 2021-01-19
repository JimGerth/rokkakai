//! # Rokkakari Game Logic Library
//!
//! This crate exposes Rokkakari game functionality inlcuding interacting with and saving the state of
//! the game board.
//!
//! # About
//!
//! Rokkakari is a combination of the japanese words [**rokkakkei** _(hexagon)_][rokkakei_translation]
//! and [**akari** _(light)_][akari_translation].
//!
//! ## Akari
//!
//! The latter is also the name for a pen and paper [logic puzzle][akari_puzzle] introduced by the
//! japanese publisher [Nikoli][nikoli_publisher].
//! It consists of a square grid that has to be entirely _lit up_ by placing _lamps_ into it, which
//! will light up the corresponding columns and rows of cells.
//! Some cells of the grid may be _walls_ that will block off the light and furthermore can constrain
//! the number of lamps allowed to be placed directly adjacent to them (indicated by a number written
//! in that cell).
//! Also more importantly two lamps cannot light up _each other_.
//!
//! Refer to this [video][akari_video] for a more detailed explanation of the puzzle or try out this
//! [interactive demo][akari_demo].
//!
//! ## Hexagonal Akari
//!
//! As the translation of [rokkakkei][rokkakei_translation] might suggest, Rokkakari takes all the
//! mechanics of regular Akari and applies them to an hexagonal grid.
//!
//! See this [example puzzle][rokkakari_puzzle] or try out this [interactive demo][rokkakari_demo].
//!
//! # Source
//!
//! This crate is maintained [on GitHub][github_repo], feel free to contribute :)
//!
//! [rokkakei_translation]: https://en.wiktionary.org/wiki/六角形#Japanese
//! [akari_translation]: https://en.wiktionary.org/wiki/明かり#Japanese
//! [akari_puzzle]: https://en.wikipedia.org/wiki/Light_Up_(puzzle)
//! [nikoli_publisher]: https://en.wikipedia.org/wiki/Nikoli_(publisher)
//! [akari_video]: https://www.youtube.com/watch?v=huMgnsAIDjw
//! [akari_demo]: https://www.janko.at/Raetsel/Akari/index.htm
//! [rokkakari_puzzle]: https://www.janko.at/Raetsel/Varianten/020.a.htm
//! [rokkakari_demo]: https://jimgerth.herokuapp.com/akari
//! [github_repo]: https://github.com/JimGerth/rokkakari

pub mod cell;
pub mod board;
