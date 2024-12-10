fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i64 {

    fn check_line(target: i64, operands: &Vec<i32>)-> i64 {

        fn back_track(target: i64, total: i64, index: i32, operands: &Vec<i32>) -> bool {
            if total > target {
                return false;
            }
            if index >= operands.len() as i32 {
                return total == target;
            }

            let curr_val: i64 = operands[index as usize] as i64;
            return 
                back_track(target, total + curr_val, index+1, operands)
                || back_track(target, total * curr_val, index+1, operands);
        }

        if back_track(target, operands[0] as i64, 1, operands) {
            return target;
        } else {
            return 0;
        }
    }

    input
        .lines()
        .fold(0,|sum, line| {
            let x: Vec<&str> = line
                .split(':')
                .collect();
            let target = x[0].parse::<i64>().unwrap();
            let operands = x[1]
                .split(' ')
                .skip_while(|a| a.is_empty())
                .map(|a| 
                    a.parse::<i32>().unwrap()
                ).collect();
            sum + check_line(target, &operands)
        })
}


fn part2(input: &str) -> i64 {

    fn check_line(target: i64, operands: &Vec<i32>)-> i64 {

        fn back_track(target: i64, total: i64, index: i32, operands: &Vec<i32>) -> bool {
            if total > target {
                return false;
            }
            if index >= operands.len() as i32 {
                return total == target;
            }

            let curr_val: i64 = operands[index as usize] as i64;
            return 
                back_track(target, total + curr_val, index+1, operands)
                || back_track(target, total * curr_val, index+1, operands)
                || back_track(target, total * (10 as i64).pow(curr_val.to_string().len() as u32) + curr_val, index+1, operands);
        }

        if back_track(target, operands[0] as i64, 1, operands) {
            return target;
        } else {
            return 0;
        }
    }

    input
        .lines()
        .fold(0,|sum, line| {
            let x: Vec<&str> = line
                .split(':')
                .collect();
            let target = x[0].parse::<i64>().unwrap();
            let operands = x[1]
                .split(' ')
                .skip_while(|a| a.is_empty())
                .map(|a| 
                    a.parse::<i32>().unwrap()
                ).collect();
            sum + check_line(target, &operands)
        })

}
