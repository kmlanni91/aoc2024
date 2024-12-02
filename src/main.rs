use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;
use aoc;
use aoc::Run;


fn main() -> Result<(), Box<dyn Error>> {
    let cli = aoc::Cli::parse();
    let input_path = &cli.input.ok_or("No Input provided!")?; 
    let f = File::open(input_path)?;
    let runner = aoc::to_runner(&cli.command);
    let result = match cli.part2 {
        false => runner.run(BufReader::new(f))?,
        true => runner.run2(BufReader::new(f))?
    };
    println!("{}", result);
    Ok(())
}

