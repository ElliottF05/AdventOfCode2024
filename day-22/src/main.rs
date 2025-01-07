use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut secret_nums: Vec<u64> = input.lines().map(|line| line.parse::<u64>().unwrap()).collect();

    const STEPS: i32 = 2000;

    for _ in 0..STEPS {
        for secret_num in secret_nums.iter_mut() {

            let other = *secret_num * 64;
            mix_number(secret_num, other);
            prune_number(secret_num);

            let other = *secret_num / 32;
            mix_number(secret_num, other);
            prune_number(secret_num);

            let other = *secret_num * 2048;
            mix_number(secret_num, other);
            prune_number(secret_num);
        }
    }

    // println!("{:?}", secret_nums);

    return secret_nums.iter().sum();

    fn mix_number(secret_num: &mut u64, other: u64) {
        *secret_num ^= other;
    }

    fn prune_number(secret_num: &mut u64) {
        const MOD: u64 = 16777216; // = 2^24
        *secret_num %= MOD;
    }
}

fn part2(input: &str) -> i32 {
    let secret_nums: Vec<u64> = input.lines().map(|line| line.parse::<u64>().unwrap()).collect();

    const STEPS: i32 = 2000;

    let mut sequence_profit = HashMap::new();
    sequence_profit.reserve(STEPS as usize * secret_nums.len());

    for mut secret_num in secret_nums {

        let mut prices = Vec::new();
        let mut sequences_seen = HashSet::new();

        prices.reserve(STEPS as usize);
        sequences_seen.reserve(STEPS as usize);

        for _ in 0..STEPS {

            let other = secret_num * 64;
            mix_number(&mut secret_num, other);
            prune_number(&mut secret_num);

            let other = secret_num / 32;
            mix_number(&mut secret_num, other);
            prune_number(&mut secret_num);

            let other = secret_num * 2048;
            mix_number(&mut secret_num, other);
            prune_number(&mut secret_num);

            prices.push((secret_num.to_string().bytes().last().unwrap() - b'0') as i32);
        }

        for window in prices.windows(5) {
            let change_sequence: Vec<_> = window.windows(2).map(|p| p[1]-p[0]).collect();

            if !sequences_seen.insert(change_sequence.clone()) { // insert returns false if value was already there
                continue;
            }

            let curr_price = window[4];
            sequence_profit.entry(change_sequence).and_modify(|profit| *profit += curr_price).or_insert(curr_price);
            
        }
    }

    return *sequence_profit.values().max().unwrap();

    fn mix_number(secret_num: &mut u64, other: u64) {
        *secret_num ^= other;
    }

    fn prune_number(secret_num: &mut u64) {
        const MOD: u64 = 16777216; // = 2^24
        *secret_num %= MOD;
    }
}