use aoc_lib::map2d::Map2D;

pub fn run(input: String) {
    let map = Map2D::from_string(input);

    let mut p1 = 0;
    let mut p2 = 0;

    let dirs = [
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];

    for x in 0..map.width() {
        for y in 0..map.height() {
            if *map.get(x, y).unwrap() == 'X' {
                let chars = ['M', 'A', 'S'];
                'dirs: for d in dirs {
                    for (idx, c) in chars.iter().enumerate() {
                        if let Some(m) =
                            map.get(x + d.0 * (1 + idx as i32), y + d.1 * (1 + idx as i32))
                        {
                            if *m != *c {
                                continue 'dirs;
                            }
                        } else {
                            continue 'dirs;
                        }
                    }
                    p1 += 1;
                }
            }
        }
    }

    let dirs_pt2 = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
    let incorrect = [String::from("MSMS"), String::from("SMSM")];
    for x in 0..map.width() {
        'y: for y in 0..map.height() {
            if *map.get(x, y).unwrap() == 'A' {
                let mut chars_found: String = String::new();
                let mut m_count = 0;
                let mut s_count = 0;
                for d in dirs_pt2 {
                    if let Some(m) = map.get(x + d.0, y + d.1) {
                        if *m == 'M' {
                            m_count += 1;
                        } else if *m == 'S' {
                            s_count += 1;
                        } else {
                            continue 'y;
                        }
                        chars_found.push(*m);
                    } else {
                        continue 'y;
                    }
                }
                if m_count != 2 || s_count != 2 {
                    continue 'y;
                }
                for inc in incorrect.iter() {
                    if chars_found == *inc {
                        continue 'y;
                    }
                }
                println!("{chars_found}");
                p2 += 1;
            }
        }
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
