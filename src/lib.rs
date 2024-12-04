use core::panic;
use std::fmt::Debug;
use clap::{Parser, Subcommand};
mod runner;
mod day1;
pub use runner::Run;
use runner::RunFile;
mod day2;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(global=true)]
    pub input: Option<String>,

    #[arg(short, long, global=true, default_value_t=false)]
    pub part2: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Day1,
    Day2,
}


pub fn to_runner(command: &Option<Commands>) -> Box<dyn RunFile> {
    match command {
        Some(Commands::Day1) => Box::new(day1::Runner),
        Some(Commands::Day2) => Box::new(day2::Runner),
        None => panic!("Not a valid command runner")
    }
}

