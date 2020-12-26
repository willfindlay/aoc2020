use anyhow::Result;
use std::collections::HashMap;

#[aoc_generator(day17, part1)]
pub fn input_generator_part1(input: &str) -> Result<HashMap<(i64, i64, i64), bool>> {
    let mut state = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            state.insert((j as i64, i as i64, 0), c == '#');
        }
    }
    Ok(state)
}

#[aoc_generator(day17, part2)]
pub fn input_generator_part2(input: &str) -> Result<HashMap<(i64, i64, i64, i64), bool>> {
    let mut state = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            state.insert((j as i64, i as i64, 0, 0), c == '#');
        }
    }
    Ok(state)
}

#[aoc(day17, part1)]
pub fn part1(state: &HashMap<(i64, i64, i64), bool>) -> Result<u32> {
    let mut state = state.clone();

    let mut xlo: i64 = *state.keys().map(|(x, _, _)| x).min().unwrap();
    let mut xhi: i64 = *state.keys().map(|(x, _, _)| x).max().unwrap();
    let mut ylo: i64 = *state.keys().map(|(_, y, _)| y).min().unwrap();
    let mut yhi: i64 = *state.keys().map(|(_, y, _)| y).max().unwrap();
    let mut zlo: i64 = 0;
    let mut zhi: i64 = 0;

    for _ in 0..6 {
        xlo -= 1;
        xhi += 1;
        ylo -= 1;
        yhi += 1;
        zlo -= 1;
        zhi += 1;

        let old = state.clone();
        for x in xlo..=xhi {
            for y in ylo..=yhi {
                for z in zlo..=zhi {
                    let count = count_active_neighbors(&old, x, y, z)?;
                    if *old.get(&(x, y, z)).unwrap_or(&false) {
                        state.insert((x, y, z), count == 2 || count == 3);
                    } else {
                        state.insert((x, y, z), count == 3);
                    }
                }
            }
        }
    }

    Ok(state.values().filter(|&&v| v).count() as u32)
}

#[aoc(day17, part2)]
pub fn part2(state: &HashMap<(i64, i64, i64, i64), bool>) -> Result<u32> {
    let mut state = state.clone();

    let mut xlo: i64 = *state.keys().map(|(x, _, _, _)| x).min().unwrap();
    let mut xhi: i64 = *state.keys().map(|(x, _, _, _)| x).max().unwrap();
    let mut ylo: i64 = *state.keys().map(|(_, y, _, _)| y).min().unwrap();
    let mut yhi: i64 = *state.keys().map(|(_, y, _, _)| y).max().unwrap();
    let mut zlo: i64 = 0;
    let mut zhi: i64 = 0;
    let mut wlo: i64 = 0;
    let mut whi: i64 = 0;

    for _ in 0..6 {
        xlo -= 1;
        xhi += 1;
        ylo -= 1;
        yhi += 1;
        zlo -= 1;
        zhi += 1;
        wlo -= 1;
        whi += 1;

        let old = state.clone();
        for x in xlo..=xhi {
            for y in ylo..=yhi {
                for z in zlo..=zhi {
                    for w in wlo..=whi {
                        let count = count_active_neighbors2(&old, x, y, z, w)?;
                        if *old.get(&(x, y, z, w)).unwrap_or(&false) {
                            state.insert((x, y, z, w), count == 2 || count == 3);
                        } else {
                            state.insert((x, y, z, w), count == 3);
                        }
                    }
                }
            }
        }
    }

    Ok(state.values().filter(|&&v| v).count() as u32)
}

fn count_active_neighbors(
    state: &HashMap<(i64, i64, i64), bool>,
    x: i64,
    y: i64,
    z: i64,
) -> Result<u32> {
    let mut count = 0;

    for i in (x - 1)..=(x + 1) {
        for j in (y - 1)..=(y + 1) {
            for k in (z - 1)..=(z + 1) {
                // Skip current
                if (i, j, k) == (x, y, z) {
                    continue;
                }

                // Count active
                if *state.get(&(i, j, k)).unwrap_or(&false) {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn count_active_neighbors2(
    state: &HashMap<(i64, i64, i64, i64), bool>,
    x: i64,
    y: i64,
    z: i64,
    w: i64,
) -> Result<u32> {
    let mut count = 0;

    for i in (x - 1)..=(x + 1) {
        for j in (y - 1)..=(y + 1) {
            for k in (z - 1)..=(z + 1) {
                for l in (w - 1)..=(w + 1) {
                    // Skip current
                    if (i, j, k, l) == (x, y, z, w) {
                        continue;
                    }

                    // Count active
                    if *state.get(&(i, j, k, l)).unwrap_or(&false) {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}
