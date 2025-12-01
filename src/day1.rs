use aoc_runner_derive::aoc;

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

#[derive(Debug, Copy, Clone)]
enum Move {
    Left(usize),
    Right(usize),
}

fn moves(input: &str) -> impl Iterator<Item = Move> {
    input
        .split('\n')
        .map(|line| (line.as_bytes()[0], &line[1..]))
        .map(|(direction, amount)| match direction {
            b'L' => Move::Left(amount.parse().unwrap()),
            b'R' => Move::Right(amount.parse().unwrap()),
            _ => panic!("not L or R"),
        })
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut dial = 50isize;
    let mut count_zeros = 0;
    for m in moves(input) {
        match m {
            Move::Left(amount) => dial -= amount as isize,
            Move::Right(amount) => dial += amount as isize,
        }
        dial = dial.rem_euclid(100);
        if dial == 0 {
            count_zeros += 1;
        }
    }
    count_zeros
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut dial = 50isize;
    let mut touched_zero = 0;
    for m in moves(input) {
        let prev_dial = dial;
        match m {
            Move::Left(amount) => {
                touched_zero += amount / 100;
                if dial > 0 && amount as isize % 100 >= dial {
                    touched_zero += 1;
                }
                dial -= amount as isize;
            }
            Move::Right(amount) => {
                touched_zero += amount / 100;
                if dial > 0 && dial + (amount as isize % 100) >= 100 {
                    touched_zero += 1;
                }
                dial += amount as isize;
            }
        }
        dial = dial.rem_euclid(100);
    }
    touched_zero
}
