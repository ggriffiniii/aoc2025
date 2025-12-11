use std::{collections::VecDeque, convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

const EXAMPLE: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

#[derive(Debug)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    jolts: Vec<usize>,
}
impl FromStr for Machine {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Machine, Infallible> {
        let (lights_input, input) = input.split_once("] ").unwrap();
        let lights_input = &lights_input[1..].as_bytes();
        assert!(lights_input.len() <= 10);
        let lights = lights_input.iter().enumerate().fold(0u16, |mut a, (idx, &value)| {
            if value == b'#' {
                a |= 1 << idx;
            }
            a
        });
        let (buttons_input, _jolts_input) = input.split_once(" {").unwrap();
        let buttons: Vec<_> = buttons_input.split_whitespace().map(|button_input| {
            button_input[1..button_input.len()-1].split(",").map(|b| b.parse::<u16>().unwrap()).fold(0u16, |a, b| a | (1 << b))
        }).collect();
        Ok(Machine{lights, buttons, jolts: Vec::new()})
    }
}

fn find_min_button_presses(m: &Machine) -> usize {
    struct SearchState{lights: u16, num_presses: usize}
    let mut queue = VecDeque::from([SearchState{lights: 0, num_presses: 0}]);
    let mut seen = vec![false; 1<<10];

    while let Some(SearchState{lights, num_presses}) = queue.pop_front() {
        if lights == m.lights {
            return num_presses;
        }
        for button in &m.buttons {
            let lights = lights ^ button;
            if !seen[lights as usize] {
                seen[lights as usize] = true;
                queue.push_back(SearchState{lights, num_presses: num_presses+1});
            }
        }
    }
    unreachable!();
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    //let input = EXAMPLE;
    let machines: Vec<_> = input.lines().map(|input| Machine::from_str(input).unwrap()).collect();
    machines.iter().map(find_min_button_presses).sum()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    todo!()
}
