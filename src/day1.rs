use fancy_regex::Regex;

pub fn run(input: String) {
    let mut l: Vec<i32> = vec![];
    let mut r: Vec<i32> = vec![];
    let rx = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    for line in  input.lines().into_iter() {
        let m = rx.captures(line).unwrap().unwrap();
        l.push(m.get(1).unwrap().as_str().parse::<i32>().unwrap());
        r.push(m.get(2).unwrap().as_str().parse::<i32>().unwrap());
    }
    l.sort();
    r.sort();

    let mut p1 = 0;
    let mut p2 = 0;
    for (idx,  a) in l.iter().enumerate() {
        p1 += (*a - r[idx]).abs();
        
        let count = r.iter().filter(|b| **b == *a).count();
        p2 += *a * count as i32;
    }
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}