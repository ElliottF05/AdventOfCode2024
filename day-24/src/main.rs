use core::panic;
use std::{collections::{HashMap, VecDeque}, env};

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let input = include_str!("input.txt");

    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CompType {
    Wire,
    AND,
    OR,
    XOR,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Component {
    id: String,
    comp_type: CompType,
    input_vals: Vec<bool>,
    input_ids: Vec<String>,
    output_ids: Vec<String>
}

impl Component {
    fn new(id: String, comp_type: CompType, input_vals: Vec<bool>, input_ids: Vec<String>, output_ids: Vec<String>) -> Self {
        return Component {id: id, comp_type: comp_type, input_vals: input_vals, input_ids: input_ids, output_ids: output_ids};
    }
    fn add_gate(gate_id: &str, comp_map: &mut HashMap<String, Component>) {
        let parts: Vec<_> = gate_id.split(" ").collect();
        let gate_type = match parts[1] {
            "AND" => CompType::AND,
            "OR" => CompType::OR,
            "XOR" => CompType::XOR,
            _ => panic!("invalid gate type"),
        };
        let gate = Self::new(gate_id.to_owned(), gate_type, Vec::new(), vec![parts[0].to_owned(), parts[2].to_owned()], vec![parts[4].to_owned()]);
        comp_map.insert(gate.id.to_owned(), gate);

        Self::add_output_to_wire(parts[0], gate_id, comp_map);
        Self::add_output_to_wire(parts[2], gate_id, comp_map);

        if !comp_map.contains_key(parts[4]) {
            comp_map.insert(
                parts[4].to_owned(),
                Component::new(
                    parts[4].to_owned(),
                    CompType::Wire,
                    Vec::new(),
                    Vec::new(),
                    Vec::new()
                )
            );
        }

    }
    fn add_output_to_wire(wire_id: &str, output: &str, comp_map: &mut HashMap<String, Component>) {
        let wire = comp_map
            .entry(wire_id.to_owned())
            .or_insert(
                Self::new(wire_id.to_owned(), CompType::Wire, Vec::new(), Vec::new(), Vec::new())
            );
        wire.output_ids.push(output.to_owned());
    }
    fn get_output_val(&self) -> bool {
        // println!("{}, {:?}", self.id, self.input_vals);
        match self.comp_type {
            CompType::Wire => {
                if self.input_vals.len() != 1 {
                    panic!();
                } else {
                    return self.input_vals[0];
                }
            },
            CompType::AND => {
                if self.input_vals.len() != 2 {
                    panic!();
                } else {
                    return self.input_vals[0] && self.input_vals[1];
                }
            },
            CompType::OR => {
                if self.input_vals.len() != 2 {
                    panic!();
                } else {
                    return self.input_vals[0] || self.input_vals[1];
                }
            },
            CompType::XOR => {
                if self.input_vals.len() != 2 {
                    panic!();
                } else {
                    return self.input_vals[0] != self.input_vals[1];
                }
            },
        }
    }
    fn ready_to_output(&self) -> bool {
        match self.comp_type {
            CompType::Wire => self.input_vals.len() == 1,
            _ => self.input_vals.len() == 2
        }
    }
}


fn part1(input: &str) -> u64 {
    // ans is 58367545758258
    let input_sections: Vec<_> = input.split("\n\n").collect();
    let initial_wires: Vec<_> = input_sections[0].lines().collect();
    let gate_strs: Vec<_> = input_sections[1].lines().collect();

    let mut comp_map = HashMap::new();

    for gate_str in gate_strs {
        Component::add_gate(gate_str, &mut comp_map);
    }

    let mut queue = VecDeque::new();

    for initial_wire in initial_wires {
        let parts: Vec<_> = initial_wire.split(": ").collect();
        let wire_id = parts[0];
        let truth_val = parts[1] == "1";
        comp_map.get_mut(wire_id).unwrap().input_vals.push(truth_val);
        queue.push_back(wire_id.to_owned());
    }

    // println!("{:?}", comp_map);

    while !queue.is_empty() {
        let curr_id = queue.pop_front().unwrap();
        let curr_component;
        {
            curr_component = comp_map.get(&curr_id).unwrap().clone()
        }

        let output_val = curr_component.get_output_val();
        for next_id in &curr_component.output_ids {
            let next_component = comp_map.get_mut(next_id).unwrap();
            next_component.input_vals.push(output_val);
            if next_component.ready_to_output() {
                queue.push_back(next_component.id.clone());
            }
        }
    }

    const MAX_BIT: i32 = 45;
    let mut res: u64 = 0;
    let mut pow: u64 = 1;
    for i in 0..=MAX_BIT {
        let wire = format!("z{:0>2}", i);
        if comp_map.get(&wire).unwrap().input_vals[0] {
            res += pow;
        }
        pow *= 2;
    }

    return res;
}


fn part2(input: &str) -> String {
    // finished part 2!!!
    // Had to assume that gates were setup optimally for full adders
    // Assumed that mismatched next gates were never of the same type

    // To improve code: work with id's instead of clones to Components,
    // to better deal with mutability. Make helper function that operate on the ids
    // and do the functions I need
    
    let input_sections: Vec<_> = input.split("\n\n").collect();
    let gate_strs: Vec<_> = input_sections[1].lines().collect();

    let mut comp_map = HashMap::new();

    for gate_str in gate_strs {
        Component::add_gate(gate_str, &mut comp_map);
    }

    let mut res = Vec::new();

    // assume bit 0 is set up correctly

    let mut carry_in_comp = Component::new("ERROR".to_owned(), CompType::Wire, Vec::new(), Vec::new(), Vec::new());
    const MAX_BIT: i32 = 44;
    for i in 0..=MAX_BIT {
        // println!("at bit {}", i);

        let x_wire_id = format!("x{:0>2}", i);
        let y_wire_id = format!("y{:0>2}", i);
        let z_wire_id = format!("z{:0>2}", i);

        let x_wire = {comp_map.get(&x_wire_id).unwrap().clone()};
        // let y_wire = comp_map.get(&y_wire_id).unwrap();

        let xor_1 = get_next_gate(&x_wire, CompType::XOR, &comp_map).unwrap();
        let and_1 = get_next_gate(&x_wire, CompType::AND, &comp_map).unwrap();

        if i == 0 {
            carry_in_comp = and_1.clone();
            continue;
        }


        let xor_2a = get_next_gate(&xor_1, CompType::XOR, &comp_map);
        let xor_2b = get_next_gate(&carry_in_comp, CompType::XOR, &comp_map);

        if xor_2a.is_none() || xor_2b.is_none() {
            // println!("ERROR at bit {}! xor_2 is None: {:?}, {:?}", i, xor_2a, xor_2b);
            fix_none_gate(&xor_1, &carry_in_comp, CompType::XOR, &mut comp_map, &mut res);
        }

        let xor_1 = get_next_gate(&x_wire, CompType::XOR, &comp_map).unwrap();
        let and_1 = get_next_gate(&x_wire, CompType::AND, &comp_map).unwrap();

        let xor_2a = get_next_gate(&xor_1, CompType::XOR, &comp_map);
        let xor_2b = get_next_gate(&carry_in_comp, CompType::XOR, &comp_map);

        let and_2a = get_next_gate(&xor_1, CompType::AND, &comp_map);
        let and_2b = get_next_gate(&carry_in_comp, CompType::AND, &comp_map);

        if xor_2a != xor_2b {
            // println!("ERROR at bit {}! unequal xor_2 ids: {:?}, {:?}", i, xor_2a, xor_2b);
        }

        if and_2a.is_none() || and_2b.is_none() {
            // println!("ERROR at bit {}! and_2 is None: {:?}, {:?}", i, and_2a, and_2b);
        }
        if and_2a != and_2b {
            // println!("ERROR at bit {}! unequal and_2 ids: {:?}, {:?}", i, and_2a, and_2b);
        }
        // if no errors by here, then xor_1, and_1, carry_in are CORRECT


        let xor_1 = get_next_gate(&x_wire, CompType::XOR, &comp_map).unwrap();
        let and_1 = get_next_gate(&x_wire, CompType::AND, &comp_map).unwrap();

        let xor_2a = get_next_gate(&xor_1, CompType::XOR, &comp_map);
        let xor_2b = get_next_gate(&carry_in_comp, CompType::XOR, &comp_map);

        let and_2a = get_next_gate(&xor_1, CompType::AND, &comp_map);
        let and_2b = get_next_gate(&carry_in_comp, CompType::AND, &comp_map);


        if xor_2a.as_ref().unwrap().output_ids[0] != z_wire_id {
            // println!("ERROR at bit {}! sum_output is incorrect: {}", i, xor_2a.as_ref().unwrap().output_ids[0]);
            swap_output_with_desired(&xor_2a.as_ref().unwrap(), z_wire_id, &mut comp_map, &mut res);
        }
        // if no errors by here then sum_bit (zXX), xor_2 are also CORRECT

        let xor_1 = get_next_gate(&x_wire, CompType::XOR, &comp_map).unwrap();
        let and_1 = get_next_gate(&x_wire, CompType::AND, &comp_map).unwrap();

        let xor_2a = get_next_gate(&xor_1, CompType::XOR, &comp_map);
        let xor_2b = get_next_gate(&carry_in_comp, CompType::XOR, &comp_map);

        let and_2a = get_next_gate(&xor_1, CompType::AND, &comp_map);
        let and_2b = get_next_gate(&carry_in_comp, CompType::AND, &comp_map);

        let or_1a = get_next_gate(&and_1, CompType::OR, &comp_map);
        let or_1b = get_next_gate(&and_2a.as_ref().unwrap(), CompType::OR, &comp_map);

        if or_1a.is_none() || or_1b.is_none() {
            // println!("ERROR at bit {}! or_1 is None: {:?}, {:?}", i, or_1a, or_1b);
            fix_none_gate(&and_1, &and_2a.clone().unwrap(), CompType::OR, &mut comp_map, &mut res);
        }

        let xor_1 = get_next_gate(&x_wire, CompType::XOR, &comp_map).unwrap();
        let and_1 = get_next_gate(&x_wire, CompType::AND, &comp_map).unwrap();

        let xor_2a = get_next_gate(&xor_1, CompType::XOR, &comp_map);
        let xor_2b = get_next_gate(&carry_in_comp, CompType::XOR, &comp_map);

        let and_2a = get_next_gate(&xor_1, CompType::AND, &comp_map);
        let and_2b = get_next_gate(&carry_in_comp, CompType::AND, &comp_map);

        let or_1a = get_next_gate(&and_1, CompType::OR, &comp_map);
        let or_1b = get_next_gate(&and_2a.as_ref().unwrap(), CompType::OR, &comp_map);


        if or_1a != or_1b {
            // println!("ERROR at bit {}! unequal or_1 ids: {:?}, {:?}", i, or_1a, or_1b);
        }
        // if no errors by here then all gates are correct,
        // but we still don't know about sum and carry wires

        let xor_1 = get_next_gate(&x_wire, CompType::XOR, &comp_map).unwrap();
        let and_1 = get_next_gate(&x_wire, CompType::AND, &comp_map).unwrap();

        let xor_2a = get_next_gate(&xor_1, CompType::XOR, &comp_map);
        let xor_2b = get_next_gate(&carry_in_comp, CompType::XOR, &comp_map);

        let and_2a = get_next_gate(&xor_1, CompType::AND, &comp_map);
        let and_2b = get_next_gate(&carry_in_comp, CompType::AND, &comp_map);

        let or_1a = get_next_gate(&and_1, CompType::OR, &comp_map);
        let or_1b = get_next_gate(&and_2a.as_ref().unwrap(), CompType::OR, &comp_map);

        carry_in_comp = or_1a.unwrap();

    }

    res.sort();
    return res.join(",");


    fn get_next_gate<'a>(curr_comp: &Component, next_comp_type: CompType, comp_map: &HashMap<String, Component>) -> Option<Component> {
        let output_wire = if curr_comp.comp_type == CompType::Wire {
            curr_comp
        } else {
            comp_map.get(&curr_comp.output_ids[0]).unwrap()
        };
        let next_comp_id = &output_wire.output_ids[
                output_wire.output_ids.iter().position(|x| comp_map.get(x).unwrap().comp_type == next_comp_type)?
            ];
        return Some(comp_map.get(next_comp_id).unwrap().clone());
    }

    fn swap_output_with_desired(curr_comp: &Component, desired_wire_output_id: String, comp_map: &mut HashMap<String, Component>, res: &mut Vec<String>) {
        let mut other_comp = Component::new("ERROR".to_owned(), CompType::Wire, Vec::new(), Vec::new(), Vec::new());
        for (_, o) in comp_map.iter() {
            if o.output_ids.len() > 0 && o.output_ids[0] == desired_wire_output_id {
                other_comp = o.clone();  
                break;
            }
        }

        // println!("before swap: {:?}, {:?}", comp_map.get(&curr_comp.id.clone()).unwrap(), comp_map.get(&other_comp.id).unwrap());

        comp_map.get_mut(&curr_comp.id).unwrap().output_ids = vec![other_comp.output_ids[0].clone()];
        comp_map.get_mut(&other_comp.id).unwrap().output_ids = vec![curr_comp.output_ids[0].clone()];

        res.push(other_comp.output_ids[0].clone());
        res.push(curr_comp.output_ids[0].clone());


        // println!("after swap: {:?}, {:?}", comp_map.get(&curr_comp.id.clone()).unwrap(), comp_map.get(&other_comp.id).unwrap());
        // println!("SWAP DONE");
    }

    fn fix_none_gate(prev_1: &Component, prev_2: &Component, comp_type: CompType, comp_map: &mut HashMap<String, Component>, res: &mut Vec<String>) {
        let next_1 = get_next_gate(prev_1, comp_type.clone(), &comp_map);
        let next_2 = get_next_gate(prev_2, comp_type.clone(), &comp_map);

        // println!("next_1, next_2: {:?}, {:?}", next_1, next_2);

        // println!("prev_1, prev_2: {:?}, {:?}", prev_1, prev_2);

        if next_1.is_none() {
            let desired_wire_id = &next_2.as_ref().unwrap().input_ids[
                next_2.as_ref().unwrap().input_ids.iter().position(|x| x != &prev_2.output_ids[0]).unwrap()
            ];
            swap_output_with_desired(prev_1, desired_wire_id.clone(), comp_map, res);
        } else { // next_2 is none
            let desired_wire_id = &next_1.as_ref().unwrap().input_ids[
                next_1.as_ref().unwrap().input_ids.iter().position(|x| x != &prev_1.output_ids[0]).unwrap()
            ];
            swap_output_with_desired(prev_2, desired_wire_id.clone(), comp_map, res);
        }

        let new_prev_1 = comp_map.get(&prev_1.id);
        let new_prev_2 = comp_map.get(&prev_2.id);

        // println!("AFTER prev_1, prev_2: {:?}, {:?}\n\n", new_prev_1, new_prev_2);
    }
}