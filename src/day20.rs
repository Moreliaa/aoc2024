use aoc_lib::map2d::Map2D;
use std::collections::{HashMap, VecDeque};

pub fn run(input: String) {
    let map = Map2D::from_string(input);

    let mut x_start = 0;
    let mut y_start = 0;
    'outer: for x in 0..map.width() {
        for y in 0..map.height() {
            if *map.get(x,y).unwrap() == 'S' {
                x_start = x;
                y_start = y;
                break 'outer;
            }
        }
    }

    let mut seen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut unseen: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let target = 'E';
    let min_shortcut = 100;
    unseen.push_back((x_start, y_start, 0));
    
    loop {
        let u = unseen.pop_front().unwrap();
        let pos_x = u.0;
        let pos_y = u.1;
        let mut steps = u.2;
        if seen.contains_key(&(pos_x, pos_y)) {
            continue;
        }
        seen.insert((pos_x, pos_y), steps);
        

        if *map.get(pos_x, pos_y).unwrap() == target {
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

    let mut shortcuts_pt1: Vec<(i32, i32, i32, i32, i32)> = vec![];
    
    for ((x_start, y_start), d) in seen.iter() {
        let dirs = [(0, 2), (0, -2), (-2, 0), (2, 0)];
        let new_dist = d + 2;
        for d in dirs {
            let x_end = x_start + d.0;
            let y_end = y_start + d.1;
            if !map.is_in_bounds(x_end, y_end) || *map.get(x_end, y_end).unwrap() == '#' {
                continue;
            }
            let item = (x_end, y_end);
            if let Some(val) = seen.get(&item) {
                if new_dist < *val {
                    let shortcut = val - new_dist;
                    if shortcut >= min_shortcut {
                        shortcuts_pt1.push((shortcut, *x_start, *y_start, x_end, y_end));
                    }
                }
            }
        }
    }

    println!("Part 1: {}", shortcuts_pt1.len());

    let mut shortcuts_pt2: Vec<(i32, i32, i32, i32, i32)> = vec![];
    
    for ((x_start, y_start), d) in seen.iter() {
        let max_dist = 20;
        for x_end in x_start - max_dist..=x_start + max_dist {
            for y_end in y_start - max_dist..=y_start + max_dist {
                let dist_to_coord = aoc_lib::util::manhattan_2d((x_end, y_end), (*x_start, *y_start));
                if !map.is_in_bounds(x_end, y_end) || *map.get(x_end, y_end).unwrap() == '#' || dist_to_coord > max_dist {
                    continue;
                }
                let new_dist = d + dist_to_coord;
                let item = (x_end, y_end);
                if let Some(val) = seen.get(&item) {
                    if new_dist < *val {
                        let shortcut = val - new_dist;
                        if shortcut >= min_shortcut {
                            shortcuts_pt2.push((shortcut, *x_start, *y_start, x_end, y_end));
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", shortcuts_pt2.len());

}