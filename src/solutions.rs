mod day01; mod day02; mod day03; mod day04; mod day05;
mod day06; mod day07; mod day08; mod day09; mod day10;
mod day11; mod day12; mod day13; mod day14; mod day15;
mod day16; mod day17; mod day18; mod day19; mod day20;
mod day21; mod day22; mod day23; mod day24; mod day25;
use anyhow::Result;

pub trait Solution {
    fn part1(&self, input: &str) -> Result<String>;
    fn part2(&self, input: &str) -> Result<String>;
}

pub fn get_solution(day: u32) -> Result<Box<dyn Solution>> {
    match day {
        0 => Err(anyhow::anyhow!("Day cannot be zero")),
        1 => Ok(Box::new(day01::Day1)),
        2 => Ok(Box::new(day02::Day2)),
        3 => Ok(Box::new(day03::Day3)),
        4 => Ok(Box::new(day04::Day4)),
        5 => Ok(Box::new(day05::Day5)),
        6 => Ok(Box::new(day06::Day6)),
        7 => Ok(Box::new(day07::Day7)),
        8 => Ok(Box::new(day08::Day8)),
        9 => Ok(Box::new(day09::Day9)),
        10 => Ok(Box::new(day10::Day10)),
        11 => Ok(Box::new(day11::Day11)),
        12 => Ok(Box::new(day12::Day12)),
        13 => Ok(Box::new(day13::Day13)),
        14 => Ok(Box::new(day14::Day14)),
        15 => Ok(Box::new(day15::Day15)),
        16 => Ok(Box::new(day16::Day16)),
        17 => Ok(Box::new(day17::Day17)),
        18 => Ok(Box::new(day18::Day18)),
        19 => Ok(Box::new(day19::Day19)),
        20 => Ok(Box::new(day20::Day20)),
        21 => Ok(Box::new(day21::Day21)),
        22 => Ok(Box::new(day22::Day22)),
        23 => Ok(Box::new(day23::Day23)),
        24 => Ok(Box::new(day24::Day24)),
        25 => Ok(Box::new(day25::Day25)),
        _ => Err(anyhow::anyhow!("Day out of range {}", day)),
    }
}