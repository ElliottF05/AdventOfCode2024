use regex::Regex;

// Luckily none of the systems had det == 0, this would've added some more cases to consider

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"[0-9]+").unwrap();
    
    let machines: Vec<Vec<i32>> = input
        .split("\n\n")
        .into_iter()
        .map(|group| {
            re.captures_iter(group)
                .into_iter()
                .map(|curr_match| {
                    curr_match.get(0).unwrap().as_str().parse::<i32>().unwrap()
                })
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut res = 0;
    
    for machine in machines {
        let [a, c, b, d, b1, b2] = machine[0..6].try_into().unwrap();
        // println!("\n{}, {}, {}, {}, {}, {}", a,b,c,d,b1,b2);
        let det = a*d - b*c;
        // println!("det: {}", det);

        let x1_times_det = b1 * d + b2 * (-b);
        let x2_times_det = b1 * (-c) + b2 * a;

        // println!("x1_times_det: {}, x2_times_det: {}", x1_times_det, x2_times_det);

        if det == 0 {
            println!("det = 0");
        } else if x1_times_det % det == 0 && x2_times_det % det == 0 {
            //println!("one integer solution");
            let button_1_presses = x1_times_det / det;
            let button_2_presses = x2_times_det / det;
            if button_1_presses <= 100 && button_2_presses <= 100 {
                res += 3 * (x1_times_det / det) + (x2_times_det / det);
            }
        } else {
            //println!("solution is not integer");
        }
    }

    return res;
}



fn part2(input: &str) -> i64 {
    let re = Regex::new(r"[0-9]+").unwrap();
    
    let machines: Vec<Vec<i64>> = input
        .split("\n\n")
        .into_iter()
        .map(|group| {
            re.captures_iter(group)
                .into_iter()
                .map(|curr_match| {
                    curr_match.get(0).unwrap().as_str().parse::<i64>().unwrap()
                })
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut res = 0;
    
    for machine in machines {
        let [a, c, b, d, mut b1, mut b2] = machine[0..6].try_into().unwrap();
        const ADD: i64 = 10000000000000;
        b1 += ADD;
        b2 += ADD;
        // println!("\n{}, {}, {}, {}, {}, {}", a,b,c,d,b1,b2);
        let det = a*d - b*c;
        // println!("det: {}", det);

        let x1_times_det = b1 * d + b2 * (-b);
        let x2_times_det = b1 * (-c) + b2 * a;

        // println!("x1_times_det: {}, x2_times_det: {}", x1_times_det, x2_times_det);

        if det == 0 {
            println!("det = 0");
        } else if x1_times_det % det == 0 && x2_times_det % det == 0 {
            //println!("one integer solution");
            res += 3 * (x1_times_det / det) + (x2_times_det / det);
        } else {
            //println!("solution is not integer");
        }
    }

    return res;
}