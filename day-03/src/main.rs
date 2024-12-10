use regex::Regex;
fn main() {
    let input1 = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input1));
    println!("PART 2 ANSWER: {}", part2(input1));
}

fn part1(input: &str) -> i32 {
    let re_main = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let re_num = Regex::new(r"[0-9]{1,3}").unwrap();

    let mut res = 0;
    for pat in re_main.find_iter(input) {
        let s = pat.as_str();

        let mut product = 1;
        for pat2 in re_num.find_iter(s) {
            product *= pat2.as_str().parse::<i32>().unwrap();
        }
        res += product
    }

    // OH MY GOD I LOVE REGEX (first time using it)

    return res;
}


fn part2(input: &str)-> i32 {
    let re_main = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let re_num = Regex::new(r"[0-9]{1,3}").unwrap();

    let mut res = 0;
    let mut mul_enabled = true;
    for pat1 in re_main.find_iter(input) {
        let s = pat1.as_str();
        match s {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            _ => {
                if mul_enabled {
                    let mut product = 1;
                    for pat2 in re_num.find_iter(s) {
                        product *= pat2.as_str().parse::<i32>().unwrap();
                    }
                    res += product
                }
            }
        }
    }

    return res;
}
