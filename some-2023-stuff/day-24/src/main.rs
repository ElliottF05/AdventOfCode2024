use std::{collections::HashMap, i64::{MAX, MIN}};

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {:?}", part2(input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"\-*[0-9]+").unwrap();
    let stones = input
        .lines()
        .map(|line| {
            let vals = re.captures_iter(line).map(|cap| cap.get(0).unwrap().as_str().parse::<f64>().unwrap()).collect::<Vec<_>>();
            return vec![vals[0], vals[1], vals[3], vals[4]];
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for (i, stone_1) in stones.iter().enumerate() {
        for stone_2 in stones.iter().skip(i+1) {
            if check_stones(stone_1, stone_2) {
                res += 1;
            }
        }
    }

    fn check_stones(stone_1: &Vec<f64>, stone_2: &Vec<f64>) -> bool {
        if stone_1[3] * stone_2[2] == stone_2[3] * stone_1[2] { // parallel
            // assume no parallel lines are desired
            println!("parallel");
            return false;
        }

        const MIN_COORD: f64 = (200000000000000 as i64) as f64;
        const MAX_COORD: f64 = (400000000000000 as i64) as f64;

        let bx = stone_2[0] - stone_1[0];
        let by = stone_2[1] - stone_1[1];

        let ux = stone_1[2];
        let uy = stone_1[3];

        let vx = -stone_2[2];
        let vy = -stone_2[3];

        // c1 * u + c2 * v = b
        // A = [u, v]
        // Ax = b
        // Find A^-1

        // A = [a b]
        //     [c d]
        //
        // A^-1 = [d -b] * 1/det
        //        [-c a]

        let (a,b,c,d) = (ux, vx, uy, vy);

        let det = a*d - b*c;
        let c1 = 1.0/det * (bx*d + by*(-b));
        let c2 = 1.0/det * (bx*(-c) + by*a);

        if c1 < 0.0 || c2 < 0.0 {
            return false;
        }

        let x = stone_1[0] + c1 * ux;
        let y = stone_1[1] + c1 * uy;

        if MIN_COORD <= x && x <= MAX_COORD && MIN_COORD <= y && y <= MAX_COORD {
            return true;
        }

        return false;
    }   

    return res;
}



fn part2(input: &str) -> Option<f64> {

    /*
    With help from reddit...

    Key insight is shifting perspective to that of the flying rock:

        Let the flying rock have velocity v_r and initial position p_r. For a given hailstone h_i 
        (vel = v_i, initial pos = p_i), its velocity *relative to the hailstone* is v_i - v_r.

        In this reference frame, the rock is stationary. Therefore, since it does not move,
        it will intersect a hailstone iff it is initial position is already on the hailstone's path
        in this reference frame. 

        Now, this must hold true for every hailstone. However, since the rock is stationary, this means
        that for it to intersect every path, all the paths must intersect at one point, where the
        hailstone is stationary at.

        This is now an extension from part 1, and we check if the adjusted paths for a given velocity
        all intersect at one point (first check x-y plane, then verify the z coordinate).

        If range of potential velocities is small enough, this can be brute forced.
     */

    const MAX_SPEED: i64 = 1000;

    let re = Regex::new(r"\-*[0-9]+").unwrap();
    let hailstones = input
        .lines()
        .map(|line| {
            let vals = re.captures_iter(line).map(|cap| cap.get(0).unwrap().as_str().parse::<i64>().unwrap()).collect::<Vec<_>>();
            return vec![vals[0], vals[1], vals[2], vals[3], vals[4], vals[5]];
        })
        .collect::<Vec<_>>();

    for i in 0..=MAX_SPEED {
        for j in -i..=i {
            let patterns = [
                [-i as f64, j as f64, 0.0],
                [i as f64, j as f64, 0.0],
                [j as f64, -i as f64, 0.0],
                [j as f64, i as f64, 0.0],
            ];

            for pattern in patterns {
                if let Some(intersection) = check_stones(&hailstones, &pattern.to_vec()) {
                    println!("intersection with rock vel {:?}", pattern); // -228, 166, 0
                    return Some(intersection.iter().sum());
                }
            }
        }
    }

    return None;
        
    fn check_stones(hailstones: &Vec<Vec<i64>>, rock_vel: &Vec<f64>) -> Option<Vec<f64>> {
        // println!("rock_vel: {:?}", rock_vel);
        let base_stone = &hailstones[0];

        let (p1, mut v1) = (
            base_stone.iter().cloned().take(3).collect::<Vec<_>>(),
            base_stone.iter().cloned().skip(3).take(3).collect::<Vec<_>>()
        );
        for i in 0..3 {
            v1[i] -= rock_vel[i] as i64;
        }

        let mut intersection = None;
        for other_stone in hailstones.iter().skip(1) {
            let (p2, mut v2) = (
                other_stone.iter().cloned().take(3).collect::<Vec<_>>(),
                other_stone.iter().cloned().skip(3).take(3).collect::<Vec<_>>()
            );
            for i in 0..3 {
                v2[i] -= rock_vel[i] as i64;
            }
            if let Some(new_intersection) = get_intersection(&p1, &v1, &p2, &v2) {
                if intersection.is_none() {
                    intersection = Some(new_intersection);
                } else {
                    if intersection.clone().unwrap() != new_intersection {
                        return None;
                    }
                }
            } else {
                return None;
            }
        }

        if intersection == None {
            return None;
        }

        // find z:
        let base_stone1 = &hailstones[1];

        let t1 = (base_stone[0] as f64 - intersection.clone().unwrap()[0]) / (rock_vel[0] as f64 - base_stone[3] as f64); // 5/3
        let t2 = (base_stone1[0] as f64 - intersection.clone().unwrap()[0]) / (rock_vel[0] as f64 - base_stone1[3] as f64); // 2
        let z1 = base_stone[2] as f64 + t1 * base_stone[5] as f64; // 
        let z2 = base_stone1[2] as f64 + t2 * base_stone1[5] as f64; 

        let vz = (z2 - z1) / (t2 - t1);
        let z = z1 - t1 * vz;
        
        return Some(vec![intersection.clone().unwrap()[0], intersection.clone().unwrap()[1], z]);
    }

    fn get_intersection(p1: &Vec<i64>, v1: &Vec<i64>, p2: &Vec<i64>, v2: &Vec<i64>) -> Option<Vec<f64>> {
        // println!("p1, v1: {:?}, {:?}", p1, v1);
        // println!("p2, v2: {:?}, {:?}", p2, v2);
        if v1[1] * v2[0] == v2[1] * v1[0] { // parallel
            // assume no parallel lines are desired
            // println!("parallel");
            return None;
        }

        // solve p1 + c1*v1 = p2 + c2*v2
        // -> c1*v1 + c2*(-v2) = p2 - p1

        let rx = p2[0] - p1[0];
        let ry = p2[1] - p1[1];

        // c1 * u + c2 * v = r
        // A = [u, v]
        // Ax = r
        // Find A^-1

        // A = [a b]
        //     [c d]
        //
        // A^-1 = [d -b] * 1/det
        //        [-c a]

        let (a,c,b,d) = (v1[0], v1[1], -v2[0], -v2[1]);

        let det = (a*d - b*c) as f64;
        if det == 0.0 {
            return None;
        }
        let c1 = (rx*d + ry*(-b)) as f64 / det;
        let c2 = (rx*(-c) + ry*a) as f64 / det;

        if c1 < 0.0 || c2 < 0.0 {
            return None;
        }

        let x = p1[0] as f64 + (c1 * v1[0] as f64);
        let y = p1[1] as f64 + (c1 * v1[1] as f64);
        let z = p1[2] as f64 + (c1 * v1[2] as f64);

        // println!("intersection at {:?}", vec![x,y,z]);

        return Some(vec![x,y,z]);
    }

}
