use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Grid {
    row_len: usize,
    data: Vec<bool>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut data = Vec::with_capacity(input.len());
        let mut row_len = 0;
        for line in input.as_bytes().split(|&b| b == b'\n') {
            row_len = line.len();
            data.extend(line.iter().copied().map(|b| b == b'@'));
        }
        Grid { row_len, data }
    }

    fn col_len(&self) -> usize {
        self.data.len() / self.row_len
    }

    fn neighbors(&self, pos: usize) -> impl Iterator<Item = usize> {
        let x = pos % self.row_len;
        let y = pos / self.row_len;
        let x = x as isize;
        let y = y as isize;
        let row_len = self.row_len as isize;
        let col_len = self.col_len() as isize;

        (x - 1..=x + 1)
            .flat_map(move |x| (y - 1..=y + 1).map(move |y| (x, y)))
            .filter(move |&(nx, ny)| {
                nx >= 0 && ny >= 0 && nx < row_len && ny < col_len && !(nx == x && ny == y)
            })
            .map(move |(x, y)| (y * row_len + x) as usize)
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.data
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, occupied)| occupied)
        .filter(|&(idx, _)| {
            grid.neighbors(idx)
                .filter(|&neighbor_idx| grid.data[neighbor_idx])
                .count()
                < 4
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let initial_num_rolls = grid.data.iter().filter(|&&b| b).count();
    loop {
        let rolls_to_remove: Vec<_> = grid
            .data
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, occupied)| occupied)
            .filter(|&(idx, _)| {
                grid.neighbors(idx)
                    .filter(|&neighbor_idx| grid.data[neighbor_idx])
                    .count()
                    < 4
            })
            .map(|(idx, _)| idx)
            .collect();

        if rolls_to_remove.is_empty() {
            break;
        }

        for idx in rolls_to_remove {
            grid.data[idx] = false;
        }
    }
    let num_rolls = grid.data.iter().filter(|&&b| b).count();
    initial_num_rolls - num_rolls
}
