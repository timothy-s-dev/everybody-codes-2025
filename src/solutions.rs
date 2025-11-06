mod quest01; mod quest02; mod quest03; mod quest04; mod quest05;
mod quest06; mod quest07; mod quest08; mod quest09; mod quest10;
mod quest11; mod quest12; mod quest13; mod quest14; mod quest15;
mod quest16; mod quest17; mod quest18; mod quest19; mod quest20;
use anyhow::Result;

pub trait Solution {
    fn part1(&self, input: &str) -> Result<String>;
    fn part2(&self, input: &str) -> Result<String>;
    fn part3(&self, input: &str) -> Result<String>;
}

pub fn get_solution(quest: u32) -> Result<Box<dyn Solution>> {
    match quest {
        0 => Err(anyhow::anyhow!("Quest cannot be zero")),
        1 => Ok(Box::new(quest01::Quest1)),
        2 => Ok(Box::new(quest02::Quest2)),
        3 => Ok(Box::new(quest03::Quest3)),
        4 => Ok(Box::new(quest04::Quest4)),
        5 => Ok(Box::new(quest05::Quest5)),
        6 => Ok(Box::new(quest06::Quest6)),
        7 => Ok(Box::new(quest07::Quest7)),
        8 => Ok(Box::new(quest08::Quest8)),
        9 => Ok(Box::new(quest09::Quest9)),
        10 => Ok(Box::new(quest10::Quest10)),
        11 => Ok(Box::new(quest11::Quest11)),
        12 => Ok(Box::new(quest12::Quest12)),
        13 => Ok(Box::new(quest13::Quest13)),
        14 => Ok(Box::new(quest14::Quest14)),
        15 => Ok(Box::new(quest15::Quest15)),
        16 => Ok(Box::new(quest16::Quest16)),
        17 => Ok(Box::new(quest17::Quest17)),
        18 => Ok(Box::new(quest18::Quest18)),
        19 => Ok(Box::new(quest19::Quest19)),
        20 => Ok(Box::new(quest20::Quest20)),
        _ => Err(anyhow::anyhow!("Quest out of range {}", quest)),
    }
}