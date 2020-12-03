//! My solutions to Day 2 of Advent of Code 2020.
//! The challenge is available here: https://adventofcode.com/2020/day/2

#![feature(format_args_capture)]

use std::io::{stdin, Read};

fn trees(terrain: Vec<&str>, right: usize, down: usize) -> Result<u32, Box<dyn std::error::Error>> {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < terrain.len() - 1 {
        x += right;
        y += down;

        let string = terrain[y];

        if string.chars().nth(x % string.len()).unwrap() == '#' {
            trees += 1;
        }
    }

    Ok(trees)
}

fn part1(input: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut terrain = vec![];
    for line in input.lines() {
        terrain.push(line);
    }

    if let Ok(trees) = trees(terrain, 3, 1) {
        println!("{}", trees);
    }

    Ok(())
}

fn part2(input: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut terrain = vec![];
    for line in input.lines() {
        terrain.push(line);
    }

    let mut ans = 1;

    for t in &[[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]] {
        let right = t[0];
        let down = t[1];

        if let Ok(trees) = trees(terrain.clone(), right, down) {
            ans *= trees;
        }
    }

    println!("{}", ans);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}
