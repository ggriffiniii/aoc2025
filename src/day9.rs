
use std::sync::atomic::AtomicBool;

use aoc_runner_derive::aoc;

const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let coords: Vec<_> = input
        .lines()
        .map(|line| -> (usize, usize) {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
        .collect();
    let coords = coords.as_slice();

    (0..coords.len())
        .flat_map(|i| (i..coords.len()).map(move |j| (coords[i], coords[j])))
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .unwrap()
}

fn is_rect_inside_polygon(grid: &[usize], grid_row_len: usize, a: (usize,usize), b: (usize,usize)) -> bool {
    let [x_min, x_max] = std::cmp::minmax(a.0, b.0);
    let [y_min, y_max] = std::cmp::minmax(a.1, b.1);

    let s = grid[grid_row_len*(y_max)+(x_max)];
    let left = if x_min > 0 { grid[grid_row_len*(y_max)+(x_min-1)] } else { 0 };
    let top = if y_min > 0 { grid[grid_row_len*(y_min-1)+(x_max)]} else { 0 };
    let corner = if x_min > 0 && y_min > 0 { grid[grid_row_len*(y_min-1)+(x_min-1)]} else { 0 };

    let area_inside_polygon = s - top - left + corner;
    let rect_area = ((x_max - x_min) + 1) * ((y_max - y_min) + 1);

    if debug() {
        eprintln!("({x_min},{y_min}) -> ({x_max},{y_max})");
        dbg!(&area_inside_polygon);
        dbg!(&rect_area);
    }

    area_inside_polygon == rect_area
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let coords: Vec<_> = input
        .lines()
        .map(|line| -> (usize, usize) {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
        .collect();

    // Coordinate compress the values so we can use a matrix efficiently.
    let compress = |f: fn((usize, usize)) -> usize| -> Vec<usize> {
        let mut v: Vec<usize> = coords.iter().copied().map(f).collect();
        v.sort();
        v.dedup();
        v
    };
    let compressed_x = compress(|(x, _y)| x);
    let compressed_y = compress(|(_x, y)| y);

    let raw_to_compressed_x = |raw_x| { compressed_x.binary_search(&raw_x). unwrap() };
    let raw_to_compressed_y = |raw_y| { compressed_y.binary_search(&raw_y). unwrap() };

    // Create a grid that contains vertical columns.
    #[repr(u8)]
    #[derive(Debug,Copy,Clone,Eq,PartialEq)]
    enum WallTile {
        Empty = 0,
        Corner = 1,
        Surface = 2,
    }
    let mut cols = vec![WallTile::Empty; compressed_x.len()*(compressed_y.len())];
    for ci in 0..coords.len() {
        let a = coords[ci];
        let b = coords[(ci+1) % coords.len()];

        if a.0 != b.0 {
            continue;
        }

        let ix = raw_to_compressed_x(a.0);
        let [min_y, max_y] = std::cmp::minmax(raw_to_compressed_y(a.1), raw_to_compressed_y(b.1));
        for j in min_y..=max_y {
            let tile = if j == min_y || j == max_y { WallTile::Corner } else { WallTile::Surface };
            cols[compressed_x.len()*j+ix]= tile;
        }
    }

    if debug() {
        for i in 0..compressed_y.len() {
            for j in 0..compressed_x.len() {
                eprint!("{}", match cols[compressed_x.len()*i+j] {
                    WallTile::Empty => '.',
                    WallTile::Corner => 'X',
                    WallTile::Surface => '#',
                });
            }
            eprintln!("");
        }
        eprintln!("");
    }

    // Create a grid where coordinates inside the polygon are true.
    let mut grid = vec![0usize; compressed_x.len()*compressed_y.len()];
    // For each row start assuming we're outside the polygon, for each wall we
    // encounter invert whether we're inside or outside.
    for j in 0..compressed_y.len() {
        let mut inside = false;
        for i in 0..compressed_x.len() {
            let x = match cols[compressed_x.len()*j+i] {
                WallTile::Surface => {
                    inside = !inside;
                    true
                },
                WallTile::Corner => { 
                    inside = true;
                    true
                },
                WallTile::Empty => { inside },
            };
            grid[compressed_x.len()*j+i] = x as usize;
        }
    }

    if debug() {
        for i in 0..compressed_y.len() {
            for j in 0..compressed_x.len() {
                eprint!("{}", grid[compressed_x.len()*i+j]);
            }
            eprintln!("");
        }
        println!("");
    }

    // Turns the grid into a prefix sum to allow for efficient calculation of
    // the sum of any rectangle.
    for i in 0..compressed_x.len() {
        for j in 0..compressed_y.len() {
            let left = if i > 0 { grid[compressed_x.len()*j+(i-1) ]} else { 0 };
            let above = if j > 0 { grid[compressed_x.len()*(j-1)+i ]} else { 0 };
            let corner = if i > 0 && j > 0 { grid[compressed_x.len()*(j-1)+(i-1)]} else { 0 };

            grid[compressed_x.len()*j+i] += left + above - corner;
        }
    }

    if debug() {
        for i in 0..compressed_y.len() {
            for j in 0..compressed_x.len() {
                eprint!("{:02} ", grid[compressed_x.len()*i+j]);
            }
            eprintln!("");
        }
        eprintln!("");
    }

    let mut max = 0;
    for i in 0..coords.len() {
        for j in i + 1 .. coords.len() {
            let a = coords[i];
            let b = coords[j];
            /*
            if a == (9,5) && b == (2,3) {
                DBG.store(true, std::sync::atomic::Ordering::SeqCst)
            } else {
                DBG.store(false, std::sync::atomic::Ordering::SeqCst)
            }
            */


            let compressed_a = (raw_to_compressed_x(a.0), raw_to_compressed_y(a.1));
            let compressed_b = (raw_to_compressed_x(b.0), raw_to_compressed_y(b.1));

            if compressed_a.0 == compressed_b.0 || compressed_a.1 == compressed_b.1 {
                continue;
            }

            if is_rect_inside_polygon(&grid, compressed_x.len(), compressed_a, compressed_b) {
                max = max.max((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
            }
        }
    }
    max
}

static DBG: AtomicBool = AtomicBool::new(false);
fn debug() -> bool {
    DBG.load(std::sync::atomic::Ordering::SeqCst)
}