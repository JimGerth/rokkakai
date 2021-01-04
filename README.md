# Rokkakari

A combination of the japanese words [**rokkakkei** _(hexagon)_](https://en.wiktionary.org/wiki/六角形#Japanese) and [**akari** _(light)_](https://en.wiktionary.org/wiki/明かり#Japanese). 

## Akari

The latter is also the name for a pen and paper [logic puzzle](https://en.wikipedia.org/wiki/Light_Up_(puzzle)) introduced by the japanese publisher [Nikoli](https://en.wikipedia.org/wiki/Nikoli_(publisher)).
It consists of a square grid that has to be entirely _lit up_ by placing _lamps_  into it, which will light up their corresponding column and row. Some cells of the grid may be _walls_ that will block off the light and furthermore can constrain the number of lamps allowed to be placed directly adjacent to them (indicated by a number written in that cell).
Also more importantly two lamps cannot light up _each other_.

## Regular Tilings of the Plane

There are a lot of ways to subdivide a two dimensional surface, but only a few tilings are left when restricted to only one type of polygon:

- Triangular _(isometric graph paper, etc.)_
- Square _(chess, akari, standard graph paper, etc.)_
- Hexagonal _(honeycombs, graphene's atom structure, etc.)_

Due to their geometry pentagons and any polygon with _more_ than 6 vertecies cannot subdivide the plane by themselves.

## Hexagonal Akari

As I got interested in the akari puzzle and discovered a lot of interesting patterns arising from the interaction of the lamps on the board, I wondered how the akari rules would translate to a hexagonal grid and what kind of similar or different patterns would arise from that.
I looked for anything on hexagonal akari online, but the only thing I found was [this single example puzzle](https://www.janko.at/Raetsel/Varianten/020.a.htm).
It was not interactive however and so I tried to solve it the old fashioned way on pen and paper _(or rather by digitally drawing onto a screenshot...)_, but found it to be a lot trickier than the interactive regular akari I had been solving for a while then. At some point I got frustrated and built an interactive _(and to my knowledge the first)_ version of [hexagonal akari](https://jimgerth.herokuapp.com/akari) in the better part of a month running on JavaScript and a lot of clumsy SVG. But hey, it worked :D

## Now: Rokkakari

Because of my intrest in this puzzle and in an ongoing effort to learn [Rust](https://www.rust-lang.org) this is a project to write a library to support hexagonal akari logic. I also plan to write a frontend for this puzzle,  maybe some WebAssembly compiled from Rust to  interact with my previous SVG mess, a Flutter app or a text based program for starters. I'll see how this goes...

## Thanks

Thanks to Mark and Simon from [**Cracking the Cryptic**](https://www.youtube.com/c/CrackingTheCryptic) for [introducing](https://www.youtube.com/watch?v=Xg9radRLT8s) me to akari puzzles _(and knight sudoku)_.

Thanks to Angela and Otto from [**janko.at**](https://www.janko.at/Raetsel/index.htm) for hosting so many puzzles including [interactive akari](https://www.janko.at/Raetsel/Akari/index.htm), which really developed my interest in the puzzle.

Thanks to Palmer Mebane for uploading an [**hexagonal akari**](https://www.janko.at/Raetsel/Varianten/020.a.htm) to that same site, which helped me to wrap my mind around them and inspired me to keep working on the puzzle.
