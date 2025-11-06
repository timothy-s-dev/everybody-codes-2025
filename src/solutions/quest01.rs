use crate::solutions::Solution;
use anyhow::Result;

pub struct Quest1;

fn load_input(input: &str) -> Result<(Vec<&str>, Vec<i32>), anyhow::Error> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() != 3 {
        return Err(anyhow::anyhow!(
            "Expected 3 lines of input, got {}",
            lines.len()
        ));
    }
    let names = lines[0].split(',').collect();
    let instructions = lines[2].split(',')
        .map(|s| s
            .replace('R', "")
            .replace('L', "-")
            .parse::<i32>()
        )
        .collect::<Result<Vec<i32>, _>>()?;
    Ok((names, instructions))
}

impl Solution for Quest1 {
    fn part1(&self, input: &str) -> Result<String> {
        let (names, instructions) = load_input(input)?;
        let mut position = 0;
        for step in instructions {
            position = (position + step).min((names.len() - 1) as i32).max(0);
        }
        Ok(names[position as usize].to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (names, instructions) = load_input(input)?;
        let position = instructions.iter().sum::<i32>().rem_euclid(names.len() as i32);
        Ok(names[position as usize].to_string())
    }

    fn part3(&self, _input: &str) -> Result<String> {
        let (mut names, instructions) = load_input(_input)?;
        for step in instructions {
            let position = step.rem_euclid(names.len() as i32) as usize;
            names.swap(0, position);
        }
        Ok(names[0].to_string())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let output = Quest1.part1("Vyrdax,Drakzyph,Fyrryn,Elarzris\n\nR3,L2,R3,L1").unwrap();
        assert_eq!(output, "Fyrryn");
    }

    #[test]
    fn test_part2() {
        let output = Quest1.part2("Vyrdax,Drakzyph,Fyrryn,Elarzris\n\nR3,L2,R3,L1").unwrap();
        assert_eq!(output, "Elarzris");
    }

    #[test]
    fn test_part3() {
        let output = Quest1.part3("Vyrdax,Drakzyph,Fyrryn,Elarzris\n\nR3,L2,R3,L3").unwrap();
        assert_eq!(output, "Drakzyph");
    }
}