use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut freq_to_location = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;

    input
        .lines()
        .enumerate()
        .for_each(|(r, line)| {
            rows = (r+1) as i32;
            line
                .bytes()
                .into_iter()
                .enumerate()
                .for_each(|(c, chr)| {
                    cols = (c+1) as i32;
                    if chr == b'.' {
                        return;
                    }
                    freq_to_location
                        .entry(chr.clone())
                        .or_insert(vec![])
                        .push((r as i32, c as i32));
            });
        });
    
    // println!("{:?}", freq_to_location);
    
    let mut antinodes = HashSet::new();

    for (_, locations) in freq_to_location {
        for pair in locations.iter().combinations(2) {
            let (r0, c0) = pair[0];
            let (r1, c1) = pair[1];
            let (dr, dc) = (r1 - r0, c1 - c0); // points from r0 -> r1

            let (r,c) = (r1 + dr, c1 + dc);
            if in_bounds(r, c, rows, cols) {
                antinodes.insert((r,c));
            }
            let (r,c) = (r0 - dr, c0 - dc);
            if in_bounds(r, c, rows, cols) {
                antinodes.insert((r,c));
            }
        }
    }

    fn in_bounds(r: i32, c: i32, rows: i32, cols: i32) -> bool {
        return r >= 0 && c >= 0 && r < rows && c < cols;
    }

    return antinodes.len() as i32;
}


fn part2(input: &str) -> i32 {
    let mut freq_to_location = HashMap::new();
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().len() as i32;

    input
        .lines()
        .enumerate()
        .for_each(|(r, line)| {
            line
                .bytes()
                .enumerate()
                .filter(|(_, chr)| *chr != b'.')
                .for_each(|(c, chr)| {
                    freq_to_location
                        .entry(chr)
                        .or_insert(vec![])
                        .push((r as i32, c as i32));
            });
        });
    
    // println!("{:?}", freq_to_location);
    
    let mut antinodes = HashSet::new();

    for (_, locations) in freq_to_location {
        for pair in locations.iter().combinations(2) {
            let (r0, c0) = pair[0];
            let (r1, c1) = pair[1];
            let (dr, dc) = (r1 - r0, c1 - c0); // points from r0 -> r1

            let mut i = 0;
            while in_bounds(r0 + i*dr, c0 + i*dc, rows, cols) {
                antinodes.insert((r0 + i*dr, c0 + i*dc));
                i += 1;
            }
            i = -1;
            while in_bounds(r0 + i*dr, c0 + i*dc, rows, cols) {
                antinodes.insert((r0 + i*dr, c0 + i*dc));
                i -= 1;
            }
        }
    }

    fn in_bounds(r: i32, c: i32, rows: i32, cols: i32) -> bool {
        return r >= 0 && c >= 0 && r < rows && c < cols;
    }

    return antinodes.len() as i32;
}

