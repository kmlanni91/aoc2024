use core::panic;
use std::fmt::Debug;
use clap::{Parser, Subcommand};
mod runner;
mod day1;
pub use runner::Run;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(global=true)]
    pub input: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Day1,
}


pub fn to_runner(command: &Option<Commands>) -> impl Run {
    match command {
        Some(Commands::Day1) => day1::Runner,
        None => panic!("Not a valid command runner")
    }
}

