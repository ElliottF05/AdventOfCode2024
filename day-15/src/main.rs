use core::panic;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let parts: Vec<_> = input.split("\n\n").into_iter().collect();
    let grid_str = parts[0];
    let moves_str = parts[1];

    let mut grid: Vec<Vec<u8>> = grid_str
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let (mut r, mut c) = (0 as i32, 0 as i32);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row as usize][col as usize] == b'@' {
                (r,c) = (row as i32,col as i32);
            }
        }
    }
    grid[r as usize][c as usize] = b'.';

    for curr_move in moves_str.bytes() {
        let (mut dr, mut dc) = (0 as i32,0 as i32);
        match curr_move {
            b'^' => dr = -1,
            b'v' => dr = 1,
            b'<' => dc = -1,
            b'>' => dc = 1,
            _ => continue,
        }
        
        if grid[(r + dr) as usize][(c + dc) as usize] == b'.' {
            r += dr;
            c += dc;
        } else {
            let (mut r1, mut c1) = (r + dr, c + dc);
            while grid[r1 as usize][c1 as usize] == b'O' {
                r1 += dr;
                c1 += dc;
            }
            if grid[r1 as usize][c1 as usize] == b'.' {
                grid[r1 as usize][c1 as usize] = b'O';
                grid[(r + dr) as usize][(c + dc) as usize] = b'.';
                r += dr;
                c += dc;
            }
        }
    }

    let mut res = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'O' {
                res += 100 * r + c;
            }
        }
    }

    return res as i32;
}



fn part2(input: &str) -> i32 {
    let parts: Vec<_> = input.split("\n\n").into_iter().collect();
    let grid_str = parts[0];
    let moves_str = parts[1];

    let mut grid: Vec<Vec<u8>> = grid_str
        .lines()
        .map(|line| {
            line
                .bytes()
                .map(|b| match b {
                    b'#' => b"##".to_vec(),
                    b'O' => b"[]".to_vec(),
                    b'.' => b"..".to_vec(),
                    b'@' => b"@.".to_vec(),
                    _ => panic!("invalid char")
                })
                .collect::<Vec<Vec<u8>>>()
                .concat()
        })
        .collect();

    // for line in &grid {
    //     println!("{}", String::from_utf8(line.clone()).unwrap());
    // }

    let (mut r, mut c) = (0 as i32, 0 as i32);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row as usize][col as usize] == b'@' {
                (r,c) = (row as i32,col as i32);
            }
        }
    }
    grid[r as usize][c as usize] = b'.';

    // println!("starting grid");
    // for line in &grid {
    //     println!("{}", String::from_utf8(line.clone()).unwrap());
    // }
    // println!();

    for curr_move in moves_str.bytes() {
        let (mut dr, mut dc) = (0 as i32,0 as i32);
        match curr_move {
            b'^' => dr = -1,
            b'v' => dr = 1,
            b'<' => dc = -1,
            b'>' => dc = 1,
            _ => continue,
        }
        
        let next_tile = grid[(r + dr) as usize][(c + dc) as usize];
        if next_tile == b'.' {
            r += dr;
            c += dc;
        } else if next_tile == b'#' {
            // do nothing
        } else { // next tile is a box
            if dr != 0 {
                if explore_vert(r+dr, c+dc, dr, &mut grid) {
                    r += dr;
                    c += dc;
                };
            } else if dc != 0 {
                if explore_horiz(r+dr, c+dc, dc, &mut grid) {
                    r += dr;
                    c += dc;
                }
            }
        }

        // println!("move: {}", curr_move as char);
        // for line in &grid {
        //     println!("{}", String::from_utf8(line.clone()).unwrap());
        // }
        // println!();
    }

    let mut res = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'[' {
                res += 100 * r + c;
            }
        }
    }

    return res as i32;
}

fn explore_vert(r0: i32, c0: i32, dr: i32, grid: &mut Vec<Vec<u8>>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((r0, c0));

    let mut tiles_to_move = Vec::new();

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (r,c) = queue.pop_front().unwrap();
            match grid[r as usize][c as usize] {
                b'.' => {},
                b'#' => return false, // no moves possible, stuck at wall
                b'[' => {
                    queue.push_back((r + dr, c));
                    queue.push_back((r+dr, c+1));
                    tiles_to_move.push((r,c, b'['));
                    tiles_to_move.push((r,c+1, b']'));
                },
                b']' => {
                    queue.push_back((r + dr, c));
                    queue.push_back((r+dr, c-1));
                    tiles_to_move.push((r,c, b']'));
                    tiles_to_move.push((r,c-1, b'['));
                },
                _ => panic!()
            }
        }
    }

    for (r, c, _) in &tiles_to_move {
        grid[*r as usize][*c as usize] = b'.';
    }
    for (r, c, chr) in &tiles_to_move {
        grid[(r + dr) as usize][*c as usize] = *chr;
    }

    return true;
}

fn explore_horiz(r0: i32, c0: i32, dc: i32, grid: &mut Vec<Vec<u8>>) -> bool {
    let mut tiles_to_move = Vec::new();
    let mut c = c0;
    while grid[r0 as usize][c as usize] == b'[' || grid[r0 as usize][c as usize] == b']' {
        tiles_to_move.push((r0,c,grid[r0 as usize][c as usize]));
        c += dc;
    }

    if grid[r0 as usize][c as usize] == b'#' {
        return false;
    }

    for (r, c, _) in &tiles_to_move {
        grid[*r as usize][*c as usize] = b'.';
    }
    for (r, c, chr) in &tiles_to_move {
        grid[*r as usize][(c + dc) as usize] = *chr;
    }

    return true;
}
