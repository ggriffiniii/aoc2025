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
                // The abs_diff calc below will claim to touch zero if the dial
                // starts at 0 and it moves left. Offset that here.
                if dial % 100 == 0 {
                    touched_zero -= 1;
                }
                dial -= amount as isize;

                // The abs_diff calc below will claim to not touch zero if the
                // dial finishes at 0 after it moves left. Offset that here.
                if dial % 100 == 0 {
                    touched_zero += 1;
                }
            }
            Move::Right(amount) => {
                dial += amount as isize;
            }
        }
        touched_zero += dial.div_euclid(100).abs_diff(prev_dial.div_euclid(100));
    }
    touched_zero
}
