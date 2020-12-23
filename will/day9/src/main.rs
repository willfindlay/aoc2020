use anyhow::Result;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let parsed = parse_input(&input)?;

    let target = part1(&parsed)?;
    part2(&parsed, target)?;

    Ok(())
}

/// Solve part 1
fn part1(input: &Vec<u64>) -> Result<u64> {
    // This is probably naive, but it's fast enough so who cares?
    // Start by iterating through elements 25 and onward
    for (i, &num) in input.iter().enumerate().skip(25) {
        let mut valid = false;
        // Consider each of the previous 25
        for j in i - 25..i {
            let x = input[j];
            // And pair it with subsequent candidates
            for k in j..i {
                // Check to see if the number is indeed valid
                let y = input[k];
                if x + y == num {
                    valid = true;
                }
            }
        }
        // If it's not valid, we have our answer
        if !valid {
            println!("{}", num);
            return Ok(num);
        }
    }

    unreachable!()
}

/// Solve part 2
fn part2(input: &Vec<u64>, target: u64) -> Result<()> {
    let mut lo: usize = 0;
    let mut hi: usize = 1;

    loop {
        let subseq = input[lo..hi].to_owned();
        let sum: u64 = subseq.iter().sum();

        // If we found our answer, get out
        if sum == target {
            let result = subseq.iter().min().unwrap() + subseq.iter().max().unwrap();
            println!("{}", result);
            break;
        }
        // If we are lower than the target, add another number
        else if sum < target {
            hi += 1;
        }
        // If we are higher than the target, remove the first number
        else {
            lo += 1;
        }

        // Handle the case where lo catches up with hi
        if lo == hi {
            hi += 1;
        }
    }

    Ok(())
}

/// Parse puzzle input into a vector of u64
fn parse_input(input: &String) -> Result<Vec<u64>> {
    let mut vec = vec![];

    for line in input.lines() {
        vec.push(line.parse()?);
    }

    Ok(vec)
}
