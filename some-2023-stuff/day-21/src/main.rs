use std::{collections::{HashSet, VecDeque}, f32::consts::E, hash::Hash, i32::MAX, time::Instant};

const STEPS: i32 = 26501365;

// KEY ASSUMPTIONS:
// Pattern is a square with odd side length and with sidelength//2 being even
// S occurs at the exact center of the grid
// There is an unobstructed path from S in all 4 cardinal directions
// The outer edge of the pattern has no obstacles

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    let now = Instant::now();
    println!("PART 2 ANSWER: {}", part2(input, STEPS));
    let elapsed = now.elapsed();
    println!("time elapsed: {:.2?}", elapsed);
    // println!("PART 2_2 ANSWER: {}", part2_2(input, STEPS));

    // for stp in (1..5001).step_by(2) {
    //     if stp % 200 == 1 {
    //         println!("at step = {}", stp);
    //     }
    //     let a = part2(input, stp);
    //     let b = part2_2(input, stp);
    //     if a != b as i64 {
    //         println!("ERROR at steps = {}: a = {}, b = {}", stp, a, b);
    //     }
    // }
    // println!("finished testing");
}


fn part1(input: &str) -> i32 {
    
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    println!("grid dimensions: {}, {}", grid.len(), grid[0].len());

    let mut curr_positions = HashSet::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, chr) in row.iter().enumerate() {
            if *chr == b'S' {
                curr_positions.insert((r as i32,c as i32));
            }
        }
    }

    let steps = 64;
    let directions = vec![(1,0), (0,1), (-1,0), (0,-1)];

    for _ in 0..steps {
        let mut next_positions = HashSet::new();
        for (r,c) in curr_positions {
            for (dr, dc) in &directions {
                if valid_tile(r + dr, c + dc, &grid) {
                    next_positions.insert((r+dr, c+dc));
                }
            }
        }
        curr_positions = next_positions;
    }

    fn valid_tile(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return in_bounds(r, c, &grid) && grid[r as usize][c as usize] != b'#';
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }

    return curr_positions.len() as i32;
}



fn part2(input: &str, steps: i32) -> i64 {
    
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut start_pos = (0,0);
    for (r, row) in grid.iter().enumerate() {
        for (c, chr) in row.iter().enumerate() {
            if *chr == b'S' {
                start_pos = (r as i32, c as i32);
            }
        }
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut res: i64 = 0;

    // center tile:
    let (evens, odds, _) = explore(start_pos.0, start_pos.1, steps, true, &grid);
    let center: i64 = odds as i64;

    let mut axis: i64 = 0;
    axis += explore_axis(0, start_pos.1, steps, &grid);
    axis += explore_axis(rows-1, start_pos.1, steps, &grid);
    axis += explore_axis(start_pos.1, 0, steps, &grid);
    axis += explore_axis(start_pos.1, cols-1, steps, &grid);

    let mut diag: i64 = 0;
    diag += explore_diag(0, 0, steps, &grid);
    diag += explore_diag(rows-1, 0, steps, &grid);
    diag += explore_diag(0, cols-1, steps, &grid);
    diag += explore_diag(rows-1, cols-1, steps, &grid);

    res = center + axis + diag;
    // println!("center: {center}, axis: {axis}, diag: {diag}, total: {}", center+axis+diag);
    return res;

    fn explore_axis(r0: i32, c0: i32, steps: i32, grid: &Vec<Vec<u8>>) -> i64 {
        if steps < 66 {
            return 0;
        }
        let size = grid.len() as i32;
        let (full_evens, full_odds, max_dist) = explore(r0, c0, MAX, true, grid);
        // x * size + max_dist + 66 <= steps
        let max_x = (steps - 66) / size;
        // println!("max_x: {}, full_odds: {}", max_x, full_odds);
        let mut x = max_x;
        let mut res: i64 = 0;
        while x >= 0 {
            let curr_even = x % 2 == 0;
            let (partial_evens, partial_odds, _) = explore(r0, c0, steps - 66 - x*size, curr_even, grid);
            if partial_odds != full_odds {
                res += partial_odds as i64;
                x -= 1;
            } else {
                // println!("partial_odds == full_odds, {}", partial_odds);
                if (x+1) % 2 == 0 {
                    res += ((x+1) / 2) as i64 * (full_odds + full_evens) as i64;
                } else {
                    let half_x = ((x+1) / 2) as i64;
                    res += (half_x + 1) * full_odds as i64 + half_x * full_evens as i64;
                }
                // res += (x+1) * full_odds;
                break;
            }
        }
        return res;
    }

    fn explore_diag(r0: i32, c0: i32, steps: i32, grid: &Vec<Vec<u8>>) -> i64 {
        if steps < 2*66 {
            return 0;
        }
        let size = grid.len() as i32;
        let (full_evens, full_odds, max_dist) = explore(r0, c0, MAX, true, grid);
        // x * size + max_dist + 66 <= steps
        let max_x = (steps - 2*66) / size;
        // println!("max_x: {}", max_x);
        let mut x = max_x;
        let mut partial_totals = Vec::new();
        while x >= 0 {
            let curr_even = x % 2 == 0;
            let (partial_evens, partial_odds, _) = explore(r0, c0, steps - 2*66 - x*size, curr_even, grid);
            if partial_odds != full_odds {
                partial_totals.push(partial_odds as i64);
                x -= 1;
            } else {
                break;
            }
        }
        let partials_sum: i64 = partial_totals.iter().sum();
        // println!("partial_totals: {:?}, partials_sum: {}, x: {}", partial_totals, partials_sum, x);
        
        let mut res = partials_sum * (x as i64 + 1);
        for i in (0..partial_totals.len()).rev() {
            partial_totals[i] *= (partial_totals.len() - i) as i64;
        }
        res += partial_totals.iter().sum::<i64>();

        let l = x+1;
        let a = (l+1) / 2; // a occurs first!! (is even)
        let b = (l) / 2; 

        let sum1 = a as i64 * a as i64; // 1 + 3 + 5 + ... 2a-1 = a(a+1)-a = a^2 - 1
        let sum2 = b as i64 * (b as i64 +1); // 2 + 4 + 6 + ... + 2b = 2 (1 + 2 + 3 + ... + b)

        res += (sum1 as i64) * (full_odds as i64);
        res += (sum2 as i64) * (full_evens as i64);

        return res;
    }


    fn explore(r0: i32, c0: i32, max_depth: i32, mut curr_even: bool, grid: &Vec<Vec<u8>>) -> (i32, i32, i32) {

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut odds = 0;
        let mut evens = 0;
        let mut depth = 0;

        queue.push_back((r0,c0));

        while !queue.is_empty() && depth <= max_depth {
            depth += 1;
            for _ in 0..queue.len() {
                let (r,c) = queue.pop_front().unwrap();
                if !valid_tile(r, c, grid) || visited.contains(&(r,c)) {
                    continue;
                }
                visited.insert((r,c));

                if curr_even {
                    evens += 1;
                } else {
                    odds += 1;
                }
                queue.push_back((r-1,c));
                queue.push_back((r+1,c));
                queue.push_back((r,c-1));
                queue.push_back((r,c+1));
            }
            curr_even = !curr_even;
        }

        return (evens,odds,depth-2);
    }

    fn valid_tile(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return in_bounds(r, c, &grid) && grid[r as usize][c as usize] != b'#';
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }

    return res;
}



fn part2_2(input: &str, steps: i32) -> i32 {
    // THIS IS A BRUTE-FORCE SOLUTION USED TO TEST UP TO STEPS ~= 5000
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut r0 = 0;
    let mut c0 = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, chr) in row.iter().enumerate() {
            if *chr == b'S' {
                r0 = r as i32;
                c0 = c as i32;
            }
        }
    };

    let (evens, odds, max_dist) = explore(r0, c0, steps, &grid);
    if steps % 2 == 0 {
        return evens;
    } else {
        return odds;
    }

    fn explore(r0: i32, c0: i32, max_depth: i32, grid: &Vec<Vec<u8>>) -> (i32, i32, i32) {

        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut odds = 0;
        let mut evens = 0;
        let mut axis = 0;
        let mut diag = 0;
        let mut center = 0;
        let mut curr_even = true;
        let mut depth = 0;

        queue.push_back((r0,c0));

        let size = 1 + (2 * max_depth) / rows + 1; // should be an odd number
        let offset = size / 2;
        let mut status_vec = vec![vec![0 ; size as usize]; size as usize];

        while !queue.is_empty() && depth <= max_depth {
            depth += 1;
            for _ in 0..queue.len() {
                let (r,c) = queue.pop_front().unwrap();
                if !valid_tile(r, c, grid) || visited.contains(&(r,c)) {
                    continue;
                }
                visited.insert((r,c));

                if !curr_even {
                    if 0 <= r && r < rows && 0 <= c && c < cols {
                        center += 1;
                    } else if (0 <= r && r < rows) || (0 <= c && c < cols) {
                        axis += 1;
                    } else {
                        diag += 1;
                    }

                    let mut ir = (r as f32 / rows as f32).floor() as i32;
                    let mut ic = (c as f32 / cols as f32).floor() as i32;
                    ir += offset as i32;
                    ic += offset as i32;

                    status_vec[ir as usize][ic as usize] += 1;
                }

                if curr_even {
                    evens += 1;
                } else {
                    odds += 1;
                }
                queue.push_back((r-1,c));
                queue.push_back((r+1,c));
                queue.push_back((r,c-1));
                queue.push_back((r,c+1));
            }
            curr_even = !curr_even;
        }
        // println!("center: {center}, axis: {axis}, diag: {diag}, total: {}", center+axis+diag);
        for x in status_vec {
            // println!("{:?}", x);
        }
        // println!("{:?}", status_vec);
        return (evens,odds,depth-2);
    }

    fn valid_tile(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let (r_mod, c_mod) = (r.rem_euclid(rows), c.rem_euclid(cols));
        return in_bounds(r_mod, c_mod, &grid) && grid[r_mod as usize][c_mod as usize] != b'#';
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }
}