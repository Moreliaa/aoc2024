use std::collections::HashMap;
use num_bigint::BigUint;

pub fn run(input: String) {
    let mut p1 = BigUint::ZERO;
    let mut p2 = BigUint::ZERO;

    for l in input.lines() {
        let mut numbers: HashMap<BigUint, BigUint> = HashMap::new();
        l.split(" ").for_each(|a| {
            let n = a.parse::<BigUint>().unwrap();
            if !numbers.contains_key(&n) {
                numbers.insert(n, BigUint::ZERO + 1 as u32);
            } else {
                let prev = numbers.get(&n).unwrap();
                numbers.insert(n, prev + 1 as u32);
            }
        });
        let mut next_numbers: HashMap<BigUint, BigUint> = HashMap::new();
        for i in 0..75 {
            for (n, count) in numbers.iter() {
                if *n == BigUint::ZERO {
                    let num_to_insert = BigUint::ZERO + 1 as u32;
                    if !next_numbers.contains_key(&num_to_insert) {
                        next_numbers.insert(num_to_insert, count.clone());
                    } else {
                        let prev = next_numbers.get(&num_to_insert).unwrap();
                        next_numbers.insert(num_to_insert, prev + count);
                    }
                    continue;
                } else {
                    
                    let mut digits = 1;
                    let mut shift: BigUint = BigUint::ZERO + 10 as u32;
                    while n / shift.clone() >= BigUint::ZERO + 1 as u32 {
                        shift *= 10 as u32;
                        digits += 1;
                    }
                    if digits % 2 == 0 {
                        let n_as_str = n.to_string();
                        let n0 = n_as_str[0..digits / 2].parse::<BigUint>().unwrap();
                        let n1 = n_as_str[digits / 2..digits].parse::<BigUint>().unwrap();
                        let num_to_insert = n0;
                        if !next_numbers.contains_key(&num_to_insert) {
                            next_numbers.insert(num_to_insert, count.clone());
                        } else {
                            let prev = next_numbers.get(&num_to_insert).unwrap();
                            next_numbers.insert(num_to_insert, prev + count);
                        }
                        let num_to_insert = n1;
                        if !next_numbers.contains_key(&num_to_insert) {
                            next_numbers.insert(num_to_insert, count.clone());
                        } else {
                            let prev = next_numbers.get(&num_to_insert).unwrap();
                            next_numbers.insert(num_to_insert, prev + count);
                        }
                        continue;
                    }
                }

                let num_to_insert = n * 2024 as u32;
                if !next_numbers.contains_key(&num_to_insert) {
                    next_numbers.insert(num_to_insert, count.clone());
                } else {
                    let prev = next_numbers.get(&num_to_insert).unwrap();
                    next_numbers.insert(num_to_insert, prev + count);
                }
            }
            numbers = next_numbers;
            next_numbers = HashMap::new();

            if i == 24 {
                for (_, val) in numbers.iter() {
                    p1 += val;
                }
            }
        }

        for (_, val) in numbers {
            p2 += val;
        }
    }

   


    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}