# Advent of Code, Template

This project contains a template for solutions for the Advent of Code challenges.

## Usage

To set up, do a global find and replace for "2015" to update the year number. You may also want to 
update this readme. For years from 2025 on there will only be 15 puzzles, so the other files can be deleted.

To run a solution for a puzzle, run: 

`cargo run -- <DAY> <PART>`

Where `<DAY>` is the day number (1-25) and part is 1 or 2.

You can also run `cargo test day<N>` to run the unit tests for a given day. For days 1-9 you'll
need a leading zero (E.G. `cargo test day03`), so that `day1` doesn't match days 10-19, etc.

These usually cover the sample cases for the puzzles, occasionally among other things.

## Organization

Each day's puzzle has it's solution code in `/src/solutions/dayN.rs`. For more complex puzzles
these may be just entry points/indexes for the module.  Common code used across the solutions
is put in `/src/common/*`.