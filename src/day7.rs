use fancy_regex::Regex;

pub fn run(input: String) {

    let mut p1 = 0;
    let mut p2 = 0;

    let rx = Regex::new(r"(\d+): (.+)").unwrap();
    for l in input.lines() {
        let m = rx.captures(l).unwrap().unwrap();
        let cali = m.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let values: Vec<i64> = m.get(2).unwrap().as_str().split(" ").map(|a| a.parse::<i64>().unwrap()).collect();
        
        if func_pt1(cali, values[0], '*', 1, &values) {
            p1 += cali;
        }

        if func_pt2(cali, values[0], '*', 1, &values) {
            p2 += cali;
        }
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn func_pt1(target: i64, current: i64, op: char, idx: usize, values: &Vec<i64>) -> bool {
    if idx >= values.len() {
        return current == target;
    }
    
    let v = values[idx];
    let next = match op {
        '+' => current + v,
        '*' => current * v,
        _ => panic!(),
    };

    if next <= target && func_pt1(target, next, '*',  idx + 1, values) {
        return true;
    } else if op == '*' && func_pt1(target, current, '+',  idx, values) {
        return true;
    } else {
        return false;
    }
}

fn func_pt2(target: i64, current: i64, op: char, idx: usize, values: &Vec<i64>) -> bool {
    if idx >= values.len() {
        return current == target;
    }
    
    let v = values[idx];
    let next = match op {
        '+' => current + v,
        '*' => current * v,
        '|' => {
            let mut res = current;
            let mut modder = 1;
            while v / modder > 0 {
                modder *= 10;
                res *= 10;
            }
            res += v;
            res
        },
        _ => panic!(),
    };

    if next <= target && func_pt2(target, next, '*',  idx + 1, values) {
        return true;
    } else if op == '*' && func_pt2(target, current, '+',  idx, values) {
        return true;
    } else if op == '+' && func_pt2(target, current, '|',  idx, values) {
        return true;
    } else {
        return false;
    }
}