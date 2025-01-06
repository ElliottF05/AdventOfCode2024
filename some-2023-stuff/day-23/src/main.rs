use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let initial_col = grid[0].iter().position(|x| *x == b'.').unwrap() as i32;
    
    return explore(0, initial_col, 0, &mut HashSet::new(), &grid).unwrap();


    fn explore(r: i32, c: i32, step: i32, visited: &mut HashSet<(i32, i32)>, grid: &Vec<Vec<u8>>) -> Option<i32> {
        if !in_bounds(r, c, grid) || grid[r as usize][c as usize] == b'#' || visited.contains(&(r,c)) {
            return None;
        }
        if r == grid[0].len() as i32 - 1 {
            return Some(step);
        }

        // println!("r,c: {}, {}", r,c);
        visited.insert((r,c));

        let mut res = None;

        match grid[r as usize][c as usize] {
            b'>' => res = explore(r, c+1, step+1, visited, grid),
            b'<' => res = explore(r, c-1, step+1, visited, grid),
            b'^' => res = explore(r-1, c, step+1, visited, grid),
            b'v' => res = explore(r+1, c, step+1, visited, grid),
            _ => {
                let dirs = vec![(1,0), (0,1), (-1,0), (0,-1)];
                for (dr, dc) in dirs {
                    if let Some(potential_dist) = explore(r+dr, c+dc, step+1, visited, grid) {
                        if res.is_none() || (res.is_some() && potential_dist > res.unwrap()) {
                            res = Some(potential_dist);
                        }
                    }
                }
            }
        }

        visited.remove(&(r,c));
        return res;
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }
}


fn part2(input: &str) -> i32 {
    // 2 sec runtime
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut adj_list = HashMap::new();
    adj_list.insert((-1, -1), Vec::new());

    let mut visited = HashSet::new();

    let initial_col = grid[0].iter().position(|x| *x == b'.').unwrap() as i32;
    let mut last_col = -1;

    build_graph(0, initial_col, -1, -1, 0, &mut adj_list, &mut visited, &grid, &mut last_col);

    let res = find_longest_path((-1,-1), (grid[0].len() as i32 - 1, last_col), 0, &mut HashSet::new(), &adj_list).unwrap();

    return res;


    fn find_longest_path(pos: (i32, i32), goal: (i32, i32), dist: i32, visited: &mut HashSet<(i32, i32)>, adj_list: &HashMap<(i32, i32), Vec<(i32, i32, i32)>>) -> Option<i32> {
        // println!("pos: {:?}", pos);
        if visited.contains(&pos) {
            return None;
        }
        if pos == goal {
            return Some(dist);
        }

        visited.insert(pos);

        let mut res = None;
        for (r1, c1, d) in adj_list.get(&pos).unwrap() {
            let next_pos = (r1.clone(), c1.clone());
            if let Some(potential_dist) = find_longest_path(next_pos, goal, dist+d, visited, adj_list) {
                if res.is_none() || (res.is_some() && potential_dist > res.unwrap()) {
                    res = Some(potential_dist);
                }
            }
        }
    
        visited.remove(&pos);
        return res;
    }

    fn build_graph(r: i32, c: i32, mut from_r: i32, mut from_c: i32, mut steps: i32, adj_list: &mut HashMap<(i32, i32), Vec<(i32, i32, i32)>>, visited: &mut HashSet<(i32, i32)>, grid: &Vec<Vec<u8>>, last_col: &mut i32) {
        // println!("r,c: {},{}", r, c);

        if adj_list.contains_key(&(r,c)) && (from_r, from_c) != (r,c) {
            adj_list.get_mut(&(r,c)).unwrap().push((from_r, from_c, steps));
            adj_list.get_mut(&(from_r, from_c)).unwrap().push((r,c,steps));
            return;
        }

        if !in_bounds(r, c, grid) || grid[r as usize][c as usize] == b'#' || visited.contains(&(r,c)) {
            return;
        }

        visited.insert((r,c));

        let dirs = vec![(1,0), (0,1), (-1,0), (0,-1)];
        let mut degree = 0;
        for (dr, dc) in &dirs {
            if !in_bounds(r+dr, c+dc, grid) || grid[(r+dr) as usize][(c+dc) as usize] == b'#' {
                continue;
            }
            degree += 1;
        }

        if r == grid[0].len() as i32 - 1 {
            *last_col = c;
        }

        if degree > 2 || r == grid[0].len() as i32 - 1 { // at junction
            let curr_pos = (r, c);
            if adj_list.contains_key(&curr_pos) {
                println!("ERROR");
            }
            adj_list.insert(curr_pos, vec![(from_r, from_c, steps)]);
            adj_list.get_mut(&(from_r, from_c)).unwrap().push((r,c,steps));
            from_r = r;
            from_c = c;
            steps = 0;
        }
        steps += 1;

        for (dr, dc) in &dirs {
            build_graph(r+dr, c+dc, from_r, from_c, steps, adj_list, visited, grid, last_col);
        }
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }

}