use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let input1 = include_str!("input1.txt");
    println!("PART 1 RESULT: {}", part1(input1));

    let input2 = include_str!("input1.txt");
    println!("PART 2 RESULT: {}", part2(input2));
    
}

fn part1(input: &str) -> i32 {
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|a| a.parse::<i32>().unwrap())
        .collect();

    let mut left_numbers  = Vec::new();
    let mut right_numbers = Vec::new();

    for (index, value) in numbers.into_iter().enumerate() {
        if index & 1 != 0 { // odd
            right_numbers.push(value);
        } else {
            left_numbers.push(value);
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    // println!("{:?}", left_numbers);
    // println!("{:?}", right_numbers);

    let mut res = 0;
    for i in 0..left_numbers.len() {
        let dif = (left_numbers.get(i).unwrap() - right_numbers.get(i).unwrap()).abs();
        res += dif;
    }

    return res;
}

fn part2(input: &str) -> i32 {

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|a| a.parse::<i32>().unwrap())
        .collect();

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for (index, value) in numbers.into_iter().enumerate() {
        if index & 1 != 0 { // odd
            right_numbers.push(value);
        } else {
            left_numbers.push(value);
        }
    }

    let mut map = HashMap::new();

    for num in right_numbers {
        if let Some(count) = map.get(&num) {
            map.insert(num, count + 1);
        } else {
            map.insert(num, 1);
        }
    }

    let mut res = 0;
    for num in left_numbers {
        if let Some(count) = map.get(&num) {
            res += num * count;
        }
    }

    return res;
}
