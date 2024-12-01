use clap::Parser;
use aoc;
use aoc::Run;


fn main() {
    let cli = aoc::Cli::parse();

    aoc::to_runner(&cli.command).run();
}

