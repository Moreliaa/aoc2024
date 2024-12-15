pub fn run(input: String) {
    println!("Part 1: {}", pt1(input.clone()));
    println!("Part 2: {}", pt2(input));
}

fn pt1(input: String) -> i64 {
    let input = input.lines().into_iter().next().unwrap();
    let mut p1 = 0;

    let mut blocks: Vec<(usize, i32, u32, u32)> = vec![]; // start_position, id_number, digit, length
    let mut free_blocks: Vec<(usize, u32)> = vec![]; // start_position, length
    let mut start_position = 0;
    let mut id_number = 0;
    for (i, c) in input.chars().enumerate() {
        let digit = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            // block
            let block = (start_position, id_number, digit, digit);
            blocks.push(block);
            id_number += 1;
            start_position += digit as usize;
        } else {
            // free
            let block = (start_position, digit);
            free_blocks.push(block);
            start_position += digit as usize;
        }
    }
    free_blocks.reverse();

    let mut new_blocks: Vec<(usize, i32, u32, u32)> = vec![];
    while free_blocks.len() > 0 {
        let free_block = free_blocks.pop().unwrap();
        let mut start_position = free_block.0;
        let mut length = free_block.1;
        while length > 0 {
            let next_filled_block = blocks.pop().unwrap();
            if start_position > next_filled_block.0 {
                blocks.push(next_filled_block);
                length = 0;
                continue;
            }
            let id_number = next_filled_block.1;
            let filled_digit = next_filled_block.2;
            let filled_length = next_filled_block.3;
            if filled_length > length {
                let new_length = filled_length - length;
                let new_block = (start_position, id_number, filled_digit, length);
                new_blocks.push(new_block);
                let new_filled_block = (next_filled_block.0, id_number, filled_digit, new_length);
                blocks.push(new_filled_block);

                length = 0;
            } else {
                let new_block = (start_position, id_number, filled_digit, filled_length);
                new_blocks.push(new_block);
                length -= filled_length;
                start_position += filled_length as usize;
            }
        }
    }

    blocks.append(&mut new_blocks);
    blocks.sort_by(|a, b| a.0.cmp(&b.0));

    for b in blocks {
        let mut pos = b.0 as i64;
        let mut remaining = b.3;
        while remaining > 0 {
            p1 += pos * b.1 as i64;
            pos += 1;
            remaining -= 1;
        }
    }

    return p1;
}

fn pt2(input: String) -> i64 {
    let input = input.lines().into_iter().next().unwrap();
    let mut p2 = 0;

    let mut blocks: Vec<(usize, i32, u32, u32)> = vec![]; // start_position, id_number, digit, length
    let mut free_blocks: Vec<(usize, u32)> = vec![]; // start_position, length
    let mut start_position = 0;
    let mut id_number = 0;
    for (i, c) in input.chars().enumerate() {
        let digit = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            // block
            let block = (start_position, id_number, digit, digit);
            blocks.push(block);
            id_number += 1;
            start_position += digit as usize;
        } else {
            // free
            let block = (start_position, digit);
            free_blocks.push(block);
            start_position += digit as usize;
        }
    }
    blocks.sort_by(|a, b| a.1.cmp(&b.1));

    let mut used_blocks: Vec<(usize, i32, u32, u32)> = vec![];

    let mut new_blocks: Vec<(usize, i32, u32, u32)> = vec![];
    while blocks.len() > 0 {
        let block_to_move = blocks.pop().unwrap();
        for i in 0..free_blocks.len() {
            if free_blocks[i].0 > block_to_move.0 {
                used_blocks.push(block_to_move);
                break;
            } else if free_blocks[i].1 >= block_to_move.3 {
                let new_block = (
                    free_blocks[i].0,
                    block_to_move.1,
                    block_to_move.2,
                    block_to_move.3,
                );
                used_blocks.push(new_block);

                let free_length = free_blocks[i].1 - block_to_move.3;
                let free_start_position = free_blocks[i].0 + block_to_move.3 as usize;
                let free_block = (free_start_position, free_length);
                free_blocks.splice(i..i + 1, [free_block]);
                break;
            }
        }
    }

    used_blocks.append(&mut new_blocks);
    used_blocks.sort_by(|a, b| a.0.cmp(&b.0));

    for b in used_blocks {
        let mut pos = b.0 as i64;
        let mut remaining = b.3;
        while remaining > 0 {
            p2 += pos * b.1 as i64;
            pos += 1;
            remaining -= 1;
        }
    }

    return p2;
}
