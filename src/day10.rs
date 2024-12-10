use aoc_lib::map2d::Map2D;
use std::collections::HashSet;

pub fn run(input: String) {
    let mut p1 = 0;
    let mut p2 = 0;


    let char_map = Map2D::from_string(input);
    let mut map: Map2D<u32> = Map2D::new(char_map.width(), char_map.height(), 0);
    for x in 0..map.width() {
        for y in 0..map.height() {
            map.set(x, y, char_map.get(x, y).unwrap().to_digit(10).unwrap());
        }
    }

    for x in 0..map.width() {
        for y in 0..map.height() {
            if *map.get(x,y).unwrap() != 0 {
                continue;
            }
            let dirs = [(1, 0),(0, 1),(-1, 0),(0, -1)];
            let mut seen: HashSet<(i32, i32)> = HashSet::new();
            for d in dirs {
                let x_next = x + d.0;
                let y_next = y + d.1;
                p1 += check_path(&map, x_next, y_next, x, y, &mut seen);
                p2 += check_path_pt2(&map, x_next, y_next, x, y);
            }
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn check_path(map: &Map2D<u32>, x: i32, y: i32, x_last: i32, y_last: i32, seen: &mut HashSet<(i32, i32)>) -> i32 {
    let curr_digit = map.get(x, y);
    if curr_digit == None {
        return 0;
    }
    let last_digit = map.get(x_last, y_last).unwrap();
    let curr_digit = curr_digit.unwrap();
    if *curr_digit != *last_digit + 1 {
        return 0;
    }
    if *curr_digit == 9 {
        let loc = (x, y);
        if seen.contains(&loc) {
            return 0;
        } else {
            seen.insert(loc);
            return 1;
        }
    }
    let dirs = [(1, 0),(0, 1),(-1, 0),(0, -1)];
    let mut result = 0;
    for d in dirs {
        let x_next = x + d.0;
        let y_next = y + d.1;
        if x_next == x_last && y_next == y_last {
            continue;
        }
        result += check_path(map, x_next, y_next, x, y, seen);
    }
    return result;
}

fn check_path_pt2(map: &Map2D<u32>, x: i32, y: i32, x_last: i32, y_last: i32) -> i32 {
    let curr_digit = map.get(x, y);
    if curr_digit == None {
        return 0;
    }
    let last_digit = map.get(x_last, y_last).unwrap();
    let curr_digit = curr_digit.unwrap();
    if *curr_digit != *last_digit + 1 {
        return 0;
    }
    if *curr_digit == 9 {
        return 1;
    }
    let dirs = [(1, 0),(0, 1),(-1, 0),(0, -1)];
    let mut result = 0;
    for d in dirs {
        let x_next = x + d.0;
        let y_next = y + d.1;
        if x_next == x_last && y_next == y_last {
            continue;
        }
        result += check_path_pt2(map, x_next, y_next, x, y);
    }
    return result;
}