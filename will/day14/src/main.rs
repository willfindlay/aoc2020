use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut mask = "X".repeat(36);
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mask_re = Regex::new(r"mask = (.*)")?;
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)")?;

    for line in input.lines() {
        if let Some(cap) = mask_re.captures(line) {
            // Parse mask
            mask = cap[1].to_string();
        } else if let Some(cap) = mem_re.captures(line) {
            // Parse addr
            let addr = cap[1].parse::<u64>()?;
            // Parse val
            let mut val = cap[2].parse::<u64>()?;

            // Apply mask
            for (i, c) in mask.chars().rev().enumerate() {
                let i = i as u64;
                match c {
                    '0' => val &= !(1 << i),
                    '1' => val |= 1 << i,
                    _ => {}
                }
            }

            // Update memory location
            mem.insert(addr, val);
        } else {
            panic!("Unable to parse line {}", line);
        }
    }

    let sum: u64 = mem.values().sum();
    println!("{}", sum);

    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut mask = "X".repeat(36);
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mask_re = Regex::new(r"mask = (.*)")?;
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)")?;

    for line in input.lines() {
        if let Some(cap) = mask_re.captures(line) {
            // Parse mask
            mask = cap[1].to_string();
        } else if let Some(cap) = mem_re.captures(line) {
            // Parse val
            let val = cap[2].parse::<u64>()?;
            // Parse base addr
            let mut base_addr = cap[1].parse::<u64>()?;

            // Apply mask 1s
            for (i, c) in mask.chars().rev().enumerate() {
                let i = i as u64;
                match c {
                    '1' => base_addr |= 1 << i,
                    _ => {}
                }
            }

            // We potentially write may addrs
            let mut addrs = vec![base_addr];

            // Apply mask Xs
            for (i, c) in mask.chars().rev().enumerate() {
                let i = i as u64;
                match c {
                    'X' => {
                        for addr in addrs.clone() {
                            addrs.push(addr | (1 << i));
                            addrs.push(addr & !(1 << i));
                        }
                    }
                    _ => {}
                }
            }

            // Update memory location
            for addr in addrs {
                mem.insert(addr, val);
            }
        } else {
            panic!("Unable to parse line {}", line);
        }
    }

    let sum: u64 = mem.values().sum();
    println!("{}", sum);

    Ok(())
}
