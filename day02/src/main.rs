fn part1() -> i64 {
    let input = include_str!("../input.txt");

    input
        .split(",")
        .map(|ids| {
            let (start, end) = ids.trim().split_once('-').unwrap();

            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();

            let mut res = 0;

            for i in start..=end {
                let str_i = i.to_string();
                if str_i.len() % 2 != 0 {
                    continue;
                }

                let (left, right) = str_i.split_at(str_i.len() / 2);
                res += if left == right { i } else { 0 };
            }

            res
        })
        .sum()
}

fn part2() -> i64 {
    let input = include_str!("../input.txt");

    input
        .split(",")
        .map(|ids| {
            let (start, end) = ids.trim().split_once('-').unwrap();

            let start: i64 = start.parse().unwrap();
            let end: i64 = end.parse().unwrap();

            let mut res = 0;

            for i in start..=end {
                let str_i = i.to_string();
                let chars = str_i.chars().into_iter().collect::<Vec<_>>();

                for idx in 1..str_i.len() {
                    if chars
                        .chunks(idx)
                        .zip(chars.chunks(idx).skip(1))
                        .all(|(a, b)| a == b)
                    {
                        res += i;
                        break;
                    };
                }
            }

            res
        })
        .sum()
}

fn main() {
    dbg!(part1());
    dbg!(part2());
}
