use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let (first_line, remaining_input) = input.split_once('\n').unwrap();
    let row_len = first_line.len() + 1; // count '\n'
    let mut beams = vec![false; row_len - 1];
    let starting_point = first_line
        .as_bytes()
        .iter()
        .position(|&b| b == b'S')
        .unwrap();
    beams[starting_point] = true;
    let mut num_splits = 0;
    for (idx, b) in remaining_input.as_bytes().iter().copied().enumerate() {
        if b == b'^' {
            let col = idx % row_len;
            if beams[col] {
                beams[col] = false;
                beams[col - 1] = true;
                beams[col + 1] = true;
                num_splits += 1;
            }
        }
    }
    num_splits
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let (first_line, remaining_input) = input.split_once('\n').unwrap();
    let row_len = first_line.len() + 1; // count '\n'
    let mut beams = vec![0; row_len - 1];
    let starting_point = first_line
        .as_bytes()
        .iter()
        .position(|&b| b == b'S')
        .unwrap();
    beams[starting_point] = 1;
    for (idx, b) in remaining_input.as_bytes().iter().copied().enumerate() {
        if b == b'^' {
            let col = idx % row_len;
            if beams[col] > 0 {
                beams[col - 1] += beams[col];
                beams[col + 1] += beams[col];
                beams[col] = 0;
            }
        }
    }
    beams.iter().sum()
}
