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
    for range in input.split(',') {
        if let Some((left_boundry, right_boundry)) = range.split_once('-') {
            let left_boundry: u32 = left_boundry.parse()?;
            let right_boundry: u32 = right_boundry.parse()?;


        }
        else {
            // TODO: Wrong format
        }
    }
    
    return Ok(0);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn solves_example() {
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(solve_puzzle(example).unwrap(), 1227775554);
    }
}
