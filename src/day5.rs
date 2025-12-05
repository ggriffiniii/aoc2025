use std::{cmp::Ordering, ops::Range};

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct IntervalSet(Vec<Range<usize>>);
impl IntervalSet {
    fn new(mut ranges: Vec<Range<usize>>) -> Self {
        ranges.sort_by_key(|r| r.start);

        let mut merged_tail = 0;
        for idx in 1..ranges.len() {
            if ranges[idx].start >= ranges[merged_tail].end {
                merged_tail += 1;
                ranges[merged_tail] = ranges[idx].clone();
            } else {
                ranges[merged_tail].end = std::cmp::max(ranges[merged_tail].end, ranges[idx].end);
            }
        }
        ranges.truncate(merged_tail+1);
        IntervalSet(ranges)
    }

    fn contains(&self, needle: usize) -> bool {
        self.0.binary_search_by(|range| {
            match (range.start.cmp(&needle), range.end.cmp(&needle)) {
                (Ordering::Less | Ordering::Equal, Ordering::Greater) => Ordering::Equal,
                (Ordering::Greater, _) => Ordering::Greater,
                (_, Ordering::Less | Ordering::Equal) => Ordering::Less,
            }
        }).is_ok()
    }

    fn ranges(&self) -> &[Range<usize>] {
        &self.0
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let fresh_ingredients: Vec<Range<usize>> = lines.by_ref().take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..end.parse::<usize>().unwrap()+1
        })
        .collect();
    let fresh_ingredients = IntervalSet::new(fresh_ingredients);

    let available_ingredients: Vec<usize> = lines
        .map(|line| line.parse().unwrap())
        .collect();

    available_ingredients
        .iter()
        .copied()
        .filter(|&ingredient| {
            fresh_ingredients.contains(ingredient)
        })
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let fresh_ranges: Vec<Range<usize>> = input.lines().take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..end.parse::<usize>().unwrap()+1
        })
        .collect();
    let intervalset = IntervalSet::new(fresh_ranges);

    intervalset.ranges().iter()
        .map(|range| range.len())
        .sum()
}
