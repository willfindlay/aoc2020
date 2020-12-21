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
fn part1(parsed: &Vec<Vec<&str>>) -> Result<()> {
    let mut map = HashMap::new();
    let mut sum = 0;

    // Keep track of which questions are answered using a hashmap
    for group in parsed {
        for s in group {
            // We only care about whether _somebody_ answered a question
            for c in s.chars() {
                map.insert(c, 1);
            }
        }

        // Sum them all up
        sum += map.values().sum::<i32>();
        map.clear();
    }

    println!("{}", sum);

    Ok(())
}

/// Solve part 2
fn part2(parsed: &Vec<Vec<&str>>) -> Result<()> {
    let mut map = HashMap::new();
    let mut sum = 0;

    // Keep track of which questions are answered using a hashmap
    for group in parsed {
        let group_count = group.len() as i32;

        for s in group {
            // We care that _everybody_ answered the question
            for c in s.chars() {
                let entry = map.entry(c).or_insert(0);
                *entry += 1;
            }
        }

        // Find the entries that match group_count and count those up
        sum += map.values().filter(|v| **v == group_count).count();
        map.clear();
    }

    println!("{}", sum);

    Ok(())
}

/// Parse the input into a vector of "groups" (a group is a vector of strings)
fn parse_input(input: &String) -> Result<Vec<Vec<&str>>> {
    let mut parsed = vec![];

    let mut group = vec![];
    for line in input.lines() {
        if line.is_empty() {
            parsed.push(group.clone());
            group.clear();
            continue;
        }
        group.push(line);
    }
    parsed.push(group.clone());

    Ok(parsed)
}
