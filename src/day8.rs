use aoc_runner_derive::aoc;

#[derive(Debug)]
struct CircuitGraph {
    circuits: Vec<Vec<usize>>,
    junction_box_to_circuit: Vec<Option<usize>>,
}
impl CircuitGraph {
    fn new(max_junction_boxes: usize) -> Self {
        CircuitGraph {
            circuits: Vec::new(),
            junction_box_to_circuit: vec![None; max_junction_boxes],
        }
    }

    // join a and b returning the id of the circuit that was joined.
    fn join(&mut self, a: usize, b: usize) -> usize {
        let a_circuit = self.junction_box_to_circuit[a];
        let b_circuit = self.junction_box_to_circuit[b];
        match (a_circuit, b_circuit) {
            (None, None) => {
                self.circuits.push(vec![a, b]);
                let new_circuit = self.circuits.len() - 1;
                self.junction_box_to_circuit[a] = Some(new_circuit);
                self.junction_box_to_circuit[b] = Some(new_circuit);
                new_circuit
            }
            (Some(a_circuit), Some(b_circuit)) if a_circuit == b_circuit => a_circuit,
            (Some(a_circuit), None) => {
                self.circuits[a_circuit].push(b);
                self.junction_box_to_circuit[b] = Some(a_circuit);
                a_circuit
            }
            (None, Some(b_circuit)) => {
                self.circuits[b_circuit].push(a);
                self.junction_box_to_circuit[a] = Some(b_circuit);
                b_circuit
            }
            (Some(a_circuit), Some(b_circuit)) => {
                for &junction_id in &self.circuits[b_circuit] {
                    self.junction_box_to_circuit[junction_id] = Some(a_circuit);
                }
                let mut b_junction_boxes = std::mem::take(&mut self.circuits[b_circuit]);
                self.circuits[a_circuit].append(&mut b_junction_boxes);
                a_circuit
            }
        }
    }

    fn get_circuit(&self, circuit: usize) -> &[usize] {
        &self.circuits[circuit]
    }

    fn circuits(&self) -> impl Iterator<Item = &[usize]> {
        self.circuits
            .iter()
            .filter(|boxes| !boxes.is_empty())
            .map(|boxes| boxes.as_slice())
    }
}

fn distance_squared(a: (usize, usize, usize), b: (usize, usize, usize)) -> usize {
    let x_diff = a.0.abs_diff(b.0);
    let y_diff = a.1.abs_diff(b.1);
    let z_diff = a.2.abs_diff(b.2);
    x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
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
            distances.push((a, b, distance_squared(junction_boxes[a], junction_boxes[b])));
        }
    }
    distances.sort_by_key(|&(_, _, distance)| distance);

    let mut graph = CircuitGraph::new(junction_boxes.len());
    for &(a, b, _) in distances.iter().take(1000) {
        graph.join(a, b);
    }
    let mut circuit_sizes: Vec<_> = graph.circuits().map(|boxes| boxes.len()).collect();
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
            distances.push((a, b, distance_squared(junction_boxes[a], junction_boxes[b])));
        }
    }
    distances.sort_by_key(|&(_, _, distance)| distance);

    let mut graph = CircuitGraph::new(junction_boxes.len());
    for &(a, b, _) in distances.iter() {
        let circuit = graph.join(a, b);
        if graph.get_circuit(circuit).len() == junction_boxes.len() {
            // All junction boxes are part of a single circuit.
            return junction_boxes[a].0 * junction_boxes[b].0;
        }
    }
    unreachable!()
}
