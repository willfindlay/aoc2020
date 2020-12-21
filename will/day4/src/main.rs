use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, Read};

fn parse_input(input: &String) -> Result<Vec<HashMap<String, String>>> {
    let re = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid|cid):(\S+)")?;
    let mut map = HashMap::new();
    let mut passports = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(map.clone());
            map.clear();
        }
        for cap in re.captures_iter(line) {
            map.insert(cap[1].to_string(), cap[2].to_string());
        }
    }
    passports.push(map.clone());

    Ok(passports.to_owned())
}

fn part1(input: &Vec<HashMap<String, String>>) -> Result<()> {
    let valid = input
        .iter()
        .filter(|passport| {
            vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|&field| passport.contains_key(field))
        })
        .count() as u32;

    println!("{}", valid);

    Ok(())
}

fn part2(input: &Vec<HashMap<String, String>>) -> Result<()> {
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$")?;
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$")?;
    let ecl_re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$")?;
    let pid_re = Regex::new(r"^\d{9}$")?;
    let valid = input
        .iter()
        .filter(|passport| {
            vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|&field| match passport.get(field) {
                    Some(value) => match field {
                        "byr" => {
                            let n: u32 = value.parse().unwrap_or(0);
                            value.len() == 4 && n >= 1920 && n <= 2002
                        }
                        "iyr" => {
                            let n: u32 = value.parse().unwrap_or(0);
                            value.len() == 4 && n >= 2010 && n <= 2020
                        }
                        "eyr" => {
                            let n: u32 = value.parse().unwrap_or(0);
                            value.len() == 4 && n >= 2020 && n <= 2030
                        }
                        "hgt" => {
                            if let Some(cap) = hgt_re.captures(value) {
                                let n: u32 = cap[1].parse().unwrap_or(0);
                                let cm = &cap[2] == "cm";
                                if cm {
                                    if n >= 150 && n <= 193 {
                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    if n >= 59 && n <= 76 {
                                        true
                                    } else {
                                        false
                                    }
                                }
                            } else {
                                false
                            }
                        }
                        "hcl" => hcl_re.is_match(value),
                        "ecl" => ecl_re.is_match(value),
                        "pid" => pid_re.is_match(value),
                        _ => true,
                    },
                    None => false,
                })
        })
        .count() as u32;

    println!("{}", valid);

    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let parsed = parse_input(&input)?;

    part1(&parsed)?;
    part2(&parsed)?;

    Ok(())
}
