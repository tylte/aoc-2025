fn q1() -> i64 {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|bank| {
            let mut max = 0;
            for (idx, a) in bank.chars().enumerate() {
                for b in bank.chars().skip(idx + 1) {
                    let digit = a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap();
                    if digit > max {
                        max = digit;
                    }
                }
            }
            max as i64
        })
        .sum()
}

fn q2() -> i64 {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|bank| {
            let bank_numbers: Vec<_> = bank
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect();

            let mut value = 0;
            let mut current_idx = 0;
            for digit_left in (0..12).rev() {
                let (new_idx, max) = bank_numbers
                    .iter()
                    .enumerate()
                    .skip(current_idx)
                    .rev()
                    .skip(digit_left)
                    .reduce(|(idx_a, a), (idx_b, b)| {
                        // taking the number with the lesser idx if equal
                        if a > b || (a == b && idx_a < idx_b) {
                            (idx_a, a)
                        } else {
                            (idx_b, b)
                        }
                    })
                    .unwrap();

                current_idx = new_idx + 1;
                value += *max * 10_i64.pow(digit_left as u32);
            }

            value
        })
        .sum()
}

fn main() {
    dbg!(q1());
    dbg!(q2());
}
