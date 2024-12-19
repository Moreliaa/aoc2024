use std::collections::{HashMap, HashSet};
use fancy_regex::Regex;

pub fn run(input: String) {
    let mut s = input.split("\n\n").into_iter();
    let towels: HashSet<String> = s.next().unwrap().split(", ").map(|a| a.to_string()).collect();
    let towels_string = towels.clone().into_iter().collect::<Vec<String>>().join("|");
    let towels_pattern = format!(r"^({})+$", towels_string);
    println!("{towels_pattern}");
    let towels_rx = Regex::new(towels_pattern.as_str()).unwrap();
    let mut max_token_len = 0;
    for t in towels.iter() {
        if t.len() > max_token_len {
            max_token_len = t.len();
        }
    }

    println!("Max token {max_token_len}");

    let designs: &str = s.next().unwrap();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut solved: HashMap<&str, i64> = HashMap::new();
    for d in designs.lines() {
        
        if let Some(_) = towels_rx.find(d).unwrap() {
            println!("OK: {d}");
            p1 += 1;
            p2 += check_path(d, &d.chars().collect(), &towels, &mut solved, max_token_len, 0);
        } else {
            println!("NOT OK: {d}");
        }
    }
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

pub fn check_path<'a>(d_as_str: &'a str, d: &Vec<char>, towels: &HashSet<String>, solved: &mut HashMap<&'a str, i64>, max_token_len: usize, mut idx: usize) -> i64 {
    let d_slice = &d_as_str[idx..];
    if let Some(memo) = solved.get(d_slice) {
        return *memo;
    }
    let mut result = 0;
    let mut token = String::new();
    while idx < d.len() && token.len() <= max_token_len {
        token.push(d[idx]);
        idx += 1;
        if towels.contains(&token) {
            if idx == d.len() {
                result += 1;
            } else  {
                let sub_result = check_path(d_as_str, d, towels, solved, max_token_len, idx);
                result += sub_result;
                solved.insert(&d_as_str[idx..], sub_result);
            }
        }
    }
    return result;
}