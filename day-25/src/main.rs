fn main() {
    let input = include_str!("input.txt");

    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let schematics: Vec<Vec<Vec<u8>>> = input
        .split("\n\n")
        .map(|schematic_str| {
            schematic_str
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    const WIDTH: usize = 5;
    const HEIGHT: usize = 7;

    for s in schematics {
        let mut heights = Vec::new();
        for x in 0..WIDTH {
            let mut height = 0;
            for y in 1..HEIGHT-1 {
                if s[y][x] == b'#' {
                    height += 1;
                }
            }
            heights.push(height);
        }
        if s[0][0] == b'#' { // lock
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    //println!("{:?}", locks);
    // println!("{:?}", keys);


    let mut res = 0;
    for lock in &locks {
        for key in &keys {
            let mut valid_combo = true;
            for i in 0..lock.len() {
                if lock[i] + key[i] > HEIGHT-2 {
                    valid_combo = false;
                    break;
                }
            }
            if valid_combo {
                res += 1;
            }
        }
    }

    return res;
}

fn part2(input: &str) -> i32 {
    return -1;
}