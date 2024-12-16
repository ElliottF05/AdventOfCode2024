use std::{collections::{BinaryHeap, HashMap, HashSet, VecDeque}, time::Instant};

fn main() {
    let input = include_str!("input.txt");

    let t1 = Instant::now();
    println!("PART 1 ANSWER: {}", part1(input));
    println!("part 1 took {:?} time", t1.elapsed());

    let t2 = Instant::now();
    println!("PART 2 ANSWER: {}", part2(input));
    println!("part 2 took {:?} time", t2.elapsed());
}

fn part1(input: &str) -> i32 {

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut pq: BinaryHeap<(i32, i32, i32, i32, i32)> = BinaryHeap::new();
    let mut dist: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'S' {
                pq.push((0,r as i32,c as i32,0,1));
                // dist.insert((r as i32,c as i32,0,1), 0);
            }
        }
    }

    // println!("{:?}", pq);
    // println!("{:?}", dist);

    loop {
        let (neg_d,r,c,dr,dc) = pq.pop().unwrap();
        let d = -neg_d;
        let curr_char = grid[r as usize][c as usize];

        // println!("{d},{r},{c},{dr},{dc}, {}", curr_char as char);

        if curr_char == b'E' {
            return d;
        } else if curr_char == b'#' || *(dist.get(&(r,c,dr,dc)).unwrap_or(&999999)) <= d {
            continue;
        } else {
            dist.insert((r,c,dr,dc), d);
            pq.push((-(d+1), r+dr, c+dc, dr, dc));
            pq.push((-(d+1000), r, c, dc, dr));
            pq.push((-(d+1000), r, c, -dc, -dr));
        }
    }

    // return -1;
}




#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
struct PathPoint {
    r: i32,
    c: i32,
    dr: i32,
    dc: i32
}

impl PathPoint {
    fn new(r: i32, c: i32, dr: i32, dc: i32) -> Self {
        return PathPoint {r: r, c: c, dr: dr, dc: dc};
    }
}

fn part2(input: &str) -> i32 {

    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut pq: BinaryHeap<(i32, PathPoint, Option<PathPoint>)> = BinaryHeap::new();
    let mut dist: HashMap<PathPoint, i32> = HashMap::new();
    let mut prev: HashMap<PathPoint, Vec<Option<PathPoint>>> = HashMap::new();

    let (mut end_r, mut end_c) = (0,0);
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'S' {
                pq.push((0, PathPoint::new(r as i32, c as i32, 0, 1), None));
            }
            if grid[r][c] == b'E' {
                end_r = r as i32;
                end_c = c as i32;
            }
        }
    }

    // println!("{:?}", pq);
    // println!("{:?}", dist);
    
    let mut max_dist = 999999;

    loop {
        let (neg_d, path_point, prev_point) = pq.pop().unwrap();
        let (r,c,dr,dc) = (path_point.r, path_point.c, path_point.dr, path_point.dc);
        let d = -neg_d;
        if d > max_dist {
            break;
        }
        let curr_char = grid[r as usize][c as usize];
        let curr_dist = *dist.get(&path_point).unwrap_or(&999999);

        // println!("{d},{r},{c},{dr},{dc}, {}", curr_char as char);

        if curr_char == b'E' {
            max_dist = d;
            prev.entry(path_point).or_default().push(prev_point);
        } else if curr_char == b'#' || curr_dist < d {
            continue;
        } else if curr_dist == d {
            prev.entry(path_point).or_default().push(prev_point);
        } else {
            prev.entry(path_point.clone()).or_default().clear();
            prev.get_mut(&path_point).unwrap().push(prev_point);
            dist.insert(path_point.clone(), d);
            pq.push((-(d+1), PathPoint::new(r+dr, c+dc, dr, dc), Some(path_point.clone())));
            pq.push((-(d+1000), PathPoint::new(r,c,dc,dr), Some(path_point.clone()))); 
            pq.push((-(d+1000), PathPoint::new(r,c,-dc,-dr), Some(path_point))); 
        }
    }

    let mut places_to_sit = HashSet::new();
    
    let mut queue = VecDeque::new();
    for (dr,dc) in vec![(0,1), (0,-1), (1,0), (-1,0)] {
        queue.push_back(PathPoint::new(end_r, end_c, dr, dc));
    }

    while !queue.is_empty() {
        let path_point = queue.pop_front().unwrap();
        let (r,c,_,_) = (path_point.r, path_point.c, path_point.dr, path_point.dc);

        places_to_sit.insert((r,c));

        for prev_point in prev.entry(path_point).or_default() {
            if let Some(prev) = prev_point {
                queue.push_back(prev.clone());
            }
        }
    }

    return places_to_sit.len() as i32;
}