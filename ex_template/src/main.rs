// Template for Advent of Code 2025
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Box::from("Wrong number of arguments"));
    }

    let input_path = Path::new(&args[1]);

    let mut input_file = File::open(&input_path)?;

    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    println!("{}", solve_puzzle(&input)?);

    return Ok(());
}

fn solve_puzzle(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    return Ok(0);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn solves_example() {
        let example = "";

        assert_eq!(solve_puzzle(example).unwrap(), 0);
    }
}
