mod common;
mod solutions;

use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of the challenge to run
    #[arg(value_parser = clap::value_parser!(u32).range(1..=25))]
    day: u32,

    /// Part of the challenge to run
    #[arg(value_parser = clap::value_parser!(u32).range(1..=2))]
    part: u32,
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().init()?;

    let args = Args::parse();

    let solution = solutions::get_solution(args.day)?;
    let input_file_path = format!("inputs/day{}_part{}.txt", args.day, args.part);
    let input = std::fs::read_to_string(input_file_path)?;

    log::trace!("Running Day {}, Part {}", args.day, args.part);
    let output = match args.part {
        1 => solution.part1(&input)?,
        2 => solution.part2(&input)?,
        _ => unreachable!(),
    };

    log::info!("Output: {}", output);

    Ok(())
}
