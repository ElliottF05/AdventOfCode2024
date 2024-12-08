fn main() {
    let input1 = include_str!("input.txt");
    println!("PART 1 RESULT: {}", part1(input1));

    let input2 = include_str!("input.txt");
    println!("PART 2 RESULT: {}", part2(input2));
}

fn part1(input: &str) -> i32 {

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| 
            line.split_whitespace()
                .map(|a| a.parse::<i32>().unwrap())
                .collect())
        .collect();

    // println!("{:?}", reports);

    fn is_safe(report: &Vec<i32>) -> bool {
        if report.get(0) == report.get(1) {
            return false;
        }

        let increasing = report[1] > report[0];

        return report.windows(2).all(|pair| {
            let dif = pair[1] - pair[0];
            (dif > 0) == increasing && (1..=3).contains(&dif.abs())
        });
    }

    return reports
        .iter()
        .filter(|report| is_safe(report))
        .count() as i32;
        
}


fn part2(input: &str) -> i32 {

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| 
            line.split_whitespace()
                .map(|a| a.parse::<i32>().unwrap())
                .collect())
        .collect();

    fn try_report(report: Vec<i32>) -> bool {
        report
            .iter()
            .enumerate()
            .any(|(index, _)| {
                let mut modified = report.clone();
                modified.remove(index);
                is_safe(&modified)
            })
    }

            
    fn is_safe(report: &Vec<i32>) -> bool {
        if report.get(0) == report.get(1) {
            return false;
        }

        let increasing = report[1] > report[0];

        return report.windows(2).all(|pair| {
            let dif = pair[1] - pair[0];
            (dif > 0) == increasing && (1..=3).contains(&dif.abs())
        });
    }

    return reports
        .into_iter()
        .filter(|report| try_report(report.clone()))
        .count() as i32;
    
}