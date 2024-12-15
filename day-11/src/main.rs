use core::num;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut stones: Vec<i64> = input.split(' ').map(|s| s.parse().unwrap()).collect();

    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            let curr_stone = stones[i];
            if curr_stone == 0 {
                stones[i] = 1;
            } else if curr_stone.to_string().len() & 1 == 0 { // even
                let s = curr_stone.to_string();
                let left = &s[0..(s.len()/2)];
                let right = &s[(s.len()/2)..];
                stones[i] = right.parse().unwrap();
                stones.insert(i, left.parse().unwrap());
                i += 1;
            } else {
                stones[i] = curr_stone * 2024;
            }
            i += 1;
        }
    }

    // println!("{:?}", stones);

    return stones.len() as i32;
}


// DP TO THE RESCUE
fn part2(input: &str) -> i64 {
    let stones: Vec<i64> = input.split(' ').map(|s| s.parse().unwrap()).collect();

    let mut dp = HashMap::new();

    let mut res = 0;
    for val in stones {
        res += explore_stone(val, 0, &mut dp);
    }

    fn explore_stone(val: i64, depth: i32, dp: &mut HashMap<(i64, i32),i64>) -> i64 {
        if depth == 75 {
            return 1;
        }
        if dp.contains_key(&(val, depth)) {
            return dp.get(&(val, depth)).unwrap().clone();
        }
        
        let num_stones;
        if val == 0 {
            num_stones = explore_stone(1, depth+1, dp);
        } else if val.to_string().len() & 1 == 0 { // even
            let s = val.to_string();
            let left = (&s[0..(s.len()/2)]).parse().unwrap();
            let right = (&s[(s.len()/2)..]).parse().unwrap();
            
            num_stones = explore_stone(left, depth+1, dp) + explore_stone(right, depth+1, dp);
        } else {
            num_stones = explore_stone(val * 2024, depth+1, dp)
        }
        
        dp.insert((val, depth), num_stones);
        return num_stones;
    }

    return res;
}