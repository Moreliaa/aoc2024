pub fn run(input: String) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines().into_iter() {
        let values: Vec<i32> = line.split(" ").map(|l| l.parse::<i32>().unwrap()).collect();
        let r = check(values, false);
        p1 += r.0;
        p2 += r.1;
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn check(values: Vec<i32>, failed_once: bool) -> (i32, i32) {
    let is_asc = values[0] - values[1] < 0;
    for i in 0..values.len() - 1 {
        let v0 = values[i];
        let v1 = values[i + 1];
        let diff = v0 - v1;
        if (is_asc && diff >= 0) || (!is_asc && diff <= 0) || diff.abs() > 3 {
            if failed_once {
                return (0, 0);
            }

            let mut result = (0, 0);
            for idx in [i, i + 1, 0] {
                let mut splice1 = values.clone();
                splice1.splice(idx..idx + 1, []);
                result = check(splice1, true);

                if result.1 == 1 {
                    break;
                }
            }
            return (result.0 as i32, result.1 as i32);
        }
    }
    let p1 = if !failed_once { 1 } else { 0 };
    return (p1, 1);
}
