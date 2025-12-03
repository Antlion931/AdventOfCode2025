// First exercise part two of Advent of Code 2025!
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
    // Dial at the end of each iteration of a loop should not be negative and smaller than 100
    let mut dial: i32 = 50;
    let mut count_zeroes: u32 = 0;

    for line in input.lines() {
        let number: u16 = line.get(1..).ok_or("Wrong file format")?.parse()?;
        let mut number = number as i32;

        assert!(number >= 0);

        // println!("move:{}, dial:{}, zeroes:{}", line, dial, count_zeroes);

        // Remove full rotations, they also point to zero at one point
        count_zeroes += number as u32 / 100;
        number %= 100;

        // If number is 0, it means that either:
        //  a) number was all of the time zero, no clicks, no furhter changes needed
        //  b) number was a multitude of 100, in which case removing full roation already acounted
        //  for click, which means no further changes needed
        if number == 0 {
            continue;
        }

        assert!(number > 0 && number < 100);

        match line.chars().nth(0) {
            Some('L') => {
                    
                if dial > number {
                    // Zero will not be reached because dial - number > 0
                    dial -= number;
                }
                else if dial == number {
                    dial = 0;
                    count_zeroes += 1;
                }
                else if dial < number {
                    // There is no way a zero could be reached if dial is at zero,
                    // at minimum 0 - 99 + 100 = 1, otherwise it will go throught it
                    if dial != 0 {
                        count_zeroes += 1;
                    }
                    
                    // Number is less than 100, so adding 100 should rotate it back to range
                    dial -= number;
                    dial += 100;
                }
                else {
                    unreachable!();
                }
            },

            Some('R') =>{
                dial += number;

                // If dial value is bigger than 100, it means it passed throught zero or is at 100,
                // At Maximum there will be 198 (99 + 99), so dial should be smaller than 100
                if dial >= 100 {
                    count_zeroes += 1;
                    dial -= 100;
                }
            },
            _ => return Err(Box::from("Wrong file format")),
        }

        assert!(dial >= 0 && dial < 100);
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

        assert_eq!(solve_puzzle(example).unwrap(), 6);
    }
}
