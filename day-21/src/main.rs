use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {

    let codes: Vec<&str> = input.lines().collect();

    let dir_pad = vec![ vec![b'x', b'^', b'A'], vec![b'<', b'v', b'>'] ];
    let num_pad = vec![ vec![b'7', b'8', b'9'], vec![b'4', b'5', b'6'], vec![b'1', b'2', b'3'], vec![b'x', b'0', b'A'] ];

    let dir_edges = parse_grid(&dir_pad);
    let num_edges = parse_grid(&num_pad);

    const DIR_PADS: i32 = 2;

    let mut res = 0;
    for c in codes {
        let mut code = c.as_bytes().to_vec();
        code.insert(0, b'A');

        let mut dist = c.len() as i32;
        for i in 1..code.len() {
            let mut start = vec![b'A' ; DIR_PADS as usize];
            let mut end = start.clone();

            start.push(code[i-1]);
            end.push(code[i]);

            dist += explore(start, end, &dir_edges, &num_edges);
        }
        let numeric_part = c[0..3].parse::<i32>().unwrap();
        // println!("dist: {}", dist);
        res += dist * numeric_part;
    }

    return res;


    fn explore(start: Vec<u8>, end: Vec<u8>, dir_edges: &HashMap<u8, Vec<u8>>, num_edges: &HashMap<u8, Vec<u8>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start);

        for step in 0..100000 {
            for _ in 0..queue.len() {
                let curr = queue.pop_front().unwrap();
                if visited.contains(&curr) {
                    continue;
                }
                visited.insert(curr.clone());

                if curr == end {
                    return step;
                }

                // add first dir_pad edges
                for neighbor in &dir_edges[&curr[0]] {
                    if *neighbor != b'x' {
                        let mut next = curr.clone();
                        next[0] = *neighbor;
                        queue.push_back(next);
                    }
                }

                // println!("{:?}", curr.iter().map(|c| *c as char).collect::<Vec<_>>());

                for i in 0..curr.len()-1 {
                    if i >= 1 && curr[i-1] != b'A' {
                        break;
                    }

                    let edges = if i < curr.len()-2 {dir_edges} else {num_edges};

                    match curr[i] {
                        b'^' => {
                        let neighbor = (edges[&curr[i+1]])[0];
                            if neighbor != b'x' {
                                let mut next = curr.clone();
                                next[i+1] = neighbor;
                                queue.push_back(next);
                            }
                        },
                        b'>' => {
                            let neighbor = (edges[&curr[i+1]])[1];
                            if neighbor != b'x' {
                                let mut next = curr.clone();
                                next[i+1] = neighbor;
                                queue.push_back(next);
                            }
                        },
                        b'v' => {
                            let neighbor = (edges[&curr[i+1]])[2];
                            if neighbor != b'x' {
                                let mut next = curr.clone();
                                next[i+1] = neighbor;
                                queue.push_back(next);
                            }
                        },
                        b'<' => {
                            let neighbor = (edges[&curr[i+1]])[3];
                            if neighbor != b'x' {
                                let mut next = curr.clone();
                                next[i+1] = neighbor;
                                queue.push_back(next);
                            }
                        },
                        _ => {} // dir1_pos is NOT on an arrow (so dir2_pos can't be moved)
                    }

                }
            }
        }

        println!("error");

        return -1;
    }

    fn parse_grid(grid: &Vec<Vec<u8>>) -> HashMap<u8, Vec<u8>> {
        let mut edges = HashMap::new();
        let dirs = vec![ (-1,0), (0,1), (1,0), (0,-1) ]; // stored as TOP, RIGHT, DOWN, LEFT
        for r in 0..grid.len() as i32 {
            for c in 0..grid[0].len() as i32 {
                let curr_char = grid[r as usize][c as usize];
                if curr_char == b'x' {
                    continue;
                }
                let mut curr_edges = Vec::new();

                for (dr, dc) in &dirs {
                    if in_bounds(r+dr, c+dc, &grid) {
                        let (r1, c1) = ((r+dr) as usize, (c+dc) as usize);
                        curr_edges.push(grid[r1][c1]);
                    } else {
                        curr_edges.push(b'x');
                    }
                }

                edges.insert(curr_char, curr_edges);
            }
        }
        return edges;
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }
}




fn part2(input: &str) -> i64 {

    const DIR_PADS: i32 = 25;
    const MAX: i64 = 1_000_000_000_000;

    // let dirs = vec![b'^', b'>', b'v', b'<'];
    let extended_dirs = vec![b'^', b'>', b'v', b'<', b'A'];

    let dir_pad = vec![ vec![b'x', b'^', b'A'], vec![b'<', b'v', b'>'] ];
    let num_pad = vec![ vec![b'7', b'8', b'9'], vec![b'4', b'5', b'6'], vec![b'1', b'2', b'3'], vec![b'x', b'0', b'A'] ];

    let dir_edges = get_edges_from_grid(&dir_pad);
    let num_edges = get_edges_from_grid(&num_pad);

    // for (k,v) in &num_edges {
    //     println!("key: {:?}, value: {:?}", (k.0 as char, k.1 as char), v.iter().map(|x| (x.0 as char, x.1 as char)).collect::<Vec<_>>())
    // }

    let mut dir_vertices= Vec::new();
    for curr_char in &extended_dirs {
        for curr_dir in &extended_dirs {
            dir_vertices.push((*curr_char, *curr_dir));
        }
    }

    let num_pad_vals = vec![b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A'];
    let mut num_vertices = Vec::new();
    for curr_char in &num_pad_vals {
        for curr_dir in &extended_dirs {
            num_vertices.push((*curr_char, *curr_dir));
        }
    }

    let mut prev_adj_map: HashMap<(u8, u8, u8, u8), i64> = init_adj_map(&extended_dirs, 1);
    for d1 in &extended_dirs {
        for d2 in &extended_dirs {
            prev_adj_map.insert((*d1, b'A', *d2, b'A'), 0); // rotations initially have no cost
        }
    }


    for _ in 0..DIR_PADS {
        let mut next_adj_map = init_adj_map(&extended_dirs, MAX);

        // set edges to their weights
        for ((curr_char, curr_dir), next) in dir_edges.iter() {
            for (next_char, next_dir) in next {
                // next_adj_map.insert(
                //     (*curr_char, *curr_dir, *next_char, *next_dir), 
                //     *prev_adj_map.get(&(*curr_dir, b'A', *next_dir, b'A')).unwrap()
                // );
                if curr_char == next_char { // rotation
                    next_adj_map.insert(
                        (*curr_char, *curr_dir, *next_char, *next_dir), 
                        *prev_adj_map.get(&(*curr_dir, b'A', *next_dir, b'A')).unwrap()
                    );
                } else { // translation
                    next_adj_map.insert(
                        (*curr_char, *curr_dir, *next_char, *next_dir), 
                        1
                    );
                }
            }
        }

        // set self-edges to 0
        for (curr_char, curr_dir) in &dir_vertices {
            next_adj_map.insert((*curr_char, *curr_dir, *curr_char, *curr_dir), 0);
        }

        // triple V loop
        for (k_chr, k_dir) in &dir_vertices {
            for (i_chr, i_dir) in &dir_vertices {
                for (j_chr, j_dir) in &dir_vertices {

                    // if dist[i][j] > dist[i][k] + dist[k][j] 
                    // dist[i][j] ← dist[i][k] + dist[k][j]
                    let dist_i_j = next_adj_map.get(&(*i_chr, *i_dir, *j_chr, *j_dir)).unwrap();
                    let dist_i_k = next_adj_map.get(&(*i_chr, *i_dir, *k_chr, *k_dir)).unwrap();
                    let dist_k_j = next_adj_map.get(&(*k_chr, *k_dir, *j_chr, *j_dir)).unwrap();
                    
                    if *dist_i_j > *dist_i_k + *dist_k_j {
                        next_adj_map.insert((*i_chr, *i_dir, *j_chr, *j_dir), dist_i_k + dist_k_j);
                    }
                }
            }
        }

        prev_adj_map = next_adj_map;
    }


    let codes: Vec<_> = input.lines().collect();
    let mut res = 0;

    for c in codes {
        let mut code = c.as_bytes().to_vec();
        code.insert(0, b'A');

        let mut next_adj_map = init_adj_map2(&num_pad_vals, &extended_dirs, MAX);

        // set edges to their weights
        for ((curr_char, curr_dir), next) in num_edges.iter() {
            for (next_char, next_dir) in next {
                if curr_char == next_char { // rotation
                    next_adj_map.insert(
                        (*curr_char, *curr_dir, *next_char, *next_dir), 
                        *prev_adj_map.get(&(*curr_dir, b'A', *next_dir, b'A')).unwrap()
                    );
                } else { // translation
                    next_adj_map.insert(
                        (*curr_char, *curr_dir, *next_char, *next_dir), 
                        1
                    );
                }
            }
        }

        // set self-edges to 0
        for (curr_char, curr_dir) in &num_vertices {
            next_adj_map.insert((*curr_char, *curr_dir, *curr_char, *curr_dir), 0);
        }

        // triple V loop
        for (k_chr, k_dir) in &num_vertices {
            for (i_chr, i_dir) in &num_vertices {
                for (j_chr, j_dir) in &num_vertices {

                    // if dist[i][j] > dist[i][k] + dist[k][j] 
                    // dist[i][j] ← dist[i][k] + dist[k][j]
                    let dist_i_j = next_adj_map.get(&(*i_chr, *i_dir, *j_chr, *j_dir)).unwrap();
                    let dist_i_k = next_adj_map.get(&(*i_chr, *i_dir, *k_chr, *k_dir)).unwrap();
                    let dist_k_j = next_adj_map.get(&(*k_chr, *k_dir, *j_chr, *j_dir)).unwrap();
                    
                    if *dist_i_j > *dist_i_k + *dist_k_j {
                        next_adj_map.insert((*i_chr, *i_dir, *j_chr, *j_dir), dist_i_k + dist_k_j);
                    }
                }
            }
        }

        // use next_adj_map to find paths
        let numeric_part = c[0..3].parse::<i64>().unwrap();

        let mut dist = c.len() as i64; // initialize to length of code since each button needs a press
        for i in 1..code.len() {
            dist += next_adj_map.get(&(code[i-1], b'A', code[i], b'A')).unwrap();
        }
        println!("dist: {dist}");
        
        res += dist * numeric_part;

        // println!("{:?}", next_adj_map);

    }

    return res;

    fn init_adj_map(dirs: &Vec<u8>, default: i64) -> HashMap<(u8, u8, u8, u8), i64> {
        let mut map: HashMap<(u8, u8, u8, u8), i64> = HashMap::new();
        for p1 in dirs {
            for d1 in dirs {
                for p2 in dirs {
                    for d2 in dirs {
                        map.insert((*p1,*d1,*p2,*d2), default);
                    }
                }
            }
        }
        return map;
    }

    fn init_adj_map2(chars: &Vec<u8>, dirs: &Vec<u8>, default: i64) -> HashMap<(u8, u8, u8, u8), i64> {
        let mut map: HashMap<(u8, u8, u8, u8), i64> = HashMap::new();
        for p1 in chars {
            for d1 in dirs {
                for p2 in chars {
                    for d2 in dirs {
                        map.insert((*p1,*d1,*p2,*d2), default);
                    }
                }
            }
        }
        return map;
    }


    fn get_edges_from_grid(grid: &Vec<Vec<u8>>) -> HashMap<(u8, u8), Vec<(u8, u8)>> {
        let mut edges = HashMap::new();
        let dirs = vec![ (-1,0, b'^'), (0,1, b'>'), (1,0, b'v'), (0,-1, b'<'), (0,0, b'A') ]; // stored as TOP, RIGHT, DOWN, LEFT
        for r in 0..grid.len() as i32 {
            for c in 0..grid[0].len() as i32 {
                let curr_char = grid[r as usize][c as usize];
                if curr_char == b'x' {
                    continue;
                }

                for (dr, dc, curr_dir) in &dirs {
                    let mut curr_edges = Vec::new();

                    if in_bounds(r+dr, c+dc, &grid) && *curr_dir != b'A' {
                        let (r1, c1) = ((r+dr) as usize, (c+dc) as usize);
                        let neighbor = grid[r1][c1];
                        if neighbor != b'x' {
                            curr_edges.push((grid[r1][c1], *curr_dir));
                        }
                    }

                    for (_,_, next_dir) in &dirs {
                        curr_edges.push((curr_char, *next_dir));
                    }

                    edges.insert((curr_char, *curr_dir), curr_edges);
                }

            }
        }
        return edges;
    }

    fn in_bounds(r: i32, c: i32, grid: &Vec<Vec<u8>>) -> bool {
        return r >= 0 && c >= 0 && r < grid.len() as i32 && c < grid[0].len() as i32;
    }
}