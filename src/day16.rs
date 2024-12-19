use aoc_lib::map2d::Map2D;
use std::{collections::{HashMap, HashSet}, i32};
use std::collections::BinaryHeap;
use std::cmp::Reverse;
pub fn run(input: String) {
    let map = Map2D::from_string(input);
    let x_prev = map.aggregate(|val, x, _| if *val == 'S' { x } else { 0 });
    let y_prev = map.aggregate(|val, _, y| if *val == 'S' { y } else { 0 });
    let facing_prev = '>';

    let mut seen: HashMap<(i32, i32, char), i32> = HashMap::new(); // x,y,dir -> score
    let mut seen_prev: HashMap<(i32, i32, char, i32), Vec<(i32, i32, char, i32)>> = HashMap::new(); // x, y, dir, score || current -> previous
    let mut unseen: HashMap<i32, Vec<(i32,  i32, char)>> = HashMap::new(); // score,x,y,dir
    unseen.insert(0, vec![(x_prev, y_prev, facing_prev)]);

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let movements = ['^', '>', 'v', '<'];

    let mut lowest_end = i32::MAX;
    let mut unseen_scores: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    unseen_scores.push(Reverse(0));


    let mut final_nodes: Vec<(i32, i32, char, i32)> = vec![(0, 0, '.', 0)];

    loop {
        if unseen_scores.peek() == None {
            break;
        }
        let Reverse(lowest_current) = unseen_scores.pop().unwrap();

        let mut next_vec: Vec<(i32, i32, char)> = unseen.remove(&lowest_current).unwrap();
        while next_vec.len() > 0 {
            let next = next_vec.pop().unwrap();
            
            let tile = map.get(next.0, next.1).unwrap();
            match *tile  {
                'E' => {
                    if lowest_current <= lowest_end {
                        lowest_end = lowest_current;
                        final_nodes.push((next.0, next.1, next.2, lowest_end));
                    }
                    continue;
                },
                '#' => {
                    continue;
                },
                _ => {},
            };
            let k = (next.0,next.1,next.2);
            seen.insert(k, lowest_current);

            let mov_idx = movements.iter().position(|a| *a == next.2).unwrap();
            let mov_dir = dirs[mov_idx];
            // straight
            let k = (next.0 + mov_dir.0, next.1 + mov_dir.1, next.2);
            let uk = lowest_current + 1;
            if seen.contains_key(&k) {
                let prev_score = seen.get(&k).unwrap();
                if *prev_score > uk {
                    if unseen.contains_key(&uk) {
                        let v = unseen.get_mut(&uk).unwrap();
                        v.push(k);
                    } else {
                        unseen.insert(uk, vec![k]);
                        unseen_scores.push(std::cmp::Reverse(uk));
                    }
                }
            } else {
                if unseen.contains_key(&uk) {
                    let v = unseen.get_mut(&uk).unwrap();
                    v.push(k);
                } else {
                    unseen.insert(uk, vec![k]);
                    unseen_scores.push(std::cmp::Reverse(uk));
                }
            }
            let k_prev = (next.0, next.1, next.2, lowest_current);
            let k = (next.0 + mov_dir.0, next.1 + mov_dir.1, next.2, lowest_current + 1);
            if seen_prev.contains_key(&k) {
                let v = seen_prev.get_mut(&k).unwrap();
                v.push(k_prev.clone());
            } else {
                seen_prev.insert(k, vec![k_prev.clone()]);
            }
            
            
            // right
            let mov_idx_r = if mov_idx == dirs.len() - 1 { 0 } else { mov_idx + 1 };
            let k = (next.0, next.1, movements[mov_idx_r]);
            let uk = lowest_current + 1000;
            if seen.contains_key(&k) {
                let prev_score = seen.get(&k).unwrap();
                if *prev_score > uk {
                    if unseen.contains_key(&uk) {
                        let v = unseen.get_mut(&uk).unwrap();
                        v.push(k);
                    } else {
                        unseen.insert(uk, vec![k]);
                        unseen_scores.push(std::cmp::Reverse(uk));
                    }
                }
            } else {
                if unseen.contains_key(&uk) {
                    let v = unseen.get_mut(&uk).unwrap();
                    v.push(k);
                } else {
                    unseen.insert(uk, vec![k]);
                    unseen_scores.push(std::cmp::Reverse(uk));
                }
            }

            let k = (next.0, next.1, movements[mov_idx_r], uk);
            if seen_prev.contains_key(&k) {
                let v = seen_prev.get_mut(&k).unwrap();
                v.push(k_prev.clone());
            } else {
                seen_prev.insert(k, vec![k_prev.clone()]);
            }

            // left
            let mov_idx_l = if mov_idx == 0 { dirs.len() - 1 } else { mov_idx - 1 };
            let k = (next.0, next.1, movements[mov_idx_l]);
            let uk = lowest_current + 1000;
            if seen.contains_key(&k) {
                let prev_score = seen.get(&k).unwrap();
                if *prev_score > uk {
                    if unseen.contains_key(&uk) {
                        let v = unseen.get_mut(&uk).unwrap();
                        v.push(k);
                    } else {
                        unseen.insert(uk, vec![k]);
                        unseen_scores.push(std::cmp::Reverse(uk));
                    }
                }
            } else {
                if unseen.contains_key(&uk) {
                    let v = unseen.get_mut(&uk).unwrap();
                    v.push(k);
                } else {
                    unseen.insert(uk, vec![k]);
                    unseen_scores.push(std::cmp::Reverse(uk));
                }
            }

            let k = (next.0, next.1, movements[mov_idx_l], uk);
            if seen_prev.contains_key(&k) {
                let v = seen_prev.get_mut(&k).unwrap();
                v.push(k_prev.clone());
            } else {
                seen_prev.insert(k, vec![k_prev.clone()]);
            }
        }
        unseen.remove(&lowest_current);
    }

    let final_nodes: HashSet<(i32, i32, char, i32)> = final_nodes.into_iter().filter(|a| a.3 == lowest_end).collect();

    let mut nodes_pt2: HashSet<(i32, i32)> = HashSet::new();
    
    for node in final_nodes {
        let mut locations: Vec<(i32, i32, char, i32)> = vec![node];
        while locations.len() > 0 {
            let mut next_locations: Vec<(i32, i32, char, i32)> = vec![];
            for l in &locations {
                nodes_pt2.insert((l.0, l.1));
                if let Some(prev_nodes) = seen_prev.get_mut(l) {
                    next_locations.append(prev_nodes);
                }
            }
            locations = next_locations;
        }
    }

    let p1 = lowest_end;
    let p2 = nodes_pt2.len();
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}