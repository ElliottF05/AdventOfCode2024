use std::{collections::BinaryHeap, cmp::Reverse};

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

#[derive(Default, Clone, Debug)]
struct Bit {
    id: usize,
    is_negated: bool
}

impl Bit {
    fn new(id: usize, is_negated: bool) -> Bit {
        return Bit {
            id: id,
            is_negated: is_negated
        }
    }
}

fn part2(input: &str) -> u64 {
    // answer >= 950 billion
    // answer is 190,384,615,275,535

    // instructions: 2,4,1,2,7,5,4,5,0,3,1,7,5,5,3,0

    // de-assembling:
    // 2,4: b = a % 8
    // 1,2: b = b ^ 2
    // 7,5: c = a >> b
    // 4,5: b = b ^ c
    // 0,3: a = a >> 3
    // 1,7: b = b ^ 7
    // 5,5: output b % 5
    // 3,0: jump back to beginning if a != 0

    // in other words:
    // while a != 0 {
    //  b = a % 8
    //  b = b ^ 2
    //  c = a >> b
    //  b = b ^ c
    //  a = a >> 3
    //  b = b ^ 7
    //  output b % 5

    // key observations:
    // 1) the program is a simple while loop
    // 2) the values of b and c in the current iteration are INDEPENDENT of the prev iteration
    //      -> they are defined newly in each loop based on a
    // 3) a is shifted to the right by 3 for each iteration
    
    // So, if we consider octal (0-7, 3 bits) digits and we know which octal digit(s) cause the 
    // program to output the last instruction, then these are valid "candidates" and one of these 
    // must be the beginning of the initial value of a. This is because in the last iteration,
    // this octal digit will be a itself, and must output the final instruction

    // For example, the last instruction is "0". If octal digits 2 and 4 both result in an output of 0,
    // then the value of a in the last iteration must be 2 or 4. So, in the previous iteration, a will be
    // either 00002x or 00004x (x is an unknown digit).

    // This process gives us a pool of candidate numbers, which can be shifted left and appended by 
    // 0-7 to test the previous iteration.

    let re = Regex::new(r"[0-9]+").unwrap();

    let matches: Vec<u64> = re
        .captures_iter(input)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<u64>().unwrap())
        .collect();

    let instructions: Vec<u64> = matches[3..].try_into().unwrap();

    let mut candidates = BinaryHeap::new();
    candidates.push(Reverse(0));

    loop {
        let mut new_candidates = BinaryHeap::new();
        for candidate in &candidates {
            for i in 0..8 {
                let a = candidate.0 * 8 + i;
                let output = run_program(a, 0, 0, &instructions);
                let n = output.len();
                if output == instructions.iter().rev().take(n).cloned().rev().collect::<Vec<_>>() {
                    // candidate is valid
                    if n == instructions.len() {
                        return a;
                    }
                    new_candidates.push(Reverse(a));
                }

                }
            }
            candidates = new_candidates;
        }
}

fn run_program(mut a: u64, mut b: u64, mut c: u64, instructions: &Vec<u64>) -> Vec<u64> {
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
                a = a >> combo_operand;
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
            },
            6 => {
                b = a >> combo_operand;
            },
            7 => {
                c = a >> combo_operand;
            },
            _ => panic!("invalid opcode")
        }

        if !jumped {
            pc += 2;
        }
    }

    return output;
}