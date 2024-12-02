use std::error::Error;

use clap::Parser;
use aoc;
use aoc::Run;


fn main() -> Result<(), Box<dyn Error>> {
    let cli = aoc::Cli::parse();
    let input = &cli.input.ok_or("No Input provided!")?; 
    let _ = aoc::to_runner(&cli.command).run(input);
    Ok(())
}

