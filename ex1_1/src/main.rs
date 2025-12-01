// First exercise part one of Advent of Code 2025!
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
    let mut dial: i32 = 50;
    let mut count_zeroes: u32 = 0;

    for line in input.lines() {
        let mut number: i32 = line.get(1..).ok_or("Wrong file format")?.parse()?;

        number %= 100;

        match line.chars().nth(0) {
            Some('L') => dial = (dial - number).rem_euclid(100), //Here number can be negative, default
                                                                 //module would also return negative one
            Some('R') => dial = (dial + number) % 100,
            _ => return Err(Box::from("Wrong file format")),
        }


        if dial == 0 {
            count_zeroes += 1;
        }
    }

    return Ok(count_zeroes);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn solves_example() {
        let example =
"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(solve_puzzle(example).unwrap(), 3);
    }
}
