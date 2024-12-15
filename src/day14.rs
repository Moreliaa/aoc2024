use aoc_lib::map2d::Map2D;
use fancy_regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn run(input: String) {
    let w = 101;
    let h = 103;
    let seconds = 10000;
    let seconds_pt1 = 100;

    let quadrant_w = w / 2;
    let quadrant_h = h / 2;

    let rx = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").unwrap();

    let mut quad_tl = 0;
    let mut quad_tr = 0;
    let mut quad_bl = 0;
    let mut quad_br = 0;

    let mut robots: Vec<Vec<(i32, i32)>> = vec![];

    for r in input.lines() {
        let cap = rx.captures(r).unwrap().unwrap();
        let mut px = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let mut py = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vx = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();

        let mut robot: Vec<(i32, i32)> = vec![];

        for s in 1..=seconds {
            px += vx;
            py += vy;
            if px < 0 {
                px += w;
            } else if px >= w {
                px -= w;
            }

            if py < 0 {
                py += h;
            } else if py >= h {
                py -= h;
            }

            robot.push((px, py));

            if s == seconds_pt1 {
                if px < quadrant_w {
                    if py < quadrant_h {
                        quad_tl += 1;
                    } else if py > quadrant_h {
                        quad_bl += 1;
                    }
                } else if px > quadrant_w {
                    if py < quadrant_h {
                        quad_tr += 1;
                    } else if py > quadrant_h {
                        quad_br += 1;
                    }
                }
            }
        }
        robots.push(robot);
    }
    let p1 = quad_bl * quad_br * quad_tl * quad_tr;
    println!("Part 1: {}", p1);

    for s in 0..seconds {
        let mut map: Map2D<char> = Map2D::new(w, h, '.');
        for r in robots.iter() {
            map.set(r[s].0, r[s].1, '#');
        }

        let folder = "day14";
        if !Path::exists(&Path::new(folder)) {
            fs::create_dir(folder).unwrap();
        }
        let mut file = File::create(format!("{folder}/{}.ppm", s + 1)).unwrap();
        file.write_all(canvas_to_ppm(&map).as_bytes()).unwrap();
    }
}

fn canvas_to_ppm(map: &Map2D<char>) -> String {
    let mut output: Vec<String> = vec![];
    output.push(String::from("P3\n"));
    let canvas_dim = format!("{} {}\n", map.width(), map.height());
    output.push(String::from(&canvas_dim));
    output.push(String::from("255\n"));

    let max_characters = 70;

    let newline = String::from("\n");
    let space = String::from(" ");

    for y in 0..map.height() {
        let mut characters_in_line = 0;
        for x in 0..map.width() {
            if x > 0 {
                characters_in_line += 1;
                if characters_in_line > max_characters {
                    output.push(newline.clone());
                    characters_in_line = 0;
                } else {
                    output.push(space.clone());
                }
            }
            let pixel = map.get(x, y).unwrap();
            let colors = if *pixel == '#' {
                [255, 255, 255]
            } else {
                [0, 0, 0]
            };
            for (i, c) in colors.iter().enumerate() {
                let c_str = c.to_string();

                characters_in_line += c_str.len();
                if characters_in_line > max_characters {
                    while output.last().unwrap() == &" " {
                        output.pop();
                    }
                    output.push(newline.clone());
                    characters_in_line = c_str.len();
                }
                output.push(c_str);

                if i < colors.len() - 1 {
                    characters_in_line += 1;
                    if characters_in_line > max_characters {
                        output.push(newline.clone());
                        characters_in_line = 0;
                    } else {
                        output.push(space.clone());
                    }
                }
            }
        }
        output.push(newline.clone());
    }
    output.join("")
}
