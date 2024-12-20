fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let patterns: Vec<&str> = input.lines().next().unwrap().split(", ").collect();
    let designs: Vec<&str> = input.lines().skip(2).collect();

    // println!("{:?}", patterns);
    // println!("{:?}", designs);

    let mut res = 0;
    for d in designs {
        if backtrack(d, 0, &patterns) {
            // println!("design {} can be made\n", d);
            res += 1;
        }
    }

    return res;


    fn backtrack(design: &str, i: usize, patterns: &Vec<&str>) -> bool {
        if i == design.len() {
            return true;
        }
        if i > design.len() {
            return false;
        }
    ;
        for p in patterns {
            if i+p.len() <= design.len() && **p == design[i..i+p.len()] {
                if backtrack(design, i+p.len(), patterns) {
                    return true;
                }
            }
        }
    
        return false;
    }
}



fn part2(input: &str) -> i64 {
    let patterns: Vec<&str> = input.lines().next().unwrap().split(", ").collect();
    let designs: Vec<&str> = input.lines().skip(2).collect();

    // println!("{:?}", patterns);
    // println!("{:?}", designs);

    let mut res: i64 = 0;
    for d in designs {
        res += count_ways(d, &patterns);
    }

    return res;
    

    fn count_ways(design: &str, patterns: &Vec<&str>) -> i64 {
        let mut dp = vec![ 0 ; design.len() ];

        for i in 0..dp.len() as i32 {
            for p in patterns {
                let j = i - p.len() as i32 + 1;
                if j >= 0 && **p == design[(j as usize)..=(i as usize)] {
                    if j-1 >= 0 {
                        dp[i as usize] += dp[(j-1) as usize];
                    } else {
                        dp[i as usize] += 1;
                    }

                }
            }
        }

        return *dp.last().unwrap();
    }
}