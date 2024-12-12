use std::{collections::HashSet, time::Instant};

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));

    let t1 = Instant::now();
    println!("PART 2 ANSWER: {}", part2(input));
    println!("time elapsed: {:?}", t1.elapsed());

    let t2 = Instant::now();
    println!("PART 2_2 ANSWER: {}", part2_2(input));
    println!("time elapsed: {:?}", t2.elapsed());
}

fn part1(input: &str) -> i32 {

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut res = 0;
    let mut visited = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let (area, perim) = explore(r as i32,c as i32, grid[r][c], &mut visited, &grid);
            // println!("at tile {}, area: {}, perim: {}", grid[r][c] as char, area, perim);
            res += area * perim;
        }
    }

    return res;

    fn explore(r: i32, c: i32, curr: u8, visited: &mut HashSet<(i32, i32)>, grid: &Vec<Vec<u8>>)-> (i32, i32) {
        if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid.len() as i32
            || grid[r as usize][c as usize] != curr {
            return (0, 1);
        }
        if visited.contains(&(r,c)) {
            return (0, 0);
        }
        visited.insert((r,c));

        let mut area = 1;
        let mut perim = 0;
        let directions = vec![(1,0), (0,1), (-1,0), (0,-1)];

        for (dr,dc) in directions {
            let (da, dp) = explore(r + dr, c + dc, curr, visited, grid);
            area += da;
            perim += dp;
        }
        return (area, perim);
    }
}

fn part2(input: &str) -> i32 {
    // 873570 is TOO LOW

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut visited = HashSet::new();
    let mut edges = HashSet::new();

    let mut res = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            // edges.clear();
            let (area, sides) = explore2(r as i32,c as i32, grid[r][c], &mut visited, &mut edges, &grid);
            res += area * sides;
            // if area * sides != 0 {
            //     println!("at tile {}, area: {}, sides: {}", grid[r][c] as char, area, sides);
            //     println!("{:?}", edges);
            // }
        }
    }
    
    return res;

    fn explore2(r: i32, c: i32, curr: u8, visited: &mut HashSet<(i32, i32)>, edges: &mut HashSet<(i32, i32, i32, i32)>, grid: &Vec<Vec<u8>>)-> (i32, i32) {
        // println!("r: {}, c: {}, tile: {}", r, c, curr as char);
        if visited.contains(&(r,c)) {
            return (0,0);
        }
        visited.insert((r,c));

        let mut area = 1;
        let mut sides = 0;
        let directions = vec![(1,0), (0,1), (-1,0), (0,-1)];

        for (dr,dc) in directions {
            let (r1, c1) = (r + dr, c + dc);
            if !in_bounds(r1, c1, grid) || grid[r1 as usize][c1 as usize] != curr {
                sides += traverse_side(r, c, dr, dc, curr, edges, grid);
            } else {
                let (d_area, d_sides) = explore2(r + dr, c + dc, curr, visited, edges, grid);
                area += d_area;
                sides += d_sides;
            }
        }
        return (area, sides);

    }

    fn traverse_side(r: i32, c: i32, dr: i32, dc: i32, curr: u8, edges: &mut HashSet<(i32, i32, i32, i32)>, grid: &Vec<Vec<u8>>) -> i32 {
        // println!("traverse_side: r: {}, c: {}, dr: {}, dc: {}, curr: {}", r, c, dr, dc, curr as char);
        if edges.contains(&(r,c,dr,dc)) {
            //println!("duplicate");
            return 0;
        }

        let (traverse_r, traverse_c) = (dc, dr);

        let (mut r0, mut c0) = (r,c);

        while in_bounds(r0, c0, grid) && grid[r0 as usize][c0 as usize] == curr && (!in_bounds(r0 + dr, c0 + dc, grid) || grid[(r0 + dr) as usize][(c0 + dc) as usize] != curr) {
            edges.insert((r0, c0, dr, dc));
            r0 += traverse_r;
            c0 += traverse_c;
        }

        let (mut r0, mut c0) = (r,c);

        while in_bounds(r0, c0, grid) && grid[r0 as usize][c0 as usize] == curr && (!in_bounds(r0 + dr, c0 + dc, grid) || grid[(r0 + dr) as usize][(c0 + dc) as usize] != curr) {
            edges.insert((r0, c0, dr, dc));
            r0 -= traverse_r;
            c0 -= traverse_c;
        }

        return 1;
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }
}



fn part2_2(input: &str) -> i32 {
    // use counting corners method with inspiration from online (previous solution was all mine)
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut cell_ids = vec![vec![-1 ; grid[0].len()] ; grid.len()];

    let mut areas = Vec::new();
    let mut side_lengths = Vec::new();

    let mut curr_id = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if cell_ids[r][c] == -1 {
                areas.push(explore3(r as i32, c as i32, grid[r][c], curr_id, &mut cell_ids, &grid));
                side_lengths.push(0);
                curr_id += 1;
            }
        }
    }

    // println!("cell_ids: {:?}", cell_ids);

    for r in 0..=grid.len() {
        for c in 0..=grid[0].len() {
            let vals = vec![
                if in_bounds(r as i32 - 1, c as i32 - 1, &grid) {cell_ids[r-1][c-1]} else {-1}, 
                if in_bounds(r as i32 - 1, c as i32, &grid) {cell_ids[r-1][c]} else {-1}, 
                if in_bounds(r as i32, c as i32, &grid) {cell_ids[r][c]} else {-1}, 
                if in_bounds(r as i32, c as i32 - 1, &grid) {cell_ids[r][c-1]} else {-1}, 
            ];
            for i in 0..4 {
                let curr = vals[i];
                let adj1 = vals[(i+1) % 4];
                let adj2 = if i == 0 {vals[3]} else {vals[i-1]};
                let opp = vals[(i+2) % 4];

                if curr == -1 {
                    continue;
                }

                if curr == adj1 && curr == adj2 && curr != opp {
                    side_lengths[curr as usize] += 1;
                } else if curr != adj1 && curr != adj2 {
                    side_lengths[curr as usize] += 1;
                }
            }
        }
    }

    //println!("areas: {:?}", areas);
    // println!("side_lengths: {:?}", areas);

    let mut res = 0;
    for i in 0..areas.len() {
        res += areas[i] * side_lengths[i]
    }

    return res;


    fn explore3(r: i32, c: i32, curr_char: u8, curr_id: i32, cell_ids: &mut Vec<Vec<i32>>, grid: &Vec<Vec<u8>>) -> i32 {
        if !in_bounds(r, c, grid) || grid[r as usize][c as usize] != curr_char || cell_ids[r as usize][c as usize] != -1 {
            return 0;
        } else {
            cell_ids[r as usize][c as usize] = curr_id;
            let mut area = 1;
            for (dr, dc) in vec![(1,0), (0,1), (-1,0), (0,-1)] {
                area += explore3(r + dr, c + dc, curr_char, curr_id, cell_ids, grid);
            }
            return area;
        }
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid.len() as i32;
    }
}