fn q1() -> usize {
    let input = include_str!("../input.txt");

    let (ranges, list) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|range| {
            let (low, high) = range.split_once("-").unwrap();
            let low = low.parse::<usize>().unwrap();
            let high = high.parse::<usize>().unwrap();

            low..=high
        })
        .collect::<Vec<_>>();

    list.lines()
        .filter(|id| {
            let id = id.parse::<usize>().unwrap();
            ranges.iter().any(|range| range.contains(&id))
        })
        .count()
}

fn q2() -> usize {
    let input = include_str!("../input.txt");

    let mut sol = 0;

    let (ranges, _list) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|range| {
            let (low, high) = range.split_once("-").unwrap();
            let low = low.parse::<usize>().unwrap();
            let high = high.parse::<usize>().unwrap();

            (high, low)
        })
        .collect::<Vec<_>>();

    while !ranges.is_empty() {
        let (mut hi_a, mut lo_a) = ranges.pop().unwrap();
        let mut to_delete = vec![];

        loop {
            for (idx, &(hi_b, lo_b)) in ranges.iter().enumerate() {
                // if ranges overlap, expends them and dedup
                if (lo_a..=hi_a).contains(&hi_b)
                    || (lo_a..=hi_a).contains(&lo_b)
                    || (lo_b..=hi_b).contains(&lo_a)
                    || (lo_b..=hi_b).contains(&hi_a)
                {
                    to_delete.push(idx);
                    hi_a = hi_a.max(hi_b);
                    lo_a = lo_a.min(lo_b);
                }
            }

            if to_delete.is_empty() {
                break;
            }

            ranges = ranges
                .into_iter()
                .enumerate()
                .filter_map(|(idx, range)| {
                    if to_delete.contains(&idx) {
                        None
                    } else {
                        Some(range)
                    }
                })
                .collect();

            to_delete.clear();
        }

        sol += hi_a - lo_a + 1;
    }

    sol
}

fn main() {
    dbg!(q1());
    dbg!(q2());
}
