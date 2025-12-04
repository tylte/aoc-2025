const OFFSETS: &[(isize, isize)] = &[
    (1, 0),
    (1, 1),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn q1() -> i32 {
    let input = include_str!("../input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sol = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != '@' {
                continue;
            }

            let adjacent_roll_count: usize = OFFSETS
                .iter()
                .filter_map(|&(offset_x, offset_y)| {
                    let x = x.checked_add_signed(offset_x)?;
                    let y = y.checked_add_signed(offset_y)?;

                    if *grid.get(x)?.get(y)? == '@' {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum();

            if adjacent_roll_count < 4 {
                sol += 1;
            }
        }
    }

    sol
}

fn q2() -> i32 {
    let input = include_str!("../input.txt");

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sol = 0;

    loop {
        let mut roll_to_remove = vec![];
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] != '@' {
                    continue;
                }

                let adjacent_roll_count: usize = OFFSETS
                    .iter()
                    .filter_map(|&(offset_x, offset_y)| {
                        let x = x.checked_add_signed(offset_x)?;
                        let y = y.checked_add_signed(offset_y)?;

                        if *grid.get(x)?.get(y)? == '@' {
                            Some(1)
                        } else {
                            None
                        }
                    })
                    .sum();

                if adjacent_roll_count < 4 {
                    roll_to_remove.push((x, y));
                }
            }
        }

        if roll_to_remove.is_empty() {
            break;
        }

        sol += roll_to_remove.len();

        for (x, y) in roll_to_remove {
            grid[x][y] = '.';
        }
    }

    sol as i32
}

fn main() {
    dbg!(q1());
    dbg!(q2());
}
