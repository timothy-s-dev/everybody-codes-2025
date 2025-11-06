use std::{fmt, ops};

use crate::solutions::Solution;
use anyhow::Result;
use regex::Regex;

pub struct Quest2;

#[derive(Clone, Copy)]
struct ComplexNumber {
    x: i64,
    y: i64,
}

impl fmt::Debug for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl ops::Add<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: ComplexNumber) -> Self::Output {
        ComplexNumber { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: ComplexNumber) -> Self::Output {
        ComplexNumber { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Mul<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: ComplexNumber) -> Self::Output {
        ComplexNumber { x: self.x * rhs.x - self.y * rhs.y, y: self.x * rhs.y + self.y * rhs.x }
    }
}

impl ops::Div<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, rhs: ComplexNumber) -> Self::Output {
        ComplexNumber { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

struct Line {
    name: String,
    value: ComplexNumber,
}

impl Line {
    fn parse(input: &str) -> Result<Self, anyhow::Error> {
        let regex = Regex::new(r"([A-Z]+)=\[(\-?\d+),(\-?\d+)\]").unwrap();
        if let Some(captures) = regex.captures(input) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let x = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let y = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            return Ok(Line { name, value: ComplexNumber { x, y } });
        }
        Err(anyhow::anyhow!("Failed to parse line: {}", input))
    }
}


fn eval_point(position: ComplexNumber) -> bool {
    let mut value = ComplexNumber { x: 0, y: 0 };
    for _ in 0..100 {
        value = value * value;
        value = value / ComplexNumber { x: 100_000, y: 100_000 };
        value = value + position;
        if value.x > 1_000_000 || value.y > 1_000_000 || value.x < -1_000_000 || value.y < -1_000_000 {
            return false;
        }
    }
    true
}

impl Solution for Quest2 {
    fn part1(&self, input: &str) -> Result<String> {
        if let Some(data) = input.lines().map(Line::parse).next() {
            let input_value = data?.value;
            let mut value = ComplexNumber { x: 0, y: 0 };
            for _ in 0..3 {
                value = value * value;
                value = value / ComplexNumber { x: 10, y: 10 };
                value = value + input_value;
            }
            Ok(format!("{:?}", value))
        }
        else {
            Err(anyhow::anyhow!("No input data"))
        }
    }

    fn part2(&self, input: &str) -> Result<String> {
        if let Some(data) = input.lines().map(Line::parse).next() {
            let start_position = data?.value;
            let mut marked_positions = 0;
            for x_offset in (0..=1000).step_by(10) {
                for y_offset in (0..=1000).step_by(10) {
                    let position = ComplexNumber { x: start_position.x + x_offset, y: start_position.y + y_offset };
                    if eval_point(position) {
                        marked_positions += 1;
                    }
                }
            }
            Ok(marked_positions.to_string())
        }
        else {
            Err(anyhow::anyhow!("No input data"))
        }
    }

    fn part3(&self, input: &str) -> Result<String> {
        if let Some(data) = input.lines().map(Line::parse).next() {
            let start_position = data?.value;
            let mut marked_positions = 0;
            for x_offset in (0..=1000) {
                for y_offset in (0..=1000) {
                    let position = ComplexNumber { x: start_position.x + x_offset, y: start_position.y + y_offset };
                    if eval_point(position) {
                        marked_positions += 1;
                    }
                }
            }
            Ok(marked_positions.to_string())
        }
        else {
            Err(anyhow::anyhow!("No input data"))
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let output = Quest2.part1("A=[25,9]").unwrap();
        assert_eq!(output, "[357,862]");
    }

    #[test]
    fn test_part2() {
        let output = Quest2.part2("A=[35300,-64910]").unwrap();
        assert_eq!(output, "4076");
    }

    #[test]
    fn test_part3() {
        let output = Quest2.part3("A=[35300,-64910]").unwrap();
        assert_eq!(output.to_string(), "406954");
    }
}