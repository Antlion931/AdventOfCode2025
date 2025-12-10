// Second exercise part one of Advent of Code 2025
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

type uint = u64;

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

fn solve_puzzle(input: &str) -> Result<uint, Box<dyn std::error::Error>> {
    let mut result = 0;

    for range in input.split(',') {
        if let Some((left_boundry, right_boundry)) = range.split_once('-') {
            let mut left_boundry: uint = left_boundry.trim().parse()?;
            let right_boundry: uint = right_boundry.trim().parse()?;


            if left_boundry == 0 {
                left_boundry = 1;
            }

            let mut half_left = left_boundry / (10 as uint).pow(count_digits(left_boundry) / 2);

            if count_digits(left_boundry) % 2 == 1 {
                let silly_pattern =  silly_pattern_creator_odd(half_left);
                half_left = silly_pattern / (10 as uint).pow(count_digits(silly_pattern) / 2);
            } else if silly_pattern_creator_even(half_left) < left_boundry {
                half_left += 1;
            }


            while silly_pattern_creator_even(half_left) <= right_boundry {
                result += silly_pattern_creator_even(half_left);
                half_left += 1;
            }
        }
        else {
            // TODO: Wrong format
        }
    }
    
    return Ok(result);
}

#[inline(always)]
fn silly_pattern_creator_even(half: uint) -> uint {
    let amount_of_digits = count_digits(half);

    return half * (10 as uint).pow(amount_of_digits) + half;
}

#[inline(always)]
fn silly_pattern_creator_odd(half: uint) -> uint {
    let amount_of_digits = count_digits(half);

    return (10 as uint).pow(amount_of_digits*2 - 1) + (10 as uint).pow(amount_of_digits - 1);
}

#[inline(always)]
fn count_digits(digit: uint) -> u32 {
    assert!(digit != 0);

    return digit.ilog10() as u32 + 1;
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
