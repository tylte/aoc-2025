fn part1() -> i32 {
    let input = include_str!("../input.txt");
    let mut solution = 0;

    let mut dial = 50;

    for line in input.lines() {
        let (side, distance) = line.split_at(1);
        let distance: i32 = distance.parse::<i32>().unwrap() % 100;

        if side == "L" {
            dial -= distance;
            if dial < 0 {
                dial += 100;
            }
        } else if side == "R" {
            dial += distance;
            if dial > 99 {
                dial -= 100;
            }
        } else {
            unreachable!("where are we going !?");
        }

        if dial == 0 {
            solution += 1;
        }
    }

    return solution;
}

fn part2() -> i32 {
    let input = include_str!("../input.txt");
    let mut solution = 0;

    let mut dial = 50;

    for line in input.lines() {
        let (side, distance) = line.split_at(1);
        let distance: i32 = distance.parse().unwrap();

        // not the greatest but works (:
        if side == "L" {
            for _ in 0..distance {
                dial -= 1;
                if dial == 0 {
                    solution += 1;
                } else if dial < 0 {
                    dial += 100;
                }
            }
        } else if side == "R" {
            for _ in 0..distance {
                dial += 1;
                if dial > 99 {
                    dial -= 100;
                    solution += 1;
                }
            }
        } else {
            unreachable!("where are we going !?");
        }
    }

    return solution;
}

fn main() {
    dbg!(part1());
    dbg!(part2());
}
