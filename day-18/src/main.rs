use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}


fn part1(input: &str) -> i32 {
    const MAX_COORD: i32 = 70;
    const MAX_BYTES: usize = 1024;

    let mut grid = vec![ vec![ false ; (MAX_COORD+1) as usize] ; (MAX_COORD+1) as usize];
    let bytes: Vec<_> = input.lines().map(|line| {
        let a: Vec<_> = line.split(",").map(|b| b.parse::<usize>().unwrap()).collect();
        (a[0], a[1])
    })
    .collect();

    for i in 0..(MAX_BYTES.min(bytes.len())) {
        let (r,c) = (bytes[i].0, bytes[i].1);
        grid[r][c] = true;
    }

    let mut queue = VecDeque::new();
    queue.push_back((0,0));

    // println!("{:?}", grid);

    for step in 0..(MAX_COORD+1)*(MAX_COORD+1) {
        for _ in 0..queue.len() {
            let (r,c) = queue.pop_front().unwrap();
            if r < 0 || c < 0 || r > MAX_COORD || c > MAX_COORD || grid[r as usize][c as usize] {
                continue;
            }
            if r == MAX_COORD && c == MAX_COORD {
                return step;
            }
            grid[r as usize][c as usize] = true;

            queue.push_back((r-1,c));
            queue.push_back((r+1,c));
            queue.push_back((r,c-1));
            queue.push_back((r,c+1));
        }
    }

    return -1;
}



fn part2(input: &str) -> String {
    const MAX_COORD: i32 = 70;

    let mut grid = vec![ vec![ false ; (MAX_COORD+1) as usize] ; (MAX_COORD+1) as usize];
    let bytes: Vec<_> = input.lines().map(|line| {
        let a: Vec<_> = line.split(",").map(|b| b.parse::<usize>().unwrap()).collect();
        (a[0], a[1])
    })
    .collect();

    for i in 0..bytes.len() {
        let (r,c) = (bytes[i].0, bytes[i].1);
        // println!("{},{}", r, c);
        grid[r][c] = true;
        if !explore(grid.clone()) {
            return format!("{},{}", r, c);
        }
    }

    return "error".to_owned();
}

fn explore(mut grid: Vec<Vec<bool>>) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((0 as i32, 0 as i32));

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (r,c) = queue.pop_front().unwrap();
            // println!("{},{}", r, c);
            if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[0].len() as i32 || grid[r as usize][c as usize] {
                continue;
            }
            if r == (grid.len()-1) as i32 && c == (grid[0].len()-1) as i32 {
                return true;
            }
            grid[r as usize][c as usize] = true;

            queue.push_back((r-1,c));
            queue.push_back((r+1,c));
            queue.push_back((r,c-1));
            queue.push_back((r,c+1));
        }
    }

    return false;
}
