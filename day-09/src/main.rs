fn main() {
    let input = include_str!("input.txt");
    println!("PART 1 ANSWER: {}", part1(input));
    println!("PART 2 ANSWER: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    let mut file_blocks: Vec<i32> = Vec::new();
    let mut id = 0;

    for (i, chr) in input.lines().next().unwrap().bytes().enumerate() {
        // println!("{:?}", chr);
        if i & 1 == 1 { // even
            id += 1;
        }
        file_blocks.append(&mut vec![if i%2 == 0 {id} else {-1}; (chr - b'0') as usize])
    }

    let mut left = 0;
    let mut right = file_blocks.len() - 1;

    while left < right {
        if file_blocks[left] != -1 {
            left += 1;
        } else if file_blocks[right] == -1 {
            right -= 1;
        } else {
            file_blocks[left] = file_blocks[right];
            file_blocks[right] = -1;
            left += 1;
            right -= 1;
        }
    }

    // println!("{:?}", file_blocks);

    let mut res: i64 = 0;
    for (i, num) in file_blocks.iter().enumerate() {
        if *num < 0 {
            break;
        }
        res += (i as i64) * (*num as i64);
    }
    return res;
}


fn part2(input: &str) -> i64 {
    let mut id = 0 as usize;
    let mut files = Vec::new();
    let mut openings = Vec::new();

    let mut index = 0;
    for (i, chr) in input.lines().next().unwrap().bytes().enumerate() {
        let file_size = (chr - b'0') as usize;
        if i & 1 == 1 { // odd (fill with empty space)
            id += 1;
            openings.push((index, file_size));
        } else {
            files.push((index, id, file_size));
        }
        index += file_size;
    }

    let mut res: i64 = 0;
    for (file_index, id, file_size) in files.into_iter().rev() {

        let mut j = 0 as usize;
        let mut final_index = file_index as i64;
        while j < openings.len() {
            let (opening_index, opening_size) = openings.get_mut(j).unwrap();

            if *opening_index >= file_index {
                break;
            } else if *opening_size >= file_size {
                // println!("moving file with id {} and size {} to index starting at {}", id, file_size, *opening_index);
                final_index = *opening_index as i64;
                *opening_size -= file_size;
                *opening_index += file_size;

                break
            }
            j += 1;
        }
        let bonus = (id as i64) * (file_size as i64) * (final_index + final_index + (file_size as i64) - 1) / 2;
        // println!("adding file with id {} and length {} at pos {} for total {}", id, file_size, final_index, bonus);
        res += bonus;
    }

    // println!("original layout: {:?}\n", file_blocks);


    

    // println!("{:?}", file_blocks);
    return res;
}
