use fancy_regex::Regex;

pub fn run(input: String) {
    let mut reg: Vec<i64> = vec![];
    let mut program: Vec<i64> = vec![];

    let rx_reg = Regex::new(r"Register .: (\d+)").unwrap();
    let rx_prog = Regex::new(r"Program: (.+)").unwrap();
    for l in input.lines() {
        if let Some(cap) = rx_reg.captures(l).unwrap() {
            reg.push(cap.get(1).unwrap().as_str().parse::<i64>().unwrap());
        } else if let Some(cap) = rx_prog.captures(l).unwrap() {
            program = cap.get(1).unwrap().as_str().split(",").map(|a| a.parse::<i64>().unwrap()).collect();
        }
    }

    let p1 = run_prog(reg.clone(), &program);
    let mut pt2_out: Vec<Vec<i64>> = vec![vec![0]];
    let target_pt2 = program;
    let mut idx = target_pt2.len() as i64 - 1;
    while idx >= 0 {
        let mut next_out: Vec<i64> = vec![];
        for perm in pt2_out.last().unwrap().iter() {
            let a_base = *perm;
            'a: for a_add in 0..9 {
                let a = a_base * 8 + a_add;
                let pt2_digits = run_prog_pt2(a);
                for (i_d, d) in pt2_digits.iter().enumerate() {
                    if *d != target_pt2[idx as usize + i_d] {
                        continue 'a;
                    }
                }
                next_out.push(a);
            }
        }
        pt2_out.push(next_out);
        idx -= 1;
    }
    println!("{:?}", pt2_out);
    let mut p2_vec = pt2_out.last().unwrap().clone();
    p2_vec.sort();
    let p2 = p2_vec[0];
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn run_prog_pt2(mut a: i64) -> Vec<i64> {
    let mut prog: Vec<i64> = vec![];
    let orig_a = a;
    while a > 0 {
        let next = ((((a % 8) ^ 5 ) ^ (a / 2_i64.pow((a as u32 % 8) ^ 5))) ^ 6) % 8;
        prog.push(next);
        a /= 8;
    }
    if orig_a == 0 {
       prog.push(0);
    }
    return prog;
}

fn run_prog(mut reg: Vec<i64>, program: &Vec<i64>) -> String {
    let mut ptr = 0;
    let mut outputs: Vec<i64> = vec![];
    loop {
        if ptr >= program.len() {
            break;
        }
        let opcode = program[ptr];
        let operand = program[ptr + 1];
        ptr += 2;
        let combo = match operand {
            0 | 1 | 2 | 3 => operand,
            4 => reg[0],
            5 => reg[1],
            6 => reg[2],
            7 => break,
            _ => panic!(),
        };
        
        match opcode {
            0 => { // adv
                reg[0] = reg[0] / 2_i64.pow(combo as u32);
            },
            1 => { // bxl
                reg[1] = reg[1] ^ operand;
            },
            2 => { // bst
                reg[1] = combo % 8;
            },
            3 => { // jnz
                if reg[0] != 0 {
                    ptr = operand as usize;
                }
            },
            4 => { // bxc
                reg[1] = reg[1] ^ reg[2];
            },
            5 => { // out
                println!("{}", reg[0]);
                outputs.push(combo % 8);
            },
            6 => { // bdv
                reg[1] = reg[0] / 2_i64.pow(combo as u32);
            },
            7 => { // cdv
                reg[2] = reg[0] / 2_i64.pow(combo as u32);
            },
            _ => panic!(),
        };
    }
    return outputs.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(",");
}