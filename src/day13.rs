use fancy_regex::Regex;

pub fn run(input: String) {
    let mut p1 = 0;
    let mut p2 = 0;

    let rx_ab = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let rx_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut lines = input.lines().into_iter();


    loop {
        let l_a = lines.next();
        if l_a == None {
            break;
        }
        let l_a = l_a.unwrap();
        let l_b = lines.next().unwrap();
        let l_prize = lines.next().unwrap();
        lines.next();

        let m_a = rx_ab.captures(l_a).unwrap().unwrap();
        let m_b = rx_ab.captures(l_b).unwrap().unwrap();
        let m_p = rx_prize.captures(l_prize).unwrap().unwrap();
        let offset_ax = m_a.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let offset_ay = m_a.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let offset_bx = m_b.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let offset_by = m_b.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let prize_x = m_p.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let prize_y = m_p.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let numerator_a = prize_y * offset_bx - prize_x * offset_by;
        let denominator_a = offset_ay * offset_bx - offset_ax * offset_by;
        let num_a =  numerator_a / denominator_a;
        let numerator_b = prize_x - num_a * offset_ax;
        let num_b = numerator_b / offset_bx;
        if numerator_a % denominator_a == 0 && numerator_b % offset_bx == 0 {
            p1 += num_a * 3 + num_b * 1;
        }

        let prize_offset = 10000000000000;
        let prize_x = prize_x + prize_offset;
        let prize_y = prize_y + prize_offset;
        let numerator_a = prize_y * offset_bx - prize_x * offset_by;
        let denominator_a = offset_ay * offset_bx - offset_ax * offset_by;
        let num_a =  numerator_a / denominator_a;
        let numerator_b = prize_x - num_a * offset_ax;
        let num_b = numerator_b / offset_bx;
        if numerator_a % denominator_a == 0 && numerator_b % offset_bx == 0 {
            p2 += num_a * 3 + num_b * 1;
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}