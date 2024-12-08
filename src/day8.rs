use aoc_lib::map2d::Map2D;
use std::collections::HashMap;
use aoc_lib::util::manhattan_2d;

pub fn run(input: String) {
    let map = Map2D::from_string(input);
    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for x in 0..map.width() {
        for y in 0..map.height() {
            if let Some(tile) = map.get(x, y) {
                if *tile == '.' {
                    continue;
                }
                if !nodes.contains_key(tile) {
                    nodes.insert(*tile, vec![(x, y)]);
                } else {
                    let n = nodes.get_mut(tile).unwrap();
                    n.push((x, y));
                }
            }
        }
    }
    let mut map_anti = Map2D::new(map.width(), map.height(), '.');
    let mut map_anti_pt2 = Map2D::new(map.width(), map.height(), '.');

    for (_, nodes) in nodes.into_iter() {
        for i in 0..nodes.len() {
            let n1 = nodes[i];
            for j in i + 1..nodes.len() {
                let n2 = nodes[j];
                let step_x = n2.0 - n1.0;
                let step_y = n2.1 - n1.1;
                step(&mut map_anti, &n1, &n2, n1.0, n1.1, step_x, step_y);
                step(&mut map_anti, &n1, &n2, n2.0, n2.1, step_x, step_y);
                step(&mut map_anti, &n1, &n2, n1.0, n1.1, -step_x, -step_y);
                step(&mut map_anti, &n1, &n2, n2.0, n2.1, -step_x, -step_y);

                step_pt2(&mut map_anti_pt2, n1.0, n1.1, step_x, step_y);
                step_pt2(&mut map_anti_pt2, n1.0, n1.1, -step_x, -step_y);
            }
        }
    }

    let p1 = map_anti.aggregate(|tile, _, _| if *tile == '#' { 1 } else { 0 });
    let p2 = map_anti_pt2.aggregate(|tile, _, _| if *tile == '#' { 1 } else { 0 });

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn step(map_anti: &mut Map2D<char>, n1: &(i32, i32), n2: &(i32, i32), x_start: i32, y_start: i32, step_x: i32, step_y: i32) {
    let mut x_curr = x_start + step_x;
    let mut y_curr = y_start + step_y;
    while map_anti.is_in_bounds(x_curr, y_curr) {
        let m1 = manhattan_2d((x_curr, y_curr), *n1);
        let m2 = manhattan_2d((x_curr, y_curr), *n2);
        if m1 == 2 * m2 || m2 == 2 * m1 {
            map_anti.set(x_curr, y_curr, '#');
        }
        x_curr += step_x;
        y_curr += step_y;
    }
}

fn step_pt2(map_anti: &mut Map2D<char>, x_start: i32, y_start: i32, step_x: i32, step_y: i32) {
    let mut x_curr = x_start;
    let mut y_curr = y_start;
    while map_anti.is_in_bounds(x_curr, y_curr) {
        map_anti.set(x_curr, y_curr, '#');
        x_curr += step_x;
        y_curr += step_y;
    }
}