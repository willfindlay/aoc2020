//! My solutions to Day 2 of Advent of Code 2020.
//! The challenge is available here: https://adventofcode.com/2020/day/2

#![feature(format_args_capture)]

use regex::Regex;
use std::error::Error;
use std::io::{stdin, Read};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut valid = 0;

    let re = Regex::new(r"(\d*)-(\d*) ([a-z]): ([a-z]*)")?;
    for line in input.lines() {
        let cap = re.captures(line).unwrap();

        // Parse out values
        let lower = cap.get(1).unwrap().as_str().parse::<u32>()?;
        let upper = cap.get(2).unwrap().as_str().parse::<u32>()?;
        let letter = cap.get(3).unwrap().as_str();
        let password = cap.get(4).unwrap().as_str();

        // Count the letters
        let mut count = 0;
        for ell in password.chars() {
            if ell == letter.chars().nth(0).unwrap() {
                count += 1;
            }
        }

        // Check if password is valid
        if count >= lower && count <= upper {
            valid += 1;
        }
    }

    println!("Valid: {valid}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut valid = 0;

    let re = Regex::new(r"(\d*)-(\d*) ([a-z]): ([a-z]*)")?;
    for line in input.lines() {
        let cap = re.captures(line).unwrap();

        // Parse out values
        let first: usize = cap.get(1).unwrap().as_str().parse::<usize>()? - 1;
        let second: usize = cap.get(2).unwrap().as_str().parse::<usize>()? - 1;
        let letter = cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password: Vec<char> = cap.get(4).unwrap().as_str().chars().collect();

        // Count letters at given positions
        let mut count = 0;
        if password[first] == letter {
            count += 1;
        }
        if password[second] == letter {
            count += 1;
        }

        // Check if valid
        if count == 1 {
            valid += 1;
        }
    }

    println!("Valid: {valid}");

    Ok(())
}
