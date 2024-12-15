use std::{fs::File, io::{BufWriter, Write}};

use regex::Regex;

// This was the first one I couldn't do :(
// I couldn't get part 2, at first I didn't like the problem structure, thought it wasn't clear
// enough at all about what a "christmas tree pattern" looks like (this is a valid criticism).
// Then looked on reddit for hints and found one guy looking for certain aspects that would suggest
// a christmas tree, such as fully surrounded blocks (I used 5x5 solid blocks).
// This strategy is really smart and convinces me this is actually a decent problem, requiring
// me to think outisde the box.

fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2_2(input));
}

fn part1(input: &str) -> i64 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const TIME: i32 = 100;

    let re = Regex::new(r"\-*[0-9]+").unwrap();

    let mut robots: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            re
                .captures_iter(line)
                .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    
    let mut quadrants = vec![0 ; 4];
    for robot in robots {
        let (mut x, mut y) = (robot[0], robot[1]);
        let (dx, dy) = (robot[2], robot[3]);

        x += dx * TIME;
        y += dy * TIME;

        x = x.rem_euclid(WIDTH);
        y = y.rem_euclid(HEIGHT);

        // println!("x,y: {},{}", x, y);

        if x == WIDTH / 2 || y == HEIGHT / 2 {
            continue;
        } else if x > WIDTH / 2 && y > HEIGHT / 2 {
            quadrants[0] += 1;
        } else if x < WIDTH / 2 && y > HEIGHT / 2{
            quadrants[1] += 1;
        } else if x < WIDTH / 2 && y < HEIGHT / 2 {
            quadrants[2] += 1
        } else {
            quadrants[3] += 1
        }
    }

    // println!("quadrants: {:?}", quadrants);

    return quadrants.iter().fold(1, |product, val| product * val);
}



fn part2(input: &str) -> i64 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const TIME: i32 = 500;

    let output_file = File::create("output.txt").unwrap();
    let mut file_writer = BufWriter::new(output_file);


    let re = Regex::new(r"\-*[0-9]+").unwrap();

    let mut robots: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            re
                .captures_iter(line)
                .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut grid = vec![ vec![b' ' ; WIDTH as usize] ; HEIGHT as usize];

    for step in 1..TIME {

        for robot in robots.iter_mut() {
            let (mut x, mut y) = (robot[0], robot[1]);
            let (dx, dy) = (robot[2], robot[3]);

            grid[y as usize][x as usize] = b' ';

            x += dx;
            y += dy;

            x = x.rem_euclid(WIDTH);
            y = y.rem_euclid(HEIGHT);

            grid[y as usize][x as usize] = b'#';

            (robot[0], robot[1]) = (x, y);
        }

        file_writer.write_all(format!("STEP: {}\n", step).as_bytes()).unwrap();
        for line in &grid {
            // println!("{:?}", String::from_utf8(line.clone()));
            file_writer.write_all(&line[..]).unwrap();
            file_writer.write_all(b"\n").unwrap();
        }
        file_writer.write_all(b"\n\n").unwrap();
    }

    file_writer.flush().unwrap();

    return -1;
}




fn part2_2(input: &str) -> i64 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const TIME: i32 = 50000;

    let re = Regex::new(r"\-*[0-9]+").unwrap();

    let mut robots: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            re
                .captures_iter(line)
                .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut grid = vec![ vec![b' ' ; WIDTH as usize] ; HEIGHT as usize];

    for step in 1..TIME {

        for robot in robots.iter_mut() {
            let (mut x, mut y) = (robot[0], robot[1]);
            let (dx, dy) = (robot[2], robot[3]);

            grid[y as usize][x as usize] = b' ';

            x += dx;
            y += dy;

            x = x.rem_euclid(WIDTH);
            y = y.rem_euclid(HEIGHT);

            grid[y as usize][x as usize] = b'#';

            (robot[0], robot[1]) = (x, y);
        }

        let mut fully_surrounded_count = 0;
        for r in 2..HEIGHT-2 {
            for c in 2..WIDTH-2 {

                let mut good = true;
                for dr in -2..=2 {
                    for dc in -2..=2 {
                        if grid[(r + dr) as usize][(c + dc) as usize] != b'#' {
                            good = false;
                            break;
                        }
                    }
                }
                if good {
                    fully_surrounded_count += 1;
                }
            }
        }

        if fully_surrounded_count >= 20 {
            println!("step: {step}");
            for line in &grid {
                println!("{}", String::from_utf8(line.clone()).unwrap());
            }
        }
    }

    return -1;
}



