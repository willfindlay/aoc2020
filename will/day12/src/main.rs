use anyhow::Result;
use regex::Regex;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let parsed = parse_input(&input)?;

    part1(&parsed)?;
    part2(&parsed)?;

    Ok(())
}

fn part1(input: &Vec<(char, i32)>) -> Result<()> {
    let mut ferry = part1::Ferry::new();

    for (operation, units) in input {
        ferry.do_operation(*operation, *units);
    }

    println!("{}", ferry.get_manhattan());

    Ok(())
}

fn part2(input: &Vec<(char, i32)>) -> Result<()> {
    let mut ferry = part2::Ferry::new();

    for (operation, units) in input {
        ferry.do_operation(*operation, *units);
    }

    println!("{}", ferry.get_manhattan());

    Ok(())
}

fn parse_input(input: &String) -> Result<Vec<(char, i32)>> {
    let line_re = Regex::new(r"([NSEWLRF])(\d+)")?;
    let mut parsed = vec![];

    for line in input.lines() {
        if let Some(cap) = line_re.captures(line) {
            let direction = cap[1].chars().nth(0).unwrap();
            let units: i32 = cap[2].parse()?;
            parsed.push((direction, units));
        }
    }

    Ok(parsed)
}

mod part1 {
    pub struct Ferry {
        direction: char,
        x: i32,
        y: i32,
    }

    impl Ferry {
        pub fn new() -> Self {
            Ferry {
                direction: 'E',
                x: 0,
                y: 0,
            }
        }

        pub fn get_manhattan(&self) -> i32 {
            self.x.abs() + self.y.abs()
        }

        pub fn do_operation(&mut self, operation: char, value: i32) {
            match operation {
                'N' => self.travel_direction('N', value),
                'W' => self.travel_direction('W', value),
                'S' => self.travel_direction('S', value),
                'E' => self.travel_direction('E', value),
                'L' => self.turn_left(value),
                'R' => self.turn_right(value),
                'F' => self.go_straight(value),
                _ => panic!("Bad operation!"),
            }
        }

        fn turn_left(&mut self, value: i32) {
            if value <= 0 {
                return;
            }
            match self.direction {
                'N' => self.direction = 'W',
                'W' => self.direction = 'S',
                'S' => self.direction = 'E',
                'E' => self.direction = 'N',
                _ => panic!("Bad direction!"),
            }
            return self.turn_left(value - 90);
        }

        fn turn_right(&mut self, value: i32) {
            if value <= 0 {
                return;
            }
            match self.direction {
                'N' => self.direction = 'E',
                'W' => self.direction = 'N',
                'S' => self.direction = 'W',
                'E' => self.direction = 'S',
                _ => panic!("Bad direction!"),
            }
            return self.turn_right(value - 90);
        }

        fn go_straight(&mut self, value: i32) {
            self.travel_direction(self.direction, value);
        }

        fn travel_direction(&mut self, direction: char, value: i32) {
            match direction {
                'N' => self.y += value,
                'W' => self.x -= value,
                'S' => self.y -= value,
                'E' => self.x += value,
                _ => panic!("Bad direction!"),
            }
        }
    }
}

mod part2 {
    struct Waypoint {
        pub x: i32,
        pub y: i32,
    }

    pub struct Ferry {
        waypoint: Waypoint,
        x: i32,
        y: i32,
    }

    impl Ferry {
        pub fn new() -> Self {
            Ferry {
                waypoint: Waypoint { x: 10, y: 1 },
                x: 0,
                y: 0,
            }
        }

        pub fn get_manhattan(&self) -> i32 {
            self.x.abs() + self.y.abs()
        }

        pub fn do_operation(&mut self, operation: char, value: i32) {
            match operation {
                'N' => self.move_waypoint('N', value),
                'W' => self.move_waypoint('W', value),
                'S' => self.move_waypoint('S', value),
                'E' => self.move_waypoint('E', value),
                'L' => self.rotate_left(value),
                'R' => self.rotate_right(value),
                'F' => self.travel(value),
                _ => panic!("Bad operation!"),
            }
        }

        fn rotate_left(&mut self, value: i32) {
            if value <= 0 {
                return;
            }
            let x = self.waypoint.x;
            let y = self.waypoint.y;
            self.waypoint.x = -y;
            self.waypoint.y = x;
            return self.rotate_left(value - 90);
        }

        fn rotate_right(&mut self, value: i32) {
            if value <= 0 {
                return;
            }
            let x = self.waypoint.x;
            let y = self.waypoint.y;
            self.waypoint.x = y;
            self.waypoint.y = -x;
            return self.rotate_right(value - 90);
        }

        fn travel(&mut self, value: i32) {
            self.x += self.waypoint.x * value;
            self.y += self.waypoint.y * value;
        }

        fn move_waypoint(&mut self, direction: char, value: i32) {
            match direction {
                'N' => self.waypoint.y += value,
                'W' => self.waypoint.x -= value,
                'S' => self.waypoint.y -= value,
                'E' => self.waypoint.x += value,
                _ => panic!("Bad direction!"),
            }
        }
    }
}
