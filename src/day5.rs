use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: String) {
    let mut split = input.split("\n\n");
    let rules: Vec<(i32, i32)> = split
        .next()
        .unwrap()
        .split("\n")
        .map(|a| {
            let mut a_split = a.split("|");
            (
                a_split.next().unwrap().parse::<i32>().unwrap(),
                a_split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let updates: Vec<Vec<i32>> = split
        .next()
        .unwrap()
        .split("\n")
        .map(|a| {
            a.split(",")
                .filter(|a| a.len() > 0)
                .map(|a| a.parse::<i32>().unwrap())
                .collect()
        })
        .filter(|a: &Vec<i32>| a.len() > 0)
        .collect();

    let mut prio_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for r in rules {
        if !prio_map.contains_key(&r.0) {
            prio_map.insert(r.0, HashSet::new());
        }
        if !prio_map.contains_key(&r.1) {
            prio_map.insert(r.1, HashSet::new());
        }
        prio_map.get_mut(&r.0).unwrap().insert(r.1);
    }
    let mut p1 = 0;
    let mut p2 = 0;

    for update in updates {
        if check_ordered(&prio_map, &update) {
            p1 += update[update.len() / 2];
        } else {
            let mut cloned = update.clone();
            cloned.sort_by(|a, b| {
                let a_prio = prio_map.get(a).unwrap();
                if a_prio.contains(&b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            p2 += cloned[cloned.len() / 2];
        }
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn check_ordered(prio_map: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    for (i, u) in update.iter().enumerate() {
        let u_prio = prio_map.get(u);
        if u_prio == None {
            continue;
        }
        for j in i + 1..update.len() {
            let b = update[j];
            if !u_prio.unwrap().contains(&b) {
                return false;
            }
        }
    }
    return true;
}
