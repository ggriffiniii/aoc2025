use core::f64;
use std::{collections::VecDeque, convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

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
        let lights_input = &lights_input.as_bytes()[1..];
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
    let machines: Vec<_> = input
        .lines()
        .map(|input| Machine::from_str(input).unwrap())
        .collect();
    machines.iter().map(find_min_button_presses).sum()
}

const EPSILON: f64 = 1e-9;

// The struct that 'recursive_solve' uses
#[derive(Debug, Clone)]
pub struct Constraint {
    // The coefficients for your independent variables (e.g., indices 10 and 11)
    pub coeffs: Vec<f64>,
    // The target value (RHS) of the row
    pub target: f64,
}

pub fn parse_rref_into_constraints(
    matrix: &[Vec<f64>],
    pivots: &[usize],
    total_vars: usize,
) -> Vec<Constraint> {
    let cols = matrix[0].len();
    let free_var_indices: Vec<usize> = (0..total_vars).filter(|i| !pivots.contains(i)).collect();

    let mut constraints = Vec::new();

    for matrix_row in matrix {
        let free_var_idx = matrix_row
            .iter()
            .enumerate()
            .filter(|&(_idx, &v)| v != 0.0)
            .map(|(idx, _v)| idx)
            .next();

        if let Some(_dep_idx) = free_var_idx {
            let mut row_coeffs = Vec::new();
            for &free_var_idx in &free_var_indices {
                row_coeffs.push(matrix_row[free_var_idx]);
            }

            constraints.push(Constraint {
                coeffs: row_coeffs,
                target: matrix_row[cols - 1],
            });
        }
    }

    constraints
}

/// Returns the RREF matrix and a list of pivot column indices.
pub fn rref(mut matrix: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<usize>) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut pivot_row = 0;
    let mut pivot_indices = Vec::new();

    for col in 0..cols {
        if pivot_row >= rows {
            break;
        }

        let mut max_row = pivot_row;
        let mut max_val = matrix[pivot_row][col].abs();

        for i in (pivot_row + 1)..rows {
            let val = matrix[i][col].abs();
            if val > max_val {
                max_val = val;
                max_row = i;
            }
        }

        // If the column is effectively all zeros, skip it (it's a free variable)
        if max_val < EPSILON {
            continue;
        }

        // Swap the best pivot to the current row
        matrix.swap(pivot_row, max_row);
        pivot_indices.push(col);

        // Divide the entire row by the pivot so the pivot becomes exactly 1.0
        let pivot_val = matrix[pivot_row][col];

        for j in col..cols {
            matrix[pivot_row][j] /= pivot_val;
        }

        for i in 0..rows {
            if i != pivot_row {
                let factor = matrix[i][col];
                if factor.abs() > EPSILON {
                    for j in col..cols {
                        matrix[i][j] -= factor * matrix[pivot_row][j];

                        if matrix[i][j].abs() < EPSILON {
                            matrix[i][j] = 0.0;
                        }
                    }
                    matrix[i][col] = 0.0;
                }
            }
        }
        pivot_row += 1;
    }

    (matrix, pivot_indices)
}

fn solve_dependent_vars(
    rref_matrix: &[Vec<f64>],
    pivots: &[usize],
    free_vars_indices: &[usize],
    free_vars_values: &[f64],
) -> Vec<f64> {
    let num_rows = rref_matrix.len();
    let num_cols = rref_matrix[0].len();
    let total_vars = num_cols - 1;

    let mut solution = vec![0.0; total_vars];

    for (i, &idx) in free_vars_indices.iter().enumerate() {
        solution[idx] = free_vars_values[i];
    }

    for r in 0..num_rows {
        if r >= pivots.len() {
            continue;
        }
        let pivot_idx = pivots[r];

        let target_val = rref_matrix[r][num_cols - 1];

        let mut sum_free_parts = 0.0;

        for (i, &free_idx) in free_vars_indices.iter().enumerate() {
            let coeff = rref_matrix[r][free_idx];
            let val = free_vars_values[i];
            sum_free_parts += coeff * val;
        }

        solution[pivot_idx] = target_val - sum_free_parts;
    }

    solution
}

fn recursive_find_min_button_presses(
    rref: &[Vec<f64>],
    pivots: &[usize],
    constraints: &[Constraint],
    depth: usize,
    free_vars_indices: &[usize],
    free_vars_values: &mut [f64],
    max_button_presses: f64,
    min_button_presses: &mut f64,
) {
    if depth == free_vars_values.len() {
        let all_var_values =
            solve_dependent_vars(rref, pivots, free_vars_indices, free_vars_values);
        if all_var_values
            .iter()
            .any(|value| value.round() < 0.0 || (value - value.round()).abs() > EPSILON)
        {
            // Can't have negative or fractional button pushes.
            return;
        }
        let total_button_presses: f64 = all_var_values.iter().sum();
        *min_button_presses = total_button_presses.min(*min_button_presses);

        return;
    }

    // Recurse through all possible combinations of the remaining free variables.
    let mut min_val = 0.0;
    let mut max_val = max_button_presses;

    // Further bound the last free variable.
    if depth == free_vars_values.len() - 1 {
        for c in constraints {
            let coeff = c.coeffs[depth];
            if coeff == 0.0 {
                continue;
            }

            let mut current_rhs = c.target;
            for (coeff, free_var_value) in c.coeffs[..depth].iter().zip(&free_vars_values[..depth])
            {
                current_rhs -= coeff * free_var_value;
            }

            if coeff > 0.0 {
                if current_rhs < 0.0 {
                    return;
                }
                let limit = current_rhs / coeff;
                if limit < max_val {
                    max_val = limit;
                }
            } else {
                let abs_coeff = coeff.abs();
                let limit = (-current_rhs + abs_coeff - 1.0) / abs_coeff;
                if limit > min_val {
                    min_val = limit;
                }
            }
        }
    }

    let min_val = min_val.trunc() as i64;
    let max_val = max_val.ceil() as i64;

    if min_val > max_val {
        return;
    }

    for val in min_val..=max_val {
        free_vars_values[depth] = val as f64;
        recursive_find_min_button_presses(
            rref,
            pivots,
            constraints,
            depth + 1,
            free_vars_indices,
            free_vars_values,
            max_button_presses,
            min_button_presses,
        );
    }
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
///
///
fn solve_min_button_presses_to_satisfy_jolt(m: &Machine) -> f64 {
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

    let (rref, pivots) = rref(matrix);
    let free_vars: Vec<_> = (0..m.buttons.len())
        .filter(|idx| !pivots.contains(idx))
        .collect();
    let constraints = parse_rref_into_constraints(&rref, &pivots, m.buttons.len());
    // We know that it should never take more button presses than the maximum
    // value required for any individual joltage counter.
    let max_button_presses = m.jolts.iter().copied().max().unwrap() as f64;

    // Our result will be stored in min_button_presses by the recursive algorithm.
    let mut min_button_presses = f64::MAX;
    recursive_find_min_button_presses(
        &rref,
        &pivots,
        &constraints,
        0,
        &free_vars,
        &mut vec![0.0; free_vars.len()],
        max_button_presses,
        &mut min_button_presses,
    );
    if min_button_presses == f64::MAX {
        panic!("unable to solve for machine: {m:?}");
    }
    min_button_presses
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u64 {
    let machines: Vec<_> = input
        .lines()
        .map(|input| Machine::from_str(input).unwrap())
        .collect();

    machines
        .iter()
        .map(solve_min_button_presses_to_satisfy_jolt)
        .sum::<f64>()
        .round() as u64
}
