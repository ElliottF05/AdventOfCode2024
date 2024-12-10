use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    // println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CellStatus {
    Empty,
    Visited,
    Obstacle
}

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    cell_status: CellStatus,
    dir_indices: HashSet<i32>

}

fn part1(input: &str) -> i32 {
    let mut guard_r = 0;
    let mut guard_c = 0;
    let mut grid: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(r, line)| 
            line
            .as_bytes()
            .to_vec()
            .iter()
            .enumerate()
            .map(|(c, cell)| 
                Cell {
                    cell_status: match cell {
                        b'.' => CellStatus::Empty,
                        b'^' => {
                            guard_r = r as i32;
                            guard_c = c as i32;
                            CellStatus::Empty
                        },
                        b'#' => CellStatus::Obstacle,
                        _ => CellStatus::Empty
                    },
                    dir_indices: HashSet::new()
                }
            )
            .collect()
        )
        .collect();

    fn in_bounds(r: &i32, c: &i32, grid: &Vec<Vec<Cell>>) -> bool {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        return r >= &0 && c >= &0 && r < &rows && c < &cols;
    }


    let directions = vec![(-1,0), (0,1), (1,0), (0,-1)];
    let mut dir_index = 0;
    
    let mut res = 0;
    while in_bounds(&guard_r, &guard_c, &grid) {
        let curr_cell = &mut grid[guard_r as usize][guard_c as usize];
        let curr_direction = directions[dir_index % directions.len()];
        match (*curr_cell).cell_status {
            CellStatus::Empty => {
                res += 1;
                (*curr_cell).cell_status = CellStatus::Visited;
                (guard_r, guard_c) = (guard_r + curr_direction.0, guard_c + curr_direction.1);
            },
            CellStatus::Obstacle => {
                (guard_r, guard_c) = (guard_r - curr_direction.0, guard_c - curr_direction.1);
                dir_index += 1;
            }
            CellStatus::Visited => {
                (guard_r, guard_c) = (guard_r + curr_direction.0, guard_c + curr_direction.1);
            }
        }
    }

    // println!("{:?}", grid);

    return res;
}


fn part2(input: &str) -> i32 {
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();

    let mut rows = 0;
    let mut cols = 0;

    let mut start_pos = (0,0);
    for (r, line) in input.lines().enumerate() {
        rows = (r+1) as i32;
        for (c, cell_char) in line.bytes().enumerate() {
            cols = (c+1) as i32;
            if cell_char == b'#' {
                obstacles.insert((r as i32,c as i32));
            } else if cell_char == b'^' {
                start_pos = (r as i32, c as i32);
            }  
        }
    }

    let mut seen = HashSet::new();
    let mut dirs = vec![(-1,0), (0,1), (1,0), (0,-1)].into_iter().cycle();
    let mut dir = dirs.next().unwrap();
    let mut curr_pos = start_pos.clone();
    while in_bounds(curr_pos, rows, cols) {
        seen.insert((curr_pos.0, curr_pos.1));
        if obstacles.contains(&curr_pos) {
            curr_pos.0 -= dir.0;
            curr_pos.1 -= dir.1;
            dir = dirs.next().unwrap();
        } else {
            seen.insert(curr_pos);
            curr_pos.0 += dir.0;
            curr_pos.1 += dir.1;
        }
    }

    let mut res = 0;
    for obstacle_pos in seen {
        if obstacle_pos.0 == start_pos.0 && obstacle_pos.1 == start_pos.1 {
            continue;
        }
        if obstacles.contains(&obstacle_pos) {
            continue;
        }

        obstacles.insert(obstacle_pos);
        if is_cycle(start_pos, &obstacles, rows, cols) {
            res += 1;
        }
        obstacles.remove(&obstacle_pos);
    }

    fn in_bounds(pos: (i32, i32), rows: i32, cols: i32) -> bool {
        return pos.0 >= 0 && pos.1 >= 0 && pos.0 < rows && pos.1 < cols;
    }

    fn is_cycle(mut pos: (i32, i32), obstacles: &HashSet<(i32, i32)>, rows: i32, cols: i32) -> bool {
        let mut dirs = vec![(-1,0), (0,1), (1,0), (0,-1)].into_iter().cycle();
        let mut dir = dirs.next().unwrap();
        let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        while in_bounds(pos, rows, cols) {
            if visited.contains(&(pos, dir)) {
                return true;
            }
            if obstacles.contains(&pos) {
                pos.0 -= dir.0;
                pos.1 -= dir.1;
                dir = dirs.next().unwrap();
            } else {
                visited.insert((pos, dir));
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
        }
        return false;
    }

    return res
}