//! My solutions to Day 1 of Advent of Code 2020.
//! The challenge is available here: https://adventofcode.com/2020/day/1

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

/// Parse a list of integers, separate by new lines.
/// Find two entries that sum to 2020 and return their product.
///
/// Runs in O(n^2) time.
fn part1(input: &str) -> Result<()> {
    let mut numbers = vec![];

    // Parse numbers from lines
    for line in input.lines() {
        let number: u32 = line.parse()?;
        numbers.push(number);
    }

    // Iterate over permutations
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let first = numbers[i];
            let second = numbers[j];

            // Do we sum up to 2020?
            if first + second == 2020 {
                println!("{}", first * second);
                return Ok(());
            }
        }
    }

    panic!("Bad input")
}

/// Parse a list of integers, separate by new lines.
/// Find three entries that sum to 2020 and return their product.
///
/// A clever optimization here involves sorting the list before searching.
/// We can sort in O(nlogn) time and search in O(n^2), rather than O(n^3).
///
/// Runs in O(n^2) time.
fn part2(input: &str) -> Result<()> {
    let mut numbers = vec![];

    // Parse numbers from lines
    for line in input.lines() {
        let number: u32 = line.parse()?;
        numbers.push(number);
    }

    // Sort the array
    numbers.sort();

    // Iterate over permutations
    for i in 0..numbers.len() {
        let mut start = i + 1;
        let mut end = numbers.len() - 1;

        while start < end {
            let first = numbers[i];
            let second = numbers[start];
            let third = numbers[end];

            let sum = first + second + third;

            if sum == 2020 {
                println!("{}", first * second * third);
                return Ok(());
            } else if sum < 2020 {
                start += 1;
            } else {
                end -= 1;
            }
        }
    }

    panic!("Bad input")
}
