use aoc_lib::map2d::Map2D;
use std::cmp::Ordering;
use std::collections::HashSet;
pub fn run(input: String) {
    let mut p1 = 0;
    let mut p2 = 0;

    let map = Map2D::from_string(input);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    for x in 0..map.width() {
        for y in 0..map.height() {
            if seen.contains(&(x, y)) {
                continue;
            }
            let mut sides: HashSet<(i32, i32, char)> = HashSet::new();
            let c = map.get(x, y).unwrap();
            let (area, perimeter) = step(&map, &mut seen, &mut sides, *c, x, y, '.');
            p1 += area * perimeter;
            let mut sides: Vec<(i32, i32, char)> = sides.into_iter().collect();
            sides.sort_by(|a, b| {
                let cmp_mov = a.2.cmp(&b.2);
                if cmp_mov != Ordering::Equal {
                    return cmp_mov;
                }
                if a.2 == 'd' || a.2 == 'u' {
                    let cmp_mov = a.1.cmp(&b.1);
                    if cmp_mov != Ordering::Equal {
                        return cmp_mov;
                    }
                    let cmp_mov = a.0.cmp(&b.0);
                    return cmp_mov;
                } else {
                    let cmp_mov = a.0.cmp(&b.0);
                    if cmp_mov != Ordering::Equal {
                        return cmp_mov;
                    }
                    let cmp_mov = a.1.cmp(&b.1);
                    return cmp_mov;
                }
            });

            let mut curr_x = sides[0].0;
            let mut curr_y = sides[0].1;
            let mut curr_mov = sides[0].2;
            let mut num_sides = 1;
            for i in 1..sides.len() {
                let next_x = sides[i].0;
                let next_y = sides[i].1;
                let next_mov = sides[i].2;
                let total_diff = (curr_x - next_x).abs() + (curr_y - next_y).abs();
                if total_diff != 1 || next_mov != curr_mov {
                    num_sides += 1;
                }
                curr_x = next_x;
                curr_y = next_y;
                curr_mov = next_mov;
            }
            p2 += area * num_sides;
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn step(
    map: &Map2D<char>,
    seen: &mut HashSet<(i32, i32)>,
    sides: &mut HashSet<(i32, i32, char)>,
    c: char,
    x: i32,
    y: i32,
    movement: char,
) -> (i32, i32) {
    let tile = map.get(x, y);
    if tile == None {
        sides.insert((x, y, movement));
        return (0, 1);
    }

    let tile = tile.unwrap();
    let mut area = 0;
    let mut perimeter = 0;

    if *tile == c {
        if !seen.contains(&(x, y)) {
            seen.insert((x, y));
            area += 1;
        } else {
            return (0, 0);
        }
    }
    if *tile != c {
        perimeter += 1;
        sides.insert((x, y, movement));
        return (0, perimeter);
    }

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let movements = ['u', 'd', 'r', 'l'];
    for (i, d) in dirs.into_iter().enumerate() {
        let result = step(map, seen, sides, c, x + d.0, y + d.1, movements[i]);
        area += result.0;
        perimeter += result.1;
    }
    return (area, perimeter);
}
