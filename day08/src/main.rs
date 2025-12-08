use std::collections::HashSet;

struct JBox {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for JBox {
    fn from(value: &str) -> Self {
        let mut iter = value.split(",");
        Self {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
            z: iter.next().unwrap().parse().unwrap(),
        }
    }
}

impl JBox {
    fn distance(&self, other: &JBox) -> f64 {
        ((self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2) as f64)
            .sqrt()
    }
}

#[derive(Debug)]
struct Connection {
    jbox_a_idx: usize,
    jbox_b_idx: usize,
    distance: f64,
}

fn q1() -> usize {
    let input = include_str!("../input.txt");

    let jboxes = input.lines().map(JBox::from).collect::<Vec<_>>();

    let mut distances = vec![];

    for (jbox_a_idx, jbox_a) in jboxes.iter().enumerate() {
        for (jbox_b_idx, jbox_b) in jboxes.iter().enumerate().skip(jbox_a_idx + 1) {
            let distance = jbox_a.distance(jbox_b);

            distances.push(Connection {
                jbox_a_idx,
                jbox_b_idx,
                distance,
            })
        }
    }

    distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut circuits: Vec<HashSet<usize>> = vec![];

    for connection in distances.iter().take(1000) {
        let circuits_with_jboxes = circuits
            .iter()
            .enumerate()
            .filter_map(|(idx, jbox_indexes)| {
                if jbox_indexes.contains(&connection.jbox_a_idx)
                    || jbox_indexes.contains(&connection.jbox_b_idx)
                {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if circuits_with_jboxes.is_empty() {
            circuits.push(HashSet::from_iter([
                connection.jbox_a_idx,
                connection.jbox_b_idx,
            ]));
        } else if circuits_with_jboxes.len() == 1 {
            let jbox_indexes = &mut circuits[circuits_with_jboxes[0]];
            jbox_indexes.insert(connection.jbox_a_idx);
            jbox_indexes.insert(connection.jbox_b_idx);
        } else if circuits_with_jboxes.len() == 2 {
            let circuit = circuits.remove(circuits_with_jboxes[1]);
            let jbox_indexes = &mut circuits[circuits_with_jboxes[0]];
            jbox_indexes.insert(connection.jbox_a_idx);
            jbox_indexes.insert(connection.jbox_b_idx);
            jbox_indexes.extend(circuit);
        }
    }

    let mut circuits_len = circuits
        .iter()
        .map(|circuit| circuit.len())
        .collect::<Vec<_>>();

    circuits_len.sort_by(|a, b| b.cmp(a));

    circuits_len
        .into_iter()
        .take(3)
        .reduce(|a, b| a * b)
        .unwrap_or_default()
}

fn q2() -> usize {
    let input = include_str!("../input.txt");

    let jboxes = input.lines().map(JBox::from).collect::<Vec<_>>();

    let mut distances = vec![];

    for (jbox_a_idx, jbox_a) in jboxes.iter().enumerate() {
        for (jbox_b_idx, jbox_b) in jboxes.iter().enumerate().skip(jbox_a_idx + 1) {
            let distance = jbox_a.distance(jbox_b);

            distances.push(Connection {
                jbox_a_idx,
                jbox_b_idx,
                distance,
            })
        }
    }

    distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut circuits: Vec<HashSet<usize>> = vec![];

    let mut sol = 0;

    for connection in distances {
        let circuits_with_jboxes = circuits
            .iter()
            .enumerate()
            .filter_map(|(idx, jbox_indexes)| {
                if jbox_indexes.contains(&connection.jbox_a_idx)
                    || jbox_indexes.contains(&connection.jbox_b_idx)
                {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if circuits_with_jboxes.is_empty() {
            circuits.push(HashSet::from_iter([
                connection.jbox_a_idx,
                connection.jbox_b_idx,
            ]));
        } else if circuits_with_jboxes.len() == 1 {
            let jbox_indexes = &mut circuits[circuits_with_jboxes[0]];
            jbox_indexes.insert(connection.jbox_a_idx);
            jbox_indexes.insert(connection.jbox_b_idx);
        } else if circuits_with_jboxes.len() == 2 {
            let circuit = circuits.remove(circuits_with_jboxes[1]);
            let jbox_indexes = &mut circuits[circuits_with_jboxes[0]];
            jbox_indexes.insert(connection.jbox_a_idx);
            jbox_indexes.insert(connection.jbox_b_idx);
            jbox_indexes.extend(circuit);
        }

        if circuits.len() == 1 && circuits[0].len() == jboxes.len() {
            sol =
                jboxes[connection.jbox_a_idx].x as usize * jboxes[connection.jbox_b_idx].x as usize;
            break;
        }
    }

    sol
}

fn main() {
    dbg!(q1());
    dbg!(q2());
}
