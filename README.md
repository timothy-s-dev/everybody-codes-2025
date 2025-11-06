# Everybody Codes 2025

This project contains my solutions for the Everybody Codes 2025 challenges.

## Usage

To run a solution for a puzzle, run: 

`cargo run -- <DAY> <PART>`

Where `<DAY>` is the quest number (1-25) and part is 1, 2, or 3.

You can also run `cargo test quest<N>` to run the unit tests for a given quest. For quests 1-9 you'll
need a leading zero (E.G. `cargo test quest03`), so that `quest1` doesn't match quests 10-19, etc.

These usually cover the sample cases for the puzzles, occasionally among other things.

## Organization

Each quest's puzzle has it's solution code in `/src/solutions/questN.rs`. For more complex puzzles
these may be just entry points/indexes for the module.  Common code used across the solutions
is put in `/src/common/*`.