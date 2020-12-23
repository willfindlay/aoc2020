use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let parsed = parse_input(input)?;

    part1(&parsed)?;
    part2(&parsed)?;

    Ok(())
}

/// Run the simulation and print the value of the accumulator
fn part1(ops: &Vec<(String, i32)>) -> Result<()> {
    let (acc, _) = run_program(ops)?;

    println!("{}", acc);

    Ok(())
}

/// Run the simulation, flipping nop and jmp instructions one by one until it
/// terminates
fn part2(ops: &Vec<(String, i32)>) -> Result<()> {
    for (i, (opcode, _)) in ops.iter().enumerate() {
        let mut new_ops = ops.clone();
        match &opcode[..] {
            "nop" => new_ops[i].0 = "jmp".to_string(),
            "jmp" => new_ops[i].0 = "nop".to_string(),
            _ => continue,
        }

        let (acc, terminated) = run_program(&new_ops)?;

        if terminated {
            println!("{}", acc);
            return Ok(());
        };
    }

    unreachable!()
}

/// Run the simulation, and return a tuple of accumulator value and whether the
/// program terminated
fn run_program(ops: &Vec<(String, i32)>) -> Result<(i32, bool)> {
    let mut terminated = true;
    let mut acc = 0;
    let mut ip = 0;
    let mut visited = HashSet::new();

    while ip < ops.len() {
        if !visited.insert(ip) {
            terminated = false;
            break;
        }
        let (opcode, number) = ops[ip].clone();
        match &opcode[..] {
            "nop" => {}
            "acc" => acc += number,
            "jmp" => {
                if number != 0 {
                    let pos = number > 0;
                    if pos {
                        ip += number.abs() as usize;
                    } else {
                        ip -= number.abs() as usize;
                    }
                    continue;
                }
            }
            _ => unreachable!(),
        }
        ip += 1;
    }

    Ok((acc, terminated))
}

/// Parse puzzle input lines into a vector of opcode,arg tuples
fn parse_input(input: String) -> Result<Vec<(String, i32)>> {
    let mut result = vec![];

    let re = Regex::new(r"([a-z]+) ([+-]\d+)")?;
    for line in input.lines() {
        match re.captures(line) {
            Some(cap) => {
                let opcode = cap.get(1).unwrap().as_str();
                let number: i32 = cap.get(2).unwrap().as_str().parse()?;
                result.push((opcode.into(), number));
            }
            None => panic!("Couldn't parse line {}", line),
        }
    }

    Ok(result)
}
