fn q1() -> usize {
    let input = include_str!("../input.txt");

    let homework = input.lines().collect::<Vec<_>>();

    let ops = homework[homework.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(|number| number.parse::<usize>())
                .collect::<Vec<_>>()
        })
        .fold(vec![], |mut acc, values| {
            if acc.is_empty() {
                return values;
            } else {
                for idx in 0..values.len() {
                    if ops[idx] == "+" {
                        acc[idx] += values[idx];
                    } else if ops[idx] == "*" {
                        acc[idx] *= values[idx];
                    }
                }
                return acc;
            }
        })
        .into_iter()
        .sum()
}

fn q2() -> usize {
    let input = include_str!("../input.txt");

    let homework = input.lines().collect::<Vec<_>>();

    let mut ops = homework[homework.len() - 1]
        .chars()
        .enumerate()
        .map(|(i, c)| (i, c, if c == '*' { 1_usize } else { 0_usize }))
        .filter(|(_, c, _)| !c.is_whitespace())
        .collect::<Vec<_>>();

    let idx_and_number = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .collect::<Vec<_>>()
        })
        .fold(vec![], |mut acc, values| {
            // Putting the digit into the numbers
            if acc.is_empty() {
                return values
                    .into_iter()
                    .map(|(idx, c)| (idx, c.to_string()))
                    .collect::<Vec<_>>();
            } else {
                for (idx, value) in values {
                    if !value.is_digit(10) {
                        continue;
                    }
                    if let Some((_, chars)) = acc.iter_mut().find(|(i, _)| *i == idx) {
                        chars.push(value);
                    } else {
                        // idx of the number not in acc yet, inserting it there
                        acc.push((idx, value.to_string()));
                    }
                }
                return acc;
            }
        })
        .into_iter()
        .map(|(idx, chars)| (idx, chars.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    for (idx, number) in idx_and_number {
        let next_idx = ops
            .iter()
            .position(|(i, _, _)| (*i > idx))
            .unwrap_or(ops.len());

        if let Some(entry) = ops.get_mut(next_idx - 1) {
            if entry.1 == '+' {
                entry.2 += number;
            } else if entry.1 == '*' {
                entry.2 *= number;
            }
        }
    }

    ops.into_iter().map(|(_, _, number)| number).sum()
}

fn main() {
    dbg!(q1());
    dbg!(q2());
}
