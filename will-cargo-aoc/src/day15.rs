use anyhow::Result;
use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<(Vec<u32>)> {
    let nums = input
        .split(",")
        .map(|s| s.parse().expect("Unable to parse"))
        .collect();
    Ok(nums)
}

#[aoc(day15, part1)]
pub fn part1(input: &Vec<u32>) -> Result<u32> {
    play_game(input, 2020)
}

#[aoc(day15, part2)]
pub fn part2(input: &Vec<u32>) -> Result<u32> {
    play_game(input, 30000000)
}

fn play_game(input: &Vec<u32>, turns: u32) -> Result<u32> {
    // Map a number to the turn it was most recently spoken and whether it was

    // the first time
    let mut numbers: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut num: u32 = 0;

    // Process starting numbers
    for (turn, &starting_num) in input.iter().enumerate() {
        let turn: u32 = turn as u32 + 1;
        numbers.insert(starting_num, (turn, 0));
        num = starting_num;
    }

    for turn in input.len() as u32 + 1..=turns {
        // Look up data for the last spoken number
        let (curr, prev) = numbers.get(&num).unwrap();

        // Never been spoken before
        if *prev == 0 {
            num = 0;
        }
        // Spoken before, take difference between last two times
        else {
            num = *curr - *prev;
        }

        // Update the new spoken number
        let new_val = if numbers.contains_key(&num) {
            // If it has been spoken before, use old curr as new prev
            (turn, numbers.get(&num).unwrap().0)
        } else {
            // If it hasn't been spoken before, 0 as prev for now
            (turn, 0)
        };
        numbers.insert(num, new_val);
    }

    Ok(num)
}
