use std::collections::VecDeque;

fn q1() -> usize {
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .filter_map(|(x, line)| line.iter().position(|&c| c == 'S').map(|y| (x, y)))
        .next()
        .unwrap();

    let mut sol = 0;
    let mut beam_indexes = Vec::new();
    beam_indexes.push((start_x, start_y));
    while beam_indexes[0].0 + 1 < grid.len() {
        for _ in 0..beam_indexes.len() {
            let (x, y) = beam_indexes.remove(0);
            if grid[x + 1][y] == '^' {
                sol += 1;
                beam_indexes.push((x + 1, y + 1));
                beam_indexes.push((x + 1, y - 1));
            } else {
                beam_indexes.push((x + 1, y));
            }
        }
        beam_indexes.sort();
        beam_indexes.dedup();
    }
    sol
}

fn q2() -> usize {
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .filter_map(|(x, line)| line.iter().position(|&c| c == 'S').map(|y| (x, y)))
        .next()
        .unwrap();

    let mut beam_indexes = VecDeque::new();

    fn insert_or_increment(
        beam_indexes: &mut VecDeque<(usize, usize, usize)>,
        x: usize,
        y: usize,
        count: usize,
    ) {
        if let Some((_, _, other_count)) = beam_indexes
            .iter_mut()
            .find(|(x_b, y_b, _)| *x_b == x && y == *y_b)
        {
            *other_count += count;
        } else {
            beam_indexes.push_back((x, y, count));
        }
    }

    beam_indexes.push_back((start_x, start_y, 1));
    while beam_indexes[0].0 + 1 < grid.len() {
        for _ in 0..beam_indexes.len() {
            let (x, y, count) = beam_indexes.pop_front().unwrap();
            if grid[x + 1][y] == '^' {
                insert_or_increment(&mut beam_indexes, x + 1, y + 1, count);
                insert_or_increment(&mut beam_indexes, x + 1, y - 1, count);
            } else {
                insert_or_increment(&mut beam_indexes, x + 1, y, count);
            }
        }
    }

    beam_indexes.iter().map(|(_, _, count)| *count).sum()
}

fn main() {
    dbg!(q1());
    dbg!(q2());
}
