use aoc_lib::map2d::Map2D;
use std::collections::HashSet;

pub fn run(input: String) {
    let mut map = Map2D::from_string(input.clone());

    let mut g = get_guard(&map);
    let g_cloned = g.clone();
    while g.step(&mut map) {}

    let p1 = map.aggregate(|tile, _, _| if *tile == 'X' { 1 } else { 0 });
    let mut p2 = 0;

    for x in 0..map.width() {
        'y: for y in 0..map.height() {
            if *map.get(x, y).unwrap() != 'X' || (x == g.first_step.0 && y == g.first_step.1) {
                continue;
            }
            let mut g_cloned_again = g_cloned.clone();
            let mut seen_moves: HashSet<(i32, i32, Dir)> = HashSet::new();
            let mut map_cloned = Map2D::from_string(input.clone());
            map_cloned.set(x, y, '#');
            while !seen_moves.contains(&(g_cloned_again.x, g_cloned_again.y, g_cloned_again.dir)) {
                seen_moves.insert((g_cloned_again.x, g_cloned_again.y, g_cloned_again.dir));
                if g_cloned_again.step(&mut map_cloned) == false {
                    continue 'y;
                }
            }
            p2 += 1;
        }
    }
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn get_guard(map: &Map2D<char>) -> Guard {
    for x in 0..map.width() {
        for y in 0..map.height() {
            let tile = *map.get(x, y).unwrap();
            if tile != '#' && tile != '.' {
                let dir = match tile {
                    '^' => Dir::U,
                    '<' => Dir::L,
                    '>' => Dir::R,
                    'v' => Dir::D,
                    _ => panic!(),
                };
                return Guard {
                    x,
                    y,
                    dir,
                    first_step: (-1, -1),
                };
            }
        }
    }
    panic!();
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Dir {
    U,
    L,
    R,
    D,
}
#[derive(Clone)]
struct Guard {
    x: i32,
    y: i32,
    dir: Dir,
    first_step: (i32, i32),
}

impl Guard {
    fn step(&mut self, map: &mut Map2D<char>) -> bool {
        map.set(self.x, self.y, 'X');
        let shift = match self.dir {
            Dir::U => (0, -1),
            Dir::L => (-1, 0),
            Dir::R => (1, 0),
            Dir::D => (0, 1),
        };
        let next_tile = map.get(self.x + shift.0, self.y + shift.1);
        if next_tile == None {
            return false;
        }
        if *next_tile.unwrap() != '#' {
            self.x += shift.0;
            self.y += shift.1;
            if self.first_step == (-1, -1) {
                self.first_step = (self.x, self.y);
            }
            return true;
        } else {
            let new_dir = match self.dir {
                Dir::U => Dir::R,
                Dir::L => Dir::U,
                Dir::R => Dir::D,
                Dir::D => Dir::L,
            };
            self.dir = new_dir;
            return true;
        }
    }
}
