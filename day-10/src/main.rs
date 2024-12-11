fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| if b == b'.' {11} else {(b - b'0') as u8}).collect())
        .collect();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    // println!("{:?}\n", grid);

    let mut status = vec![vec![-1 ; cols] ; rows];

    fn dfs(r: i32, c: i32, curr_status: i32, expected_height: u8, grid: &Vec<Vec<u8>>, status: &mut Vec<Vec<i32>>) -> i32 {
        if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[0].len() as i32 {
            return 0;
        }
        let curr_height = grid[r as usize][c as usize];
        if curr_height != expected_height {
            return 0;
        }
        if status[r as usize][c as usize] == curr_status {
            return 0;
        }
        status[r as usize][c as usize] = curr_status;
        if curr_height == 9 {
            return 1;
        }

        return dfs(r-1, c, curr_status, expected_height+1, grid, status)
            + dfs(r+1, c, curr_status, expected_height+1, grid, status)
            + dfs(r, c-1, curr_status, expected_height+1, grid, status)
            + dfs(r, c+1, curr_status, expected_height+1, grid, status);
    }

    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                res += dfs(r as i32, c as i32,(r*cols + c) as i32, 0, &grid, &mut status);
            }
        }
    }

    return res;
}



fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| if b == b'.' {11} else {(b - b'0') as u8}).collect())
        .collect();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    // println!("{:?}\n", grid);

    fn dfs(r: i32, c: i32, curr_status: i32, expected_height: u8, grid: &Vec<Vec<u8>>) -> i32 {
        if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[0].len() as i32 {
            return 0;
        }
        let curr_height = grid[r as usize][c as usize];
        if curr_height != expected_height {
            return 0;
        }
        if curr_height == 9 {
            return 1;
        }

        return dfs(r-1, c, curr_status, expected_height+1, grid)
            + dfs(r+1, c, curr_status, expected_height+1, grid)
            + dfs(r, c-1, curr_status, expected_height+1, grid)
            + dfs(r, c+1, curr_status, expected_height+1, grid);
    }

    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                res += dfs(r as i32, c as i32,(r*cols + c) as i32, 0, &grid);
            }
        }
    }

    return res;
}
