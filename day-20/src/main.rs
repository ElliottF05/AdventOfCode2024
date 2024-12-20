use std::collections::VecDeque;

const MAX: i32 = 1_000_000;

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

    let (mut start_r, mut start_c) = (0,0);
    let (mut end_r, mut end_c) = (0,0);
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'S' {
                (start_r, start_c) = (r,c);
            }
            if grid[r][c] == b'E' {
                (end_r, end_c) = (r,c);
            }
        }
    }

    let mut dist_start = vec![
        vec![
            MAX;
            grid[0].len()
        ];
        grid.len()
    ];
    let mut dist_end = vec![
        vec![
            MAX;
            grid[0].len()
        ];
        grid.len()
    ];

    explore(start_r, start_c, b'E', &grid, &mut dist_start);
    explore(end_r, end_c, b'S', &grid, &mut dist_end);

    let base_dist = dist_start[end_r][end_c];

    let mut res = 0;
    let mut skipped = vec![0 ; base_dist as usize];

    for r0 in 1..(grid.len()-1) as i32 {
        for c0 in 1..(grid[0].len()-1) as i32 {
            const RADIUS: i32 = 2;
            for dr in -RADIUS..=RADIUS {
                for dc in -RADIUS..=RADIUS {
                    let (r1, c1) = (r0+dr, c0+dc);
                    if !in_bounds(r1, c1, &grid) {
                        continue;
                    }
                    let tiles_skipped = dr.abs() + dc.abs();
                    if tiles_skipped > RADIUS {
                        continue;
                    }
                    let cheat_dist = 
                        dist_start[r0 as usize][c0 as usize]
                        + dist_end[r1 as usize][c1 as usize]
                        + tiles_skipped;
                    if cheat_dist <= base_dist - 100 {
                        res += 1;
                        skipped[(base_dist - cheat_dist) as usize] += 1;
                    }
                    // if base_dist == cheat_dist + 64 {
                    //     println!("({r0},{c0}) -> ({r1},{c1})")
                    // }
                }
            }
        }
    }

    // println!("{:?}", skipped);
    return res;

    fn explore(r0: usize, c0: usize, target: u8, grid: &Vec<Vec<u8>>, dist: &mut Vec<Vec<i32>>) {
        let mut queue = VecDeque::new();
        queue.push_back((r0,c0));
        let mut step = 0;

        while !queue.is_empty() {
            let (r,c) = queue.pop_front().unwrap();
            if dist[r][c] != MAX || grid[r][c] == b'#' {
                continue;
            }
            dist[r][c] = step;

            if grid[r][c] == target {
                break;
            }

            queue.push_back((r-1,c));
            queue.push_back((r+1,c));
            queue.push_back((r,c-1));
            queue.push_back((r,c+1));
            step += 1;
        }
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }
}




fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let (mut start_r, mut start_c) = (0,0);
    let (mut end_r, mut end_c) = (0,0);
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'S' {
                (start_r, start_c) = (r,c);
            }
            if grid[r][c] == b'E' {
                (end_r, end_c) = (r,c);
            }
        }
    }

    let mut dist_start = vec![
        vec![
            MAX;
            grid[0].len()
        ];
        grid.len()
    ];
    let mut dist_end = vec![
        vec![
            MAX;
            grid[0].len()
        ];
        grid.len()
    ];

    explore(start_r, start_c, b'E', &grid, &mut dist_start);
    explore(end_r, end_c, b'S', &grid, &mut dist_end);

    let base_dist = dist_start[end_r][end_c];

    let mut res = 0;
    let mut skipped = vec![0 ; base_dist as usize];

    for r0 in 1..(grid.len()-1) as i32 {
        for c0 in 1..(grid[0].len()-1) as i32 {
            const RADIUS: i32 = 20;
            for dr in -RADIUS..=RADIUS {
                for dc in -RADIUS..=RADIUS {
                    let (r1, c1) = (r0+dr, c0+dc);
                    if !in_bounds(r1, c1, &grid) {
                        continue;
                    }
                    let tiles_skipped = dr.abs() + dc.abs();
                    if tiles_skipped > RADIUS {
                        continue;
                    }
                    let cheat_dist = 
                        dist_start[r0 as usize][c0 as usize]
                        + dist_end[r1 as usize][c1 as usize]
                        + tiles_skipped;
                    if cheat_dist <= base_dist - 100 {
                        res += 1;
                        skipped[(base_dist - cheat_dist) as usize] += 1;
                    }
                    // if base_dist == cheat_dist + 64 {
                    //     println!("({r0},{c0}) -> ({r1},{c1})")
                    // }
                }
            }
        }
    }

    // println!("{:?}", skipped);
    return res;

    fn explore(r0: usize, c0: usize, target: u8, grid: &Vec<Vec<u8>>, dist: &mut Vec<Vec<i32>>) {
        let mut queue = VecDeque::new();
        queue.push_back((r0,c0));
        let mut step = 0;

        while !queue.is_empty() {
            let (r,c) = queue.pop_front().unwrap();
            if dist[r][c] != MAX || grid[r][c] == b'#' {
                continue;
            }
            dist[r][c] = step;

            if grid[r][c] == target {
                break;
            }

            queue.push_back((r-1,c));
            queue.push_back((r+1,c));
            queue.push_back((r,c-1));
            queue.push_back((r,c+1));
            step += 1;
        }
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }
}
