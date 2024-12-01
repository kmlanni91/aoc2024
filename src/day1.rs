use crate::runner::Run;


#[derive(Debug)]
pub struct Runner;

impl Run for Runner {
    fn run(&self) -> () {
        println!("Test day1");
    }
}
