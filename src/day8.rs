use std::cmp::Ordering;

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct Distance(f64);
impl Eq for Distance {}
impl Ord for Distance {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

fn distance(a: (usize, usize, usize), b: (usize, usize, usize)) -> Distance {
    let xd = a.0 - b.0;
    let yd = a.1 - b.1;
    let zd = a.2 - b.2;

    Distance(((xd.pow(2) + yd.pow(2) + zd.pow(2)) as f64).sqrt())
}

fn get_circuit_sizes(connections: &[bool], num_junction_boxes: usize) -> Vec<usize> {
    let mut visited = vec![false; num_junction_boxes];
    fn get_circuit_connected_to(
        junction_idx: usize,
        connections: &[bool],
        num_junction_boxes: usize,
        visited: &mut [bool],
    ) -> Vec<usize> {
        visited[junction_idx] = true;
        let row_start = junction_idx * num_junction_boxes;
        let mut neighbors: Vec<_> = connections[row_start..row_start + num_junction_boxes]
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, connected)| connected)
            .map(|(neighbor_id, _)| neighbor_id % num_junction_boxes)
            .filter(|&neighbor_id| !visited[neighbor_id])
            .collect();
        for &neighbor in &neighbors {
            visited[neighbor] = true;
        }
        for neighbor in neighbors.clone() {
            neighbors.extend(get_circuit_connected_to(
                neighbor,
                connections,
                num_junction_boxes,
                visited,
            ));
        }
        neighbors
    }
    (0..num_junction_boxes)
        .filter_map(|junction_idx| {
            let neighbors = get_circuit_connected_to(
                junction_idx,
                connections,
                num_junction_boxes,
                &mut visited,
            );
            if neighbors.is_empty() {
                None
            } else {
                Some(neighbors.len() + 1)
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let junction_boxes: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(",").map(|coord| coord.parse().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let mut distances = Vec::new();
    for a in 0..junction_boxes.len() - 1 {
        for b in a + 1..junction_boxes.len() {
            distances.push((a, b, distance(junction_boxes[a], junction_boxes[b])));
        }
    }
    distances.sort_by_key(|&(_, _, distance)| distance);

    let mut connections = vec![false; junction_boxes.len() * junction_boxes.len()];
    for &(a, b, _) in distances.iter().take(1000) {
        connections[a * junction_boxes.len() + b] = true;
        connections[b * junction_boxes.len() + a] = true;
    }
    let mut circuit_sizes = get_circuit_sizes(&connections, junction_boxes.len());
    circuit_sizes.sort_by_key(|&size| std::cmp::Reverse(size));
    circuit_sizes.into_iter().take(3).product()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let junction_boxes: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(",").map(|coord| coord.parse().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let mut distances = Vec::new();
    for a in 0..junction_boxes.len() - 1 {
        for b in a + 1..junction_boxes.len() {
            distances.push((a, b, distance(junction_boxes[a], junction_boxes[b])));
        }
    }
    distances.sort_by_key(|&(_, _, distance)| distance);

    let mut circuits = Vec::new();
    let mut junction_to_circuit = vec![-1isize; junction_boxes.len()];
    for &(a, b, _) in distances.iter() {
        let a_circuit = junction_to_circuit[a];
        let b_circuit = junction_to_circuit[b];
        if a_circuit < 0 && b_circuit < 0 {
            circuits.push(vec![a, b]);
            junction_to_circuit[a] = circuits.len() as isize - 1;
            junction_to_circuit[b] = circuits.len() as isize - 1;
            continue;
        }
        if a_circuit == b_circuit {
            continue;
        }
        let circuit_joined = if a_circuit < 0 {
            circuits[b_circuit as usize].push(a);
            junction_to_circuit[a] = b_circuit;
            b_circuit
        } else if b_circuit < 0 {
            circuits[a_circuit as usize].push(b);
            junction_to_circuit[b] = a_circuit;
            a_circuit
        } else {
            for &junction_id in &circuits[b_circuit as usize] {
                junction_to_circuit[junction_id] = a_circuit;
            }
            let mut b_junction_boxes = std::mem::take(&mut circuits[b_circuit as usize]);
            circuits[a_circuit as usize].append(&mut b_junction_boxes);
            a_circuit
        };
        if circuits[circuit_joined as usize].len() == junction_boxes.len() {
            // All junction boxes are part of a single circuit.
            return junction_boxes[a].0 * junction_boxes[b].0;
        }
    }
    unreachable!()
}
