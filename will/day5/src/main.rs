use anyhow::Result;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let parsed = parse_input(&input)?;

    part1(parsed.clone())?;
    part2(parsed.clone())?;

    Ok(())
}

/// Solve part 1 of day 5
fn part1(input: Vec<&str>) -> Result<()> {
    let mut high_id = 0;

    for s in input {
        let (row, col) = find_seat(s)?;
        let id = row * 8 + col;
        if id > high_id {
            high_id = id
        }
    }

    println!("{}", high_id);

    Ok(())
}

/// Solve part 2 of day 5
fn part2(input: Vec<&str>) -> Result<()> {
    let mut ids = vec![];

    for s in input {
        let (row, col) = find_seat(s)?;
        let id = row * 8 + col;
        ids.push(id);
    }

    ids.sort();
    let mut missing = 0;
    for i in 1..ids.len() {
        if ids[i] != ids[i - 1] + 1 {
            missing = ids[i - 1] + 1;
        }
    }

    println!("{}", missing);

    Ok(())
}

/// Parse puzzle input into a vector of str
fn parse_input(input: &String) -> Result<Vec<&str>> {
    let mut result = vec![];

    for line in input.lines() {
        result.push(line);
    }

    Ok(result)
}

/// Parse a str to find the (row, col) seat
fn find_seat(input: &str) -> Result<(i32, i32)> {
    let mut lrow = 0;
    let mut hrow = 127;
    let mut lcol = 0;
    let mut hcol = 7;

    for c in input.chars() {
        match c {
            'F' => hrow = (lrow + hrow) / 2 as i32,
            'B' => lrow = (lrow + hrow) / 2 as i32 + 1,
            'L' => hcol = (lcol + hcol) / 2 as i32,
            'R' => lcol = (lcol + hcol) / 2 as i32 + 1,
            _ => panic!("Character {} not recognized!", c),
        }
    }

    std::assert!(lrow == hrow, "Low row and high row are not equal!");
    std::assert!(lcol == hcol, "Low col and high col are not equal!");

    Ok((lrow, lcol))
}
