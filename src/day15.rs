use aoc_lib::map2d::Map2D;

pub fn run(input: String) {
    let mut split = input.split("\n\n");
    let map_string = split.next().unwrap().to_string();
    let mut map = Map2D::from_string(map_string.clone());
    let moves: String = split
        .next()
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .join("");

    let (mut robot_pos_x, mut robot_pos_y) = (0, 0);
    for x in 0..map.width() {
        for y in 0..map.height() {
            if *map.get(x, y).unwrap() == '@' {
                robot_pos_x = x;
                robot_pos_y = y;
                break;
            }
        }
    }

    for m in moves.chars() {
        let dir = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };
        if step_pt1(&mut map, robot_pos_x, robot_pos_y, dir) {
            robot_pos_x += dir.0;
            robot_pos_y += dir.1;
        }
    }

    let mut map_string_pt2 = String::new();
    for c in map_string.chars() {
        match c {
            '#' | '.' => {
                map_string_pt2.push(c);
                map_string_pt2.push(c);
            }
            '@' => {
                map_string_pt2.push(c);
                map_string_pt2.push('.');
            }
            '\n' => map_string_pt2.push(c),
            'O' => {
                map_string_pt2.push('[');
                map_string_pt2.push(']');
            }
            _ => panic!(),
        };
    }

    let mut map_pt2 = Map2D::from_string(map_string_pt2);

    let (mut robot_pos_x, mut robot_pos_y) = (0, 0);
    for x in 0..map_pt2.width() {
        for y in 0..map_pt2.height() {
            if *map_pt2.get(x, y).unwrap() == '@' {
                robot_pos_x = x;
                robot_pos_y = y;
                break;
            }
        }
    }

    for m in moves.chars() {
        let dir = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!(),
        };
        let coords = vec![(robot_pos_x, robot_pos_y)];
        let mut pushed: Vec<(char, i32, i32)> = vec![];
        if check_pt2(&map_pt2, coords, &mut pushed, dir) {
            pushed.sort_by(|a, b| {
                return match m {
                    '<' => b.1.cmp(&a.1),
                    '>' => a.1.cmp(&b.1),
                    '^' => b.2.cmp(&a.2),
                    'v' => a.2.cmp(&b.2),
                    _ => panic!(),
                };
            });
            while let Some(push) = pushed.pop() {
                let x = push.1;
                let y = push.2;
                map_pt2.set(x, y, '.');
                map_pt2.set(x + dir.0, y + dir.1, push.0);
            }
            robot_pos_x += dir.0;
            robot_pos_y += dir.1;
        }
    }

    println!(
        "Part 1: {}",
        map.aggregate(|val, x, y| if *val == 'O' { x + 100 * y } else { 0 })
    );
    println!(
        "Part 2: {}",
        map_pt2.aggregate(|val, x, y| if *val == '[' { x + 100 * y } else { 0 })
    );
}

fn step_pt1(map: &mut Map2D<char>, x: i32, y: i32, dir: (i32, i32)) -> bool {
    let tile = *map.get(x, y).unwrap();
    match tile {
        '.' => return true,
        '#' => return false,
        '@' | 'O' => {
            if step_pt1(map, x + dir.0, y + dir.1, dir) {
                map.set(x, y, '.');
                map.set(x + dir.0, y + dir.1, tile);
                return true;
            } else {
                return false;
            }
        }
        _ => panic!(),
    };
}

fn check_pt2(
    map: &Map2D<char>,
    coords: Vec<(i32, i32)>,
    pushed: &mut Vec<(char, i32, i32)>,
    dir: (i32, i32),
) -> bool {
    let mut coords_next: Vec<(i32, i32)> = vec![];
    for c in coords.iter() {
        let x = c.0;
        let y = c.1;
        let tile = *map.get(x, y).unwrap();
        match tile {
            '.' => continue,
            '#' => return false,
            '@' => {
                coords_next.push((x + dir.0, y + dir.1));
            }
            '[' => {
                coords_next.push((x + dir.0, y + dir.1));
                if dir.1 != 0 {
                    let right_next = (x + 1 + dir.0, y + dir.1);
                    if !coords_next.contains(&right_next) {
                        coords_next.push(right_next);
                    }
                    let right: (char, i32, i32) = (']', x + 1, y);
                    if !pushed.contains(&right) {
                        pushed.push(right);
                    }
                }
            }
            ']' => {
                coords_next.push((x + dir.0, y + dir.1));
                if dir.1 != 0 {
                    let left_next = (x - 1 + dir.0, y + dir.1);
                    if !coords_next.contains(&left_next) {
                        coords_next.push(left_next);
                    }

                    let left: (char, i32, i32) = ('[', x - 1, y);
                    if !pushed.contains(&left) {
                        pushed.push(left);
                    }
                }
            }
            _ => panic!(),
        };
    }

    for c in coords.iter() {
        let tile = *map.get(c.0, c.1).unwrap();
        if tile != '.' && tile != '#' {
            pushed.push((tile, c.0, c.1));
        }
    }
    if coords_next.len() == 0 {
        return true;
    } else {
        return check_pt2(map, coords_next, pushed, dir);
    }
}
