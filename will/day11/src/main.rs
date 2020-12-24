use anyhow::Result;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let parsed = parse_input(&input)?;

    part1(&parsed)?;
    part2(&parsed)?;

    Ok(())
}

/// Represents a tile state
#[derive(Debug, Clone)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}
use Tile::*;

fn part1(input: &Vec<Vec<Tile>>) -> Result<()> {
    let mut tiles: Vec<Vec<Tile>> = input.clone();
    loop {
        let mut changed = false;
        let mut tiles_new: Vec<Vec<Tile>> = tiles.clone();

        for y in 0..tiles.len() {
            for x in 0..tiles[y].len() {
                match tiles[y][x] {
                    Empty => {
                        if count_adj_occupied(&tiles, y, x)? == 0 {
                            tiles_new[y][x] = Occupied;
                            changed = true;
                        }
                    }
                    Occupied => {
                        if count_adj_occupied(&tiles, y, x)? >= 4 {
                            tiles_new[y][x] = Empty;
                            changed = true;
                        }
                    }
                    Floor => {}
                }
            }
        }

        tiles = tiles_new;

        if !changed {
            break;
        }
    }

    let mut count = 0;
    for row in tiles {
        for tile in row {
            match tile {
                Occupied => count += 1,
                _ => {}
            }
        }
    }

    println!("{}", count);

    Ok(())
}

fn part2(input: &Vec<Vec<Tile>>) -> Result<()> {
    let mut tiles: Vec<Vec<Tile>> = input.clone();
    loop {
        let mut changed = false;
        let mut tiles_new: Vec<Vec<Tile>> = tiles.clone();

        for y in 0..tiles.len() {
            for x in 0..tiles[y].len() {
                match tiles[y][x] {
                    Empty => {
                        if count_vis_occupied(&tiles, y, x)? == 0 {
                            tiles_new[y][x] = Occupied;
                            changed = true;
                        }
                    }
                    Occupied => {
                        if count_vis_occupied(&tiles, y, x)? >= 5 {
                            tiles_new[y][x] = Empty;
                            changed = true;
                        }
                    }
                    Floor => {}
                }
            }
        }

        tiles = tiles_new;

        if !changed {
            break;
        }
    }

    let mut count = 0;
    for row in tiles {
        for tile in row {
            match tile {
                Occupied => count += 1,
                _ => {}
            }
        }
    }

    println!("{}", count);

    Ok(())
}

/// Count the number of adjacent occupied tiles
fn count_adj_occupied(input: &Vec<Vec<Tile>>, y: usize, x: usize) -> Result<u32> {
    let mut count = 0;

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && i == j {
                continue;
            }

            // Prevent panics due to int overflow
            if y + i == 0 || x + j == 0 {
                continue;
            }

            if let Some(row) = input.get(y + i - 1) {
                if let Some(tile) = row.get(x + j - 1) {
                    match tile {
                        Occupied => count += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(count)
}

/// Count the number of visible occupied tiles
fn count_vis_occupied(input: &Vec<Vec<Tile>>, y: usize, x: usize) -> Result<u32> {
    let mut count = 0;

    for i in 0..3 {
        for j in 0..3 {
            let i = i as i32 - 1;
            let j = j as i32 - 1;
            let mut yk = y as i32 + i;
            let mut xk = x as i32 + j;

            if yk == y as i32 && xk == x as i32 {
                continue;
            }

            loop {
                if let Some(row) = input.get(yk as usize) {
                    if let Some(tile) = row.get(xk as usize) {
                        match tile {
                            Occupied => {
                                //println!("found occupied at {},{}", yk, xk);
                                count += 1;
                                break;
                            }
                            Empty => {
                                //println!("found empty at {},{}", yk, xk);
                                break;
                            }
                            _ => {}
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }

                yk += i;
                xk += j;
            }
        }
    }

    Ok(count)
}

/// Parse puzzle input into a 2D array of tiles
fn parse_input(input: &String) -> Result<Vec<Vec<Tile>>> {
    let mut tiles = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            match c {
                '.' => row.push(Floor),
                'L' => row.push(Empty),
                '#' => row.push(Occupied),
                _ => unreachable!(),
            }
        }
        tiles.push(row);
    }

    Ok(tiles)
}
