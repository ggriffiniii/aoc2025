use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut ops_line = "";
    let numbers: Vec<usize> = lines
        .by_ref()
        .take_while(|&line| {
            let is_num = line.as_bytes()[0].is_ascii_digit();
            if !is_num {
                ops_line = line;
            }
            is_num
        })
        .flat_map(|line| line.split_whitespace())
        .map(|num| num.parse().unwrap())
        .collect();
    let ops: Vec<&str> = ops_line.split_whitespace().collect();
    let num_cols = ops.len();
    (0..num_cols)
        .map(|colidx| -> usize {
            let col_nums = numbers.iter().copied().skip(colidx).step_by(num_cols);
            match ops[colidx] {
                "+" => col_nums.sum(),
                "*" => col_nums.product(),
                op => panic!("invalid op: {op}"),
            }
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let mut input = input.as_bytes().to_vec();
    input.push(b'\n');
    let row_len = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let num_rows = input.len() / row_len;
    let op_row = &input[(num_rows - 1) * row_len..];
    assert_eq!((input.len()) % row_len, 0);

    let op_cols: Vec<_> = op_row
        .iter()
        .copied()
        .enumerate()
        .filter(|&(idx, val)| val == b'\n' || !val.is_ascii_whitespace())
        .collect();
    let mut op_cols: Vec<_> = op_cols
        .windows(2)
        .map(|pair| {
            let col_range = pair[0].0..pair[1].0 - 1;
            let op = pair[0].1;
            (col_range, op)
        })
        .collect();
    op_cols.last_mut().unwrap().0.end += 1; // The last column doesn't have a trailing space

    op_cols
        .iter()
        .map(|(col_range, op)| {
            let numbers = col_range.clone().map(|colidx| {
                (0..num_rows - 1)
                    .map(|rowidx| input[rowidx * row_len + colidx])
                    .filter(|b| b.is_ascii_digit())
                    .map(|b| b - b'0')
                    .fold(0usize, |accum, num| accum * 10 + num as usize)
            });
            match op {
                b'+' => numbers.sum::<usize>(),
                b'*' => numbers.product::<usize>(),
                _ => panic!("invalid op"),
            }
        })
        .sum()
}
