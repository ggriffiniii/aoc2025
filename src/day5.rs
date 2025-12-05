use std::ops::RangeInclusive;

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct DB {
    fresh_ingredients: Vec<RangeInclusive<usize>>,
    available_ingredients: Vec<usize>,
}
impl DB {
    fn new(input: &str) -> DB {
        let (fresh_input, available_input) = input.split_once("\n\n").unwrap();
        let fresh_ingredients = fresh_input
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            })
            .collect();

        let available_ingredients = available_input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        DB {
            fresh_ingredients,
            available_ingredients,
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let db = DB::new(input);

    db.available_ingredients
        .iter()
        .copied()
        .filter(|ingredient| {
            db.fresh_ingredients
                .iter()
                .any(|range| range.contains(ingredient))
        })
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let db = DB::new(input);
    let mut fresh_ranges = db.fresh_ingredients;

    fresh_ranges.sort_by_key(|range| (*range.start(), *range.end()));

    let mut merged_ranges = Vec::new();

    let mut r = fresh_ranges[0].clone();
    for fresh_range in &fresh_ranges[1..] {
        if *r.end() + 1 < *fresh_range.start() {
            merged_ranges.push(r);
            r = fresh_range.clone();
        } else if fresh_range.end() > r.end() {
            r = *r.start()..=*fresh_range.end()
        }
    }
    merged_ranges.push(r);

    merged_ranges
        .into_iter()
        .map(|range| *range.end() - *range.start() + 1)
        .sum()
}
