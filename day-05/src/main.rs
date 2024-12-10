use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let parsed: Vec<&str> = input.split("\n\n").collect();
    let rules: HashSet<(i32, i32)> = HashSet::from_iter(parsed[0]
        .lines()
        .map(|line| {
            let x: Vec<&str> = line.split("|").collect();
            (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap())
            })
        );

    let updates: Vec<Vec<i32>> = parsed[1]
        .lines()
        .map(|line| 
            line.split(",")
                .map(|val| 
                    val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
        .collect();

    let mut res = 0;
    for update in updates {
        let mut valid_update = true;
        for i in 0..update.len() {
            for j in i+1..update.len() {
                if rules.contains(&(update[j], update[i])) {
                    valid_update = false;
                    break;
                }
            }
        }
        if valid_update {
            res += update[update.len() / 2];
        }
    }

    return res;
}


fn part2(input: &str) -> i32 {
    let parsed: Vec<&str> = input.split("\n\n").collect();
    let rules: HashSet<(i32, i32)> = HashSet::from_iter(parsed[0]
        .lines()
        .map(|line| {
            let x: Vec<&str> = line.split("|").collect();
            (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap())
            })
        );

    let updates: Vec<Vec<i32>> = parsed[1]
        .lines()
        .map(|line| 
            line.split(",")
                .map(|val| 
                    val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
        .collect();

    let mut res = 0;
    for mut update in updates {
        let mut valid_update = true;
        for i in 0..update.len() {
            for j in i+1..update.len() {
                if rules.contains(&(update[j], update[i])) {
                    valid_update = false;
                    break;
                }
            }
        }
        if !valid_update {
            make_valid(&mut update, &rules);
            res += update[update.len() / 2];
        }
    }

    fn make_valid(update: &mut Vec<i32>, rules: &HashSet<(i32, i32)>) {
        let mut invalid = true;
        while invalid {
            invalid = false;
            for i in 0..update.len() {
                for j in i+1..update.len() {
                    if rules.contains(&(update[j], update[i])) {
                        invalid = true;
                        (update[j], update[i]) = (update[i], update[j]);
                    }
                }
            }
        }
    }

    return res;
}

