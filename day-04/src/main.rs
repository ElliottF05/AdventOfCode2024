use std::{collections::HashSet, hash::Hash};

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    fn check_direction(grid: &Vec<Vec<u8>>, r0: i32, c0: i32, dr: i32, dc: i32) -> bool {
        let pattern = "XMAS".as_bytes().to_vec();
        for i in 0..4 {
            let r = r0 + i*dr;
            let c = c0 + i*dc;

            if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[0].len() as i32 {
                return false;
            }
            if grid[r as usize][c as usize] != pattern[i as usize] {
                return false;
            }
        }
        return true;
    }

    let mut res = 0;
    let directions = vec![(1,1), (1,0), (1,-1), (0,1), (0,-1), (-1,1), (-1,0), (-1,-1)];
    for r0 in 0..grid.len() as i32 {
        for c0 in 0..grid[0].len() as i32 {
            for (dr, dc) in &directions {
                if check_direction(&grid, r0, c0, dr.clone(), dc.clone()) {
                    res += 1
                }
            }
        }
    }

    return res;

}

fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    fn check_location(grid: &Vec<Vec<u8>>, r: usize, c: usize) -> bool {
        if grid[r][c] != 'A' as u8 {
            return false;
        }

        let target: HashSet<u8> = HashSet::from(['M' as u8, 'S' as u8]);
        let diag1: HashSet<u8> = HashSet::from([ grid[r-1][c-1], grid[r+1][c+1] ]);
        let diag2: HashSet<u8> = HashSet::from([ grid[r-1][c+1], grid[r+1][c-1] ]);

        return target == diag1 && target == diag2;
    }

    let mut res = 0;
    for r in 1..grid.len()-1 {
        for c in 1..grid[0].len()-1 {
            if check_location(&grid, r, c) {
                res += 1;
            }
        }
    }

    return res;
}
