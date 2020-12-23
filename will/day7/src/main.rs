use anyhow::Result;
use regex::Regex;
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
fn part1(parsed: &HashMap<String, Vec<(String, i32)>>) -> Result<()> {
    let mut count = 0;

    for bag in parsed.keys() {
        if bag == "shiny gold" {
            continue;
        }
        if candidate_part1(parsed, bag.to_string()) {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}

/// Solve part 2
fn part2(parsed: &HashMap<String, Vec<(String, i32)>>) -> Result<()> {
    println!("{}", count(parsed, "shiny gold".into()));

    Ok(())
}

/// Is bag a candidate in bags?
fn candidate_part1(bags: &HashMap<String, Vec<(String, i32)>>, bag: String) -> bool {
    if bag == "shiny gold" {
        return true;
    }

    for contains in bags.get(&bag).unwrap_or(&vec![]) {
        if candidate_part1(bags, (*contains.0).to_string()) {
            return true;
        }
    }

    false
}

fn count(bags: &HashMap<String, Vec<(String, i32)>>, bag: String) -> i32 {
    let mut total = 0;

    for contains in bags.get(&bag).unwrap_or(&vec![]) {
        total += contains.1 + contains.1 * count(bags, (*contains.0).to_string());
    }

    total
}

/// Parse the input into graph
fn parse_input(input: &String) -> Result<HashMap<String, Vec<(String, i32)>>> {
    let bag_re = Regex::new(r"([a-z ]+) bags? contains?")?;
    let contains_re = Regex::new(r"(\d+)\s([a-z ]+)\sbags?|(no other bags)")?;
    let mut graph = HashMap::new();

    for line in input.lines() {
        let bag = &bag_re.captures(line).unwrap()[1];
        for c in contains_re.captures_iter(line) {
            let vec = graph.entry(bag.to_string()).or_insert(vec![]);
            match c.get(3) {
                Some(_) => {}
                None => vec.push((c[2].to_string(), c[1].parse::<i32>()?)),
            };
        }
    }

    Ok(graph)
}
