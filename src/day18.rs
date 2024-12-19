use aoc_lib::map2d::Map2D;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run(input: String) {
    let limit_pt1 = 1024;
    let size = 71;
    let mut map = Map2D::new(size, size, '.');
    let mut counter = 0;
    for l in input.lines() {
        let mut s = l.split(',').into_iter();
        let (x, y) = (s.next().unwrap().parse::<i32>().unwrap(), s.next().unwrap().parse::<i32>().unwrap());
        map.set(x,y, '#');
        counter += 1;
        if counter >= limit_pt1 {
            let result = try_path(&map);
            if counter == limit_pt1 {
                println!("Part 1: {result}");
            } else if result == -1 {
                println!("Part 2: {x},{y}");
                break;
            }
        }
    }
}

fn try_path(map: &Map2D<char>) -> i32 {
    let mut seen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut unseen: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let target_x = map.width() - 1;
    let target_y = map.height() - 1;
    unseen.push_back((0, 0, 0));
    
    loop {
        if unseen.len() == 0 {
            return -1;
        }
        let u = unseen.pop_front().unwrap();
        let pos_x = u.0;
        let pos_y = u.1;
        let mut steps = u.2;
        if seen.contains_key(&(pos_x, pos_y)) {
            continue;
        }
        seen.insert((pos_x, pos_y), steps);

        if pos_x == target_x && pos_y == target_y {
            break;
        }

        steps += 1;
        let dirs = [(0, 1), (0, -1), (-1, 0), (1, 0)];
        for d in dirs {
            let x = pos_x + d.0;
            let y = pos_y + d.1;
            if !map.is_in_bounds(x, y) || *map.get(x, y).unwrap() == '#' {
                continue;
            }
            let item = (x, y);
            if !seen.contains_key(&item) {
                unseen.push_back((x, y, steps));
            }
        }
    }

    return *seen.get(&(target_x, target_y)).unwrap();
}