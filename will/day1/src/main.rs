use std::io::{stdin, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

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
    return Ok(());
}

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
    return Ok(());
}
