use fancy_regex::Regex;

pub fn run(input: String) {
    let mut p1 = 0;
    let mut p2 = 0;
    let rx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let split = input.split(r"do()");
    for s in split {
        let mut s_split = s.split(r"don't()").into_iter();
        let first = s_split.next().unwrap();
        let all = rx.captures_iter(first);
        for m in all {
            let m = m.unwrap();
            let result = m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap();
            p1 += result;
            p2 += result;
        }

        while let Some(content) = s_split.next() {
            let all = rx.captures_iter(content);
            for m in all {
                let m = m.unwrap();
                p1 += m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap();
            }
        }
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
