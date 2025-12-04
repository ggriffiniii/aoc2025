use aoc_runner_derive::aoc;

fn max_joltage(input: &str, num_batteries: usize) -> usize {
    let input = input.as_bytes();
    let mut result = 0;
    let mut next_battery_start_idx = 0;
    for i in 0..num_batteries {
        let mut max_idx = next_battery_start_idx;

        // The number of batteries that still need to be turned on after this one.
        let batteries_after = num_batteries - (i + 1);

        let search_start = max_idx + 1;
        let search_end = input.len() - batteries_after;
        let search_range = search_start..search_end;

        for i in search_range {
            if input[max_idx] < input[i] {
                max_idx = i;
            }
        }
        result = result * 10 + (input[max_idx] - b'0') as usize;
        next_battery_start_idx = max_idx + 1;
    }
    result
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    input.split("\n").map(|line| max_joltage(line, 2)).sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    input.split("\n").map(|line| max_joltage(line, 12)).sum()
}
