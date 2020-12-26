use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<(Vec<Field>, Ticket, Vec<Ticket>)> {
    let mut sections = input.split("\n\n");
    let fields: Vec<Field> = sections
        .next()
        .unwrap()
        .lines()
        .map(Field::from_string)
        .collect();

    let own_ticket: Ticket = Ticket::from_string(sections.next().unwrap().lines().nth(1).unwrap());

    let nearby_tickets: Vec<Ticket> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(Ticket::from_string)
        .collect();

    Ok((fields, own_ticket, nearby_tickets))
}

#[aoc(day16, part1)]
pub fn part1(input: &(Vec<Field>, Ticket, Vec<Ticket>)) -> Result<u32> {
    let (fields, own_ticket, nearby_tickets) = input;

    let sum = nearby_tickets
        .iter()
        .map(|ticket| ticket.get_invalid(fields))
        .flatten()
        .sum();

    Ok(sum)
}

#[aoc(day16, part2)]
pub fn part2(input: &(Vec<Field>, Ticket, Vec<Ticket>)) -> Result<u64> {
    let (fields, own_ticket, nearby_tickets) = input;

    let mut valid: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|ticket| ticket.get_invalid(fields).is_empty())
        .collect();
    valid.push(own_ticket);

    // Get all candidates
    let mut candidates: HashMap<String, HashSet<usize>> = fields
        .iter()
        .map(|field| {
            (
                field.name.clone(),
                (0..own_ticket.values.len())
                    .filter(|i| valid.iter().all(|t| field.is_valid(t.values[*i])))
                    .collect(),
            )
        })
        .collect();

    // Pull out candiadtes into known as we find loners
    let mut known: HashMap<String, usize> = HashMap::new();
    while !candidates.is_empty() {
        let (field_name, i) = candidates
            .iter()
            .filter(|(_, possible)| possible.len() == 1)
            .map(|(f, i)| (f.clone(), *i.iter().next().unwrap()))
            .next()
            .unwrap();

        candidates.remove(&field_name);
        known.insert(field_name, i);

        for c in candidates.values_mut() {
            c.remove(&i);
        }
    }

    // Get the product
    let product: u64 = known
        .iter()
        .filter(|(field_name, _)| field_name.starts_with("departure"))
        .map(|(_, i)| own_ticket.values[*i] as u64)
        .product();

    Ok(product)
}

#[derive(Debug)]
pub struct Ticket {
    pub values: Vec<u32>,
}

impl Ticket {
    pub fn from_string(s: &str) -> Self {
        let values = s.split(",").map(|s| s.parse().unwrap()).collect();
        Ticket { values }
    }

    pub fn get_invalid(&self, fields: &Vec<Field>) -> Vec<u32> {
        let mut invalid = vec![];
        for value in self.values[..].iter() {
            let mut valid = false;
            for field in fields {
                if field.is_valid(*value) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                invalid.push(*value);
            }
        }

        invalid
    }

    pub fn get_valid(&self, fields: &Vec<Field>) -> Vec<u32> {
        let mut valid = vec![];
        for value in self.values[..].iter() {
            for field in fields {
                if field.is_valid(*value) {
                    valid.push(*value);
                }
            }
        }

        valid
    }

    pub fn get_valid_field_names(&self, pos: usize, fields: &Vec<Field>) -> Vec<String> {
        fields
            .iter()
            .filter(|field| field.is_valid(*self.values.get(pos).unwrap()))
            .map(|field| field.name.clone())
            .collect()
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    ranges: Vec<(u32, u32)>,
}

impl Field {
    pub fn from_string(s: &str) -> Self {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"([a-z ]+):").unwrap();
            static ref RANGE_RE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
        }

        let name = if let Some(cap) = NAME_RE.captures(s) {
            cap[1].to_string()
        } else {
            "unknown".to_string()
        };

        let ranges: Vec<(u32, u32)> = {
            let mut ranges = vec![];

            for cap in RANGE_RE.captures_iter(s) {
                ranges.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
            }

            ranges
        };

        Field { name, ranges }
    }

    pub fn is_valid(&self, num: u32) -> bool {
        for range in self.ranges[..].iter() {
            if (range.0..=range.1).contains(&num) {
                return true;
            }
        }

        return false;
    }
}
