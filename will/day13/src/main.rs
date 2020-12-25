use anyhow::Result;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let parsed = parse_input(&input)?;

    part1(parsed.0, &parsed.1)?;
    part2(&parsed.1)?;

    Ok(())
}

fn part1(time: u32, str_ids: &Vec<&str>) -> Result<()> {
    let mut closest_id: u32 = 0;
    let mut closest_time = u32::MAX;
    let ids: Vec<u32> = str_ids
        .iter()
        .filter(|s| **s != "x")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    for id in ids {
        let trips: u32 = time / id;
        let before_trip: u32 = trips * id;
        // If we are right on the money
        if before_trip == time {
            closest_time = 0;
            closest_id = id;
            break;
        }
        // If we need to wait
        let after_trip: u32 = (trips + 1) * id;
        let diff = after_trip - time;
        if diff < closest_time {
            closest_time = diff;
            closest_id = id;
        }
    }

    println!("{}", closest_time * closest_id);

    Ok(())
}

fn part2(ids: &Vec<&str>) -> Result<()> {
    todo!("I have no idea how to do this...");
}

/// Parse puzzle @input into a tuple of (timestamp, ids) where ids is a list of
/// bus ids
fn parse_input(input: &String) -> Result<(u32, Vec<&str>)> {
    let time = input.lines().nth(0).unwrap_or("0").parse()?;
    let ids = input.lines().nth(1).unwrap_or("").split(",").collect();

    Ok((time, ids))
}
