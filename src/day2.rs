use aoc_runner_derive::{aoc, aoc_generator};

use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct RangeSet(Vec<RangeInclusive<usize>>);
impl RangeSet {
    fn new(input: &str) -> Self {
        let mut ranges: Vec<_> = input
            .split(',')
            .map(|r| {
                let (start, end) = r.split_once('-').unwrap();
                RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
            })
            .collect();

        // Sort the ranges by start key
        ranges.sort_by_key(|r| *r.start());

        // Assert none of the ranges overlap
        for pair in ranges.windows(2) {
            if *pair[0].end() >= *pair[1].start() {
                panic!("{:?} overlaps with {:?}", pair[0], pair[1]);
            }
        }
        RangeSet(ranges)
    }

    fn ranges(&self) -> &[RangeInclusive<usize>] {
        &self.0
    }
}

// repeat_seq(87, 1) -> 8787
// repeat_seq(123, 2) -> 123123123
// repeat_seq(67, 4) -> 6767676767
fn repeat_seq(seq: usize, times: usize) -> usize {
    let num_digits = seq.ilog10() + 1;
    let shift = 10usize.pow(num_digits);
    let mut result = seq;
    for _ in 0..times {
        result = result * shift + seq;
    }
    result
}

// return the upper half digits of a value.
// get_seq(1234)   -> 12
// get_seq(987654) -> 987
//
// if the value has an odd number of digits round up the half and return the
// lowest value that consumes that many digits.
// get_seq(987)   -> 10
// get_seq(12345) -> 100
fn get_seq(x: usize) -> usize {
    let num_digits = x.ilog10() + 1;
    let base = 10usize.pow(num_digits / 2);
    if num_digits.is_multiple_of(2) {
        x / base
    } else {
        base
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> RangeSet {
    RangeSet::new(input)
}

#[aoc(day2, part1)]
fn part1(input: &RangeSet) -> usize {
    input
        .ranges()
        .into_iter()
        .map(|range| -> usize {
            let start_seq = get_seq(*range.start());
            let end_seq = get_seq(*range.end());

            (start_seq..=end_seq)
                .map(|seq| repeat_seq(seq, 1))
                .filter(|value| range.contains(value))
                .sum()
        })
        .sum()
}

// determine if `x` consists of a repeating sequence of digits.
fn is_repeating_seq(x: usize) -> bool {
    let num_digits = x.ilog10() + 1;
    let sequence_digits =
        (1..=num_digits / 2).filter(|&seq_len| num_digits.is_multiple_of(seq_len));
    for seq_digits in sequence_digits {
        let mut x = x;
        let shift = 10usize.pow(seq_digits);
        let seq_pattern = x / 10usize.pow(num_digits - seq_digits);
        loop {
            if x == 0 {
                return true;
            }
            if x % shift != seq_pattern {
                break;
            }
            x /= shift;
        }
    }
    false
}

#[aoc(day2, part2)]
fn part2(input: &RangeSet) -> usize {
    input
        .ranges()
        .into_iter()
        .flat_map(|range| range.clone())
        .filter(|&value| is_repeating_seq(value))
        .sum()
}
