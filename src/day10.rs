use core::f64;
use std::{
    cmp,
    collections::{HashMap, VecDeque},
    convert::Infallible,
    ops::Range,
    str::FromStr,
};

use aoc_runner_derive::aoc;

//const EXAMPLE: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//[####..##.#] (0,1,2,3,4,6) (0,7,9) (0,1,2,3,5,6,7,8,9) (1,2,5,6,7,9) (1,3,8,9) (1,2,3,4,5,6,7,8) (1,2,3,6,7,9) (5,6,7,9) (1,2,3,6,7) (1,4,6) {16,95,61,61,32,52,85,65,42,64}
//[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
//[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
////[####..##..] (0,4,5,6,7,9) (0,2,4,6,9) (0,1,2,3,5,6,7,8) (1,9) (0,1,3,4,5,6,7,8) (0,1,3,7,9) (3,7) (1,3,4,6,7,9) (1,2,3,4,5,7,8,9) (0,1,2,6,8) (0,1,2,4,8,9) (0,1,2,7,8,9) (1,3,5,7,8) {211,227,193,67,188,46,42,85,204,212}"#;
const EXAMPLE: &str = r#"[##......##] (2) (0,1,2,4,5,7,8,9) (0,1,2,3,4,5,8,9) (0,2,4,5,6,7,9) (0,1,4,7) (0,4,9) (0,1,6,8) (4,6) (5) (3,4,6,8) (0,3,5,6) (0,3,5,6,7,9) {58,20,22,42,60,49,45,8,26,37}"#;

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
        let lights = lights_input
            .iter()
            .enumerate()
            .fold(0u16, |mut a, (idx, &value)| {
                if value == b'#' {
                    a |= 1 << idx;
                }
                a
            });
        let (buttons_input, jolts_input) = input.split_once(" {").unwrap();
        let buttons = buttons_input
            .split_whitespace()
            .map(|button_input| {
                button_input[1..button_input.len() - 1]
                    .split(",")
                    .map(|b| b.parse::<u16>().unwrap())
                    .fold(0u16, |a, b| a | (1 << b))
            })
            .collect();
        let jolts = jolts_input[..jolts_input.len() - 1]
            .split(",")
            .map(|j| j.parse().unwrap())
            .collect();
        Ok(Machine {
            lights,
            buttons,
            jolts,
        })
    }
}

fn find_min_button_presses(m: &Machine) -> usize {
    struct SearchState {
        lights: u16,
        num_presses: usize,
    }
    let mut queue = VecDeque::from([SearchState {
        lights: 0,
        num_presses: 0,
    }]);
    let mut seen = vec![false; 1 << 10];

    while let Some(SearchState {
        lights,
        num_presses,
    }) = queue.pop_front()
    {
        if lights == m.lights {
            return num_presses;
        }
        for button in &m.buttons {
            let lights = lights ^ button;
            if !seen[lights as usize] {
                seen[lights as usize] = true;
                queue.push_back(SearchState {
                    lights,
                    num_presses: num_presses + 1,
                });
            }
        }
    }
    unreachable!();
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    //let input = EXAMPLE;
    let machines: Vec<_> = input
        .lines()
        .map(|input| Machine::from_str(input).unwrap())
        .collect();
    machines.iter().map(find_min_button_presses).sum()
}

const EPSILON: f64 = 1e-9;

/// Transforms a matrix into Reduced Row Echelon Form (RREF) in place.
/// Returns a list of column indices that contain pivots (Dependent Variables).
fn rref(matrix: &mut Vec<Vec<f64>>) -> Vec<usize> {
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }
    let cols = matrix[0].len();

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    for col in 0..cols - 1 {
        // Iterate columns, excluding the RHS result column
        if pivot_row >= rows {
            break;
        }

        // 1. Find a row with a non-zero entry in this column (Pivot selection)
        let mut curr = pivot_row;
        while curr < rows && matrix[curr][col].abs() < EPSILON {
            curr += 1;
        }

        // If no row has a value here, this is a Free Variable (Independent)
        if curr == rows {
            continue;
        }

        // 2. Swap the found row up to the pivot position
        matrix.swap(pivot_row, curr);
        pivot_cols.push(col);

        // 3. Normalize the pivot row (Divide by pivot so it becomes 1.0)
        let pivot_val = matrix[pivot_row][col];
        for j in col..cols {
            // Optimize: Start from 'col' as left is already 0
            matrix[pivot_row][j] /= pivot_val;
        }

        // 4. Eliminate all other rows
        for i in 0..rows {
            if i != pivot_row {
                let factor = matrix[i][col];
                if factor.abs() > EPSILON {
                    for j in col..cols {
                        let val = matrix[pivot_row][j];
                        matrix[i][j] -= factor * val;
                    }
                }
            }
        }

        pivot_row += 1;
    }

    pivot_cols
}

fn get_global_bounds(
    target_var: usize,
    matrix: &Vec<Vec<f64>>,
    pivot_cols: &Vec<usize>,
) -> (f64, f64) {
    let num_vars = matrix[0].len() - 1;
    let mut global_min = 0.0;
    let mut global_max = f64::INFINITY;

    for row in matrix {
        // 1. Identify Dependent Variable for this row
        let pivot_idx = match row[0..num_vars]
            .iter()
            .position(|&x| (x - 1.0).abs() < 1e-9)
        {
            Some(i) => i,
            None => continue,
        };

        let target_coeff = row[target_var];
        if target_coeff.abs() < 1e-9 {
            continue;
        } // D is not in this equation

        let rhs = row[num_vars];

        // Equation: Pivot + (Target_Coeff * D) + (Other_Terms) = RHS
        // Pivot = RHS - (Target_Coeff * D) - (Other_Terms)
        // Constraint: Pivot >= 0
        // So: RHS - (Target_Coeff * D) - (Other_Terms) >= 0

        // 2. Analyze "Other Terms" (Independent Variables other than D)
        // We need to know: Can "Other_Terms" help us or hurt us?
        let mut best_case_help = 0.0;

        for col in 0..num_vars {
            if col != pivot_idx && col != target_var && row[col].abs() > 1e-9 {
                // This is another independent variable (like F)
                let coeff = row[col];

                // If we want to MAXIMIZE D, we want this term to be as POSITIVE as possible
                // so it cancels out negative pressure.

                // Term in equation is: - (coeff * Indep_Var)
                // If coeff is NEGATIVE (e.g., -1.0 * F), then -(-1*F) = +F.
                // A positive F gives us MORE room for D.

                if coeff < 0.0 {
                    // It helps us! What is the MAX value F can take?
                    // (This requires knowing F's bounds. For now, let's assume F can go up to infinity
                    // or some known limit. If F is bounded by another row, this gets complex recursively.)
                    // simpler approach: Just add 0 if we assume worst case,
                    // or calculate F's max if we want global max.

                    // For your specific case: F adds to D's limit.
                    // If we assume F can be anything, D is effectively unbounded (Infinite).
                    best_case_help = f64::INFINITY;
                } else {
                    // It hurts us! The best case is F = 0.
                    best_case_help += 0.0;
                }
            }
        }

        // 3. Solve for D
        // Basic: RHS + best_case_help >= Target_Coeff * D

        if target_coeff > 0.0 {
            // D <= (RHS + Help) / Coeff
            let limit = (rhs + best_case_help) / target_coeff;
            if limit < global_max {
                global_max = limit;
            }
        } else {
            // D >= (RHS + Help) / Coeff (Sign flips)
            let limit = (rhs + best_case_help) / target_coeff;
            if limit > global_min {
                global_min = limit;
            }
        }
    }

    (global_min, global_max)
}

fn solve_system(
    matrix: &Vec<Vec<f64>>,
    pivot_cols: &Vec<usize>,
    independent_values: &HashMap<usize, f64>,
) -> Vec<f64> {
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }
    let num_vars = matrix[0].len() - 1; // Exclude RHS column

    // 1. Initialize the solution vector with 0.0
    let mut solution = vec![0.0; num_vars];

    // 2. Fill in the Independent Variables (User Inputs)
    // We do this first because the Dependent variables rely on these numbers.
    for (&col_idx, &val) in independent_values {
        if pivot_cols.contains(&col_idx) {
            println!(
                "Warning: Column {} is a Dependent Variable (Pivot). Your input for it will be overwritten.",
                col_idx
            );
        } else {
            solution[col_idx] = val;
        }
    }

    // 3. Calculate the Dependent Variables
    // We iterate through the RREF matrix. Each row defines exactly one Dependent Variable.
    for row in matrix {
        // Find the pivot (the first 1.0) in this row to identify which variable it solves for
        let pivot_idx = match row[0..num_vars]
            .iter()
            .position(|&x| (x - 1.0).abs() < EPSILON)
        {
            Some(i) => i,
            None => continue, // Skip empty rows (0=0)
        };

        // Start calculation: Dependent_Var = RHS - (Everything Else)
        let rhs = row[num_vars];
        let mut calculated_value = rhs;

        for col in 0..num_vars {
            if col != pivot_idx {
                // Subtract the term: Coefficient * Value
                // (If the variable is 0.0, this does nothing, which is fine)
                let coeff = row[col];
                let val = solution[col];

                if coeff.abs() > EPSILON {
                    calculated_value -= coeff * val;
                }
            }
        }

        solution[pivot_idx] = calculated_value;
    }

    solution
}

/// To solve the minimum number of button presses that satisfy the joltage
/// requirements we setup a series of linear equations representing the
/// requirements.
///
/// For the example data:
///
/// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
///
/// Let's label the buttons with variable names
///
///  A    B    C    D     E     F
/// (3) (1,3) (2) (2,3) (0,2) (0,1)
///
/// The answer we want is 3,5,4,7
/// The first column of the solution `3` is incremented whenever button E or
/// F are pressed. So for the solution to be solved we need
/// E + F = 3
/// The second column of the solution `5` is incremented when button B or F
/// are pressed. So for the solution to be solved we need
/// B + F = 5
/// The third column of the solution `4` is incremented when button C, D, or
/// E are pressed. So for the solution to be solved we need
/// C + D + E = 4
/// The fourth column of the solution `7` is incremented when button A, B, or
/// D are pressed. So for the solution to be solved we need
/// A + B + D = 7
///
///
/// Lining up all the constraints looks like
///
/// E + F = 3
/// B + F = 5
/// C + D + E = 4
/// A + B + D = 7
///
/// 6 Variables with 4 equations means that 2 variables are independent and 4
/// are dependent. We can figure out which variables are independent (any number
/// can be chosen) and which variables are dependent (can be computed for any
/// given set of independent variables).
///
/// For this example we can do that by hand.
/// Solve the first equation for E
/// E = 3 - F
///
/// Solve the second equation for B
/// B = 5 - F
///
/// Substituate the equation for E into the 3rd equation
/// C + D + (3 - F) = 4
///   C + D - F = 1
///
/// Substitute the equation for B into the 4th equation
/// A + (5 - F) + D = 7
///   A - F + D = 2
///
/// E = 3 - F
/// B = 5 - F
/// C = 1 - D + F
/// A = 2 - D + F
///
/// Now we can see that E,B,C,A can be computed for any value of D and F.
///
/// We know there's no such thing as a negative button push. So we can see from
/// our equations that F would need to be <= 3 for E to not be negative.
/// From this we now iterate all combinations of F = 0..=3 and D = 0.. and
/// calculate the solution for the dependent variables.
///
/// I think since our equations all only have 1 as a coefficient once we see an
/// increase a button press increase the total button presses we don't need to
/// explore the range any higher.
///
/// In other words as we increment D from 0 to infinity. If D = 1 solves to 11
/// button presses and D = 2 solves to 13 button presses, I think it's safe to
/// stop and say that D = 2 is the minimum.
fn solve_min_button_presses_to_satisfy_jolt(m: &Machine) -> usize {
    let mut matrix = Vec::new();
    for (jolt_idx, &jolt_value) in m.jolts.iter().enumerate() {
        let mut row: Vec<_> = m
            .buttons
            .iter()
            .map(|&button| {
                if button & (1 << jolt_idx) == 0 {
                    0.0
                } else {
                    1.0
                }
            })
            .collect();
        row.push(jolt_value as f64);
        matrix.push(row);
    }
    let original_matrix = matrix.clone();
    dbg!(&matrix);
    let pivots = rref(&mut matrix);
    let independent_vars: Vec<_> = (0..matrix[0].len() - 1)
        .filter(|i| !pivots.contains(i))
        .collect();
    let mut independent_vars: Vec<_> = independent_vars
        .iter()
        .map(|&var| {
            let (min, max) = get_global_bounds(var, &matrix, &pivots);
            (var, min as usize..(max + 1.0) as usize)
        })
        .collect();
    independent_vars.sort_by_key(|(_, bounds)| cmp::Reverse(bounds.len()));
    dbg!(&independent_vars);

    let mut independent_var_state: Vec<_> = independent_vars
        .iter()
        .map(|(_, bounds)| bounds.start)
        .collect();

    let mut min = f64::MAX;
    let mut last_min_change = -1;
    for idx in 0.. {
        if last_min_change >= 0 && idx - last_min_change > 10000 {
            break;
        }
        increment_independent_vars(&independent_vars, &mut independent_var_state);
        let iv: HashMap<_, _> = independent_vars
            .iter()
            .zip(independent_var_state.iter())
            .map(|((idx, _range), &value)| (*idx, value as f64))
            .collect();
        //dbg!(&iv);
        let solution = solve_system(&matrix, &pivots, &iv);
        let button_presses: f64 = solution.iter().sum();
        //dbg!(&solution);
        //if solution.iter().any(|&v| v.round() < 0.0 || (v - v.round()).abs() > 0.1) {
        if solution.iter().any(|&v| v.round() < 0.0) || (button_presses - button_presses.round()).abs() > 0.1 {
            dbg!(button_presses);
            if button_presses.round() >= 87.0 && button_presses < 88.0 {
                eprintln!("matrix: {original_matrix:?}");
                eprintln!("rref: {matrix:?}");
                eprintln!("independent_vars: {independent_vars:?}");
                eprintln!("machine: {m:?}");
                eprintln!("{solution:?}");
                eprintln!("solution invalid: {}", solution.iter().map(|v| v.round()).sum::<f64>());
                dbg!(solution);
            }
            continue;
        }
        if button_presses < min {
            min = button_presses;
            last_min_change = idx;
            let mut solution_joltage = vec![0; m.jolts.len()];
            for (&v, &button) in solution.iter().zip(m.buttons.iter()) {
                for jolt_idx in 0..m.jolts.len() {
                    if button & (1 << jolt_idx) > 0 {
                        solution_joltage[jolt_idx] += v.round() as usize;
                    }
                }
            }
            if solution_joltage != m.jolts {
                eprintln!("matrix: {original_matrix:?}");
                eprintln!("rref: {matrix:?}");
                eprintln!("independent_vars: {independent_vars:?}");
                eprintln!("machine: {m:?}");
                eprintln!("{solution:?}");
                panic!("solution invalid: {solution_joltage:?} does not equal {:?}", m.jolts);
            }
        }
    }
    if last_min_change == -1 {
        panic!("no solution found for machine: {m:?}");
    }
    dbg!(min) as usize
}

fn increment_independent_vars(vars: &[(usize, Range<usize>)], state: &mut Vec<usize>) {
    for (idx, var_state) in state.iter_mut().enumerate().rev() {
        *var_state += 1;
        if *var_state < vars[idx].1.end {
            return;
        }
        *var_state = vars[idx].1.start;
    }
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let input = EXAMPLE;
    let machines: Vec<_> = input
        .lines()
        .map(|input| Machine::from_str(input).unwrap())
        .collect();

    machines.iter().map(solve_min_button_presses_to_satisfy_jolt).sum()
}
