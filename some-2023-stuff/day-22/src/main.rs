fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

#[derive(Debug, Default)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        return Vec3 {x: x, y: y, z: z};
    }
}

#[derive(Debug, Default)]
struct Brick {
    p1: Vec3,
    p2: Vec3,
}

impl Brick {
    fn new(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> Self {
        return Brick {
            p1: Vec3::new(x1, y1, z1),
            p2: Vec3::new(x2, y2, z2)
        }
    }
}

fn part1(input: &str) -> i32 {
    // 515 is too high
    // 457 is too high
    let (mut max_x, mut max_y) = (0,0);
    let mut bricks = Vec::new();
    for line in input.lines() {
        let coords: Vec<Vec<i32>> = line
            .split("~")
            .map(|end| {
                end.split(",")
                .map(|x| x.parse::<i32>().unwrap()).collect()
            })
            .collect();
        bricks.push(
            Brick::new(coords[0][0], coords[0][1], coords[0][2], coords[1][0], coords[1][1], coords[1][2])
        );
        max_x = max_x.max(coords[0][0]);
        max_x = max_x.max(coords[1][0]);
        max_y = max_y.max(coords[0][1]);
        max_y = max_y.max(coords[1][1]);
    }

    bricks.sort_by_key(|a| a.p1.z.min(a.p2.z));
    // println!("bricks: {:?}", bricks);

    const DEFAULT_ID: usize = 999999;

    let mut heights = vec![ vec![0 ; max_x as usize + 1] ; max_y as usize + 1];
    let mut ids = vec![ vec![DEFAULT_ID ; max_x as usize + 1] ; max_y as usize + 1];
    let mut safe_ids = vec![ 1 ; bricks.len()];

    for (id, brick) in bricks.iter_mut().enumerate() {
        //println!("heights: {:?}", heights);
        // println!("ids: {:?}", ids);

        if brick.p1.x != brick.p2.x || brick.p1.y != brick.p2.y { // brick is horizontal
            let mut max_height_under = 0;
            let mut blocks_under = Vec::new();

            let (mut dx, mut dy) = (brick.p2.x - brick.p1.x, brick.p2.y - brick.p1.y);
            if dx != 0 {
                dx /= dx.abs();
            }
            if dy != 0 {
                dy /= dy.abs();
            }

            let (mut x, mut y) = (brick.p1.x, brick.p1.y);

            loop {
                let h = heights[y as usize][x as usize];
                let block_under = ids[y as usize][x as usize]; 
                if h == max_height_under {
                    if blocks_under.len() == 0 || *blocks_under.last().unwrap() != block_under {
                        blocks_under.push(block_under);
                    }
                } else if h > max_height_under {
                    max_height_under = h;
                    blocks_under.clear();
                    blocks_under.push(block_under);
                }
                if x == brick.p2.x && y == brick.p2.y {
                    break;
                }
                x += dx;
                y += dy;
            }

            let (mut x, mut y) = (brick.p1.x, brick.p1.y);
            loop {
                heights[y as usize][x as usize] = max_height_under + 1;
                ids[y as usize][x as usize] = id;
                if x == brick.p2.x && y == brick.p2.y {
                    break;
                }
                x += dx;
                y += dy;
            }

            // println!("blocks_under: {:?}, ", blocks_under);
            if blocks_under.len() == 1 {
                if blocks_under[0] == DEFAULT_ID {
                    continue;
                }
                safe_ids[blocks_under[0]] = 0;
            }

            brick.p1.z = (max_height_under + 1) as i32;
            brick.p2.z = (max_height_under + 1) as i32;

        } else { // brick is vertical
            let (x,y) = (brick.p1.x, brick.p1.y);
            let block_under = ids[y as usize][x as usize];
            if block_under != DEFAULT_ID {
                safe_ids[block_under] = 0;
            }
            let brick_height = (brick.p1.z - brick.p2.z).abs() + 1;
            heights[y as usize][x as usize] += brick_height as usize;
            ids[y as usize][x as usize] = id;
            brick.p1.z = heights[y as usize][x as usize] as i32;
            brick.p2.z = brick.p1.z - brick_height + 1;
        }
    }
    // println!("heights: {:?}", heights);
    // println!("ids: {:?}", ids);
    //println!("bricks: {:?}", bricks);

    return safe_ids.iter().sum();
}




fn part2(input: &str) -> i32 {

    let (mut max_x, mut max_y) = (0,0);
    let mut bricks = Vec::new();
    for line in input.lines() {
        let coords: Vec<Vec<i32>> = line
            .split("~")
            .map(|end| {
                end.split(",")
                .map(|x| x.parse::<i32>().unwrap()).collect()
            })
            .collect();
        bricks.push(
            Brick::new(coords[0][0], coords[0][1], coords[0][2], coords[1][0], coords[1][1], coords[1][2])
        );
        max_x = max_x.max(coords[0][0]);
        max_x = max_x.max(coords[1][0]);
        max_y = max_y.max(coords[0][1]);
        max_y = max_y.max(coords[1][1]);
    }

    bricks.sort_by_key(|a| a.p1.z.min(a.p2.z));

    const DEFAULT_ID: usize = 999999;

    let mut heights = vec![ vec![0 ; max_x as usize + 1] ; max_y as usize + 1];
    let mut ids = vec![ vec![DEFAULT_ID ; max_x as usize + 1] ; max_y as usize + 1];
    let mut safe_ids = vec![ 1 ; bricks.len()];

    let mut edges = vec![ vec![] ; bricks.len()];
    let mut incoming_count = vec![0 ; bricks.len()];

    for (id, brick) in bricks.iter_mut().enumerate() {

        if brick.p1.x != brick.p2.x || brick.p1.y != brick.p2.y { // brick is horizontal
            let mut max_height_under = 0;
            let mut blocks_under = Vec::new();

            let (mut dx, mut dy) = (brick.p2.x - brick.p1.x, brick.p2.y - brick.p1.y);
            if dx != 0 {
                dx /= dx.abs();
            }
            if dy != 0 {
                dy /= dy.abs();
            }

            let (mut x, mut y) = (brick.p1.x, brick.p1.y);

            loop {
                let h = heights[y as usize][x as usize];
                let block_under = ids[y as usize][x as usize]; 
                if h == max_height_under {
                    if blocks_under.len() == 0 || *blocks_under.last().unwrap() != block_under {
                        blocks_under.push(block_under);
                    }
                } else if h > max_height_under {
                    max_height_under = h;
                    blocks_under.clear();
                    blocks_under.push(block_under);
                }
                if x == brick.p2.x && y == brick.p2.y {
                    break;
                }
                x += dx;
                y += dy;
            }

            let (mut x, mut y) = (brick.p1.x, brick.p1.y);
            loop {
                heights[y as usize][x as usize] = max_height_under + 1;
                ids[y as usize][x as usize] = id;
                if x == brick.p2.x && y == brick.p2.y {
                    break;
                }
                x += dx;
                y += dy;
            }

            if blocks_under.len() == 1 {
                if blocks_under[0] != DEFAULT_ID {
                    safe_ids[blocks_under[0]] = 0;
                }
            }

            incoming_count[id] = blocks_under.len();

            for under in blocks_under {
                if under != DEFAULT_ID {
                    edges[under].push(id);
                }
            }

            brick.p1.z = (max_height_under + 1) as i32;
            brick.p2.z = (max_height_under + 1) as i32;

        } else { // brick is vertical
            let (x,y) = (brick.p1.x, brick.p1.y);

            let block_under = ids[y as usize][x as usize];
            if block_under != DEFAULT_ID {
                edges[block_under].push(id);
                safe_ids[block_under] = 0;
            }

            incoming_count[id] = 1;

            let brick_height = (brick.p1.z - brick.p2.z).abs() + 1;
            heights[y as usize][x as usize] += brick_height as usize;
            ids[y as usize][x as usize] = id;
            brick.p1.z = heights[y as usize][x as usize] as i32;
            brick.p2.z = brick.p1.z - brick_height + 1;
        }
    }

    let mut res = 0;

    // println!("incoming_counts: {:?}", incoming_count);
    // println!("edges: {:?}", edges);

    for id in 0..bricks.len() {
        let mut temp_counts = incoming_count.clone();
        temp_counts[id] = 1;
        let val = explore(id, &edges, &mut temp_counts) - 1;
        // println!("id: {}, val: {}", id, val);
        res += val;
    }

    fn explore(id: usize, edges: &Vec<Vec<usize>>, temp_counts: &mut Vec<usize>) -> i32 {
        temp_counts[id] -= 1;
        if temp_counts[id] > 0 {
            return 0;
        }

        let mut res = 1; // temp_counts[id] = 0
        for nxt in &edges[id] {
            res += explore(*nxt, edges, temp_counts);
        }
        return res;
    }

    return res;
}