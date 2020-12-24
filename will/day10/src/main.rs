use anyhow::Result;
use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let parsed = parse_input(&input)?;

    part1(&parsed)?;
    part2(&parsed)?;

    Ok(())
}

/// Solve part 1
fn part1(input: &Vec<u64>) -> Result<()> {
    let mut ones = 0;
    let mut threes = 0;

    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    println!("{}", ones * threes);

    Ok(())
}

/// Solve part 2
fn part2(input: &Vec<u64>) -> Result<()> {
    let mut cache = HashMap::new();
    let num_ways = count_ways(0, &input, &mut cache)?;

    println!("{}", num_ways);

    Ok(())
}

/// Helper functions to recursively find the number of ways to traverse the
/// input array. Uses memoization to cache values
fn count_ways(i: usize, input: &Vec<u64>, cache: &mut HashMap<usize, u64>) -> Result<u64> {
    if i == input.len() - 1 {
        return Ok(1);
    }

    if let Some(&cached) = cache.get(&i) {
        return Ok(cached);
    }

    let mut total = 0;
    for j in i + 1..input.len() {
        if input[j] - input[i] <= 3 {
            total += count_ways(j, input, cache)?;
        }
    }

    cache.insert(i, total);
    return Ok(total);
}

/// Parse puzzle input into a sorted vector of u64
fn parse_input(input: &String) -> Result<Vec<u64>> {
    let mut vec = vec![];

    for line in input.lines() {
        vec.push(line.parse()?);
    }

    // Account for the wall
    vec.push(0);
    // Account for the device
    vec.push(vec.iter().max().unwrap() + 3);
    // Sort the vec
    vec.sort();

    Ok(vec)
}
