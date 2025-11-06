mod common;
mod solutions;

use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Qhich quest to run the solution for
    #[arg(value_parser = clap::value_parser!(u32).range(1..=20))]
    quest: u32,

    /// Part of the quest to run
    #[arg(value_parser = clap::value_parser!(u32).range(1..=3))]
    part: u32,
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().init()?;

    let args = Args::parse();

    let solution = solutions::get_solution(args.quest)?;
    let input_file_path = format!("inputs/quest{}_part{}.txt", args.quest, args.part);
    let input = std::fs::read_to_string(input_file_path)?;

    log::trace!("Running Quest {}, Part {}", args.quest, args.part);
    let output = match args.part {
        1 => solution.part1(&input)?,
        2 => solution.part2(&input)?,
        3 => solution.part3(&input)?,
        _ => unreachable!(),
    };

    log::info!("Output: {}", output);

    Ok(())
}
