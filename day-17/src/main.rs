use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> String {

    let re = Regex::new(r"[0-9]+").unwrap();

    let matches: Vec<u32> = re
        .captures_iter(input)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<u32>().unwrap())
        .collect();

    let mut a_reg = matches[0];
    let mut b_reg = matches[1];
    let mut c_reg = matches[2];
    let mut pc = 0;

    let instructions: Vec<u32> = matches[3..].try_into().unwrap();
    let mut output = Vec::new();

    while pc < instructions.len() {
        let opcode = instructions[pc];
        let lit_operand = instructions[pc+1];

        let combo_operand = match lit_operand {
            4 => a_reg,
            5 => b_reg,
            6 => c_reg,
            _ => lit_operand,
        };

        let mut jumped = false;

        match opcode {
            0 => {
                a_reg = a_reg / ((2 as u32).pow(combo_operand));
            },
            1 => {
                b_reg = b_reg ^ lit_operand;
            },
            2 => {
                b_reg = combo_operand % 8;
            },
            3 => {
                if a_reg != 0 {
                    pc = lit_operand as usize;
                    jumped = true;
                }
            }
            4 => {
                b_reg = b_reg ^ c_reg;
            },
            5 => {
                output.push(combo_operand % 8);
            },
            6 => {
                b_reg = a_reg / ((2 as u32).pow(combo_operand));
            },
            7 => {
                c_reg = a_reg / ((2 as u32).pow(combo_operand));
            },
            _ => panic!("invalid opcode")
        }

        if !jumped {
            pc += 2;
        }
    }

    return output.iter().map(|val| val.to_string()).collect::<Vec<_>>().join(",")
}




fn part2(input: &str) -> u64 {
    // answer >= 950 billion

    let re = Regex::new(r"[0-9]+").unwrap();

    let matches: Vec<u64> = re
        .captures_iter(input)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<u64>().unwrap())
        .collect();

    let instructions: Vec<u64> = matches[3..].try_into().unwrap();

    let mut a = 80_000_000_000;
    loop {
        if a % 10_000_000_000 == 0 {
            println!("{} billion", a / 1_000_000_000);
        }
        if run_program(a, 0, 0, &instructions) {
            return a;
        }
        a += 1;
    }
}

fn run_program(mut a: u64, mut b: u64, mut c: u64, instructions: &Vec<u64>) -> bool {
    let mut pc = 0;
    let mut output = Vec::new();

    while pc+1 < instructions.len() {
        let opcode = instructions[pc];
        let lit_operand = instructions[pc+1];

        let combo_operand = match lit_operand {
            4 => a,
            5 => b,
            6 => c,
            _ => lit_operand,
        };

        let mut jumped = false;

        match opcode {
            0 => {
                a = a / ((2 as u64).pow(combo_operand as u32));
            },
            1 => {
                b = b ^ lit_operand;
            },
            2 => {
                b = combo_operand % 8;
            },
            3 => {
                if a != 0 {
                    pc = lit_operand as usize;
                    jumped = true;
                }
            }
            4 => {
                b = b ^ c;
            },
            5 => {
                output.push(combo_operand % 8);
                let i = output.len() - 1;
                if i >= instructions.len() || output[i] != instructions[i] {
                    return false;
                }
            },
            6 => {
                b = a / ((2 as u64).pow(combo_operand as u32));
            },
            7 => {
                c = a / ((2 as u64).pow(combo_operand as u32));
            },
            _ => panic!("invalid opcode")
        }

        if !jumped {
            pc += 2;
        }
    }

    return *instructions == output;
}