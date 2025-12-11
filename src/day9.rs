
use std::{ops::{Index, IndexMut}, sync::atomic::AtomicBool};

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

/// A Grid the uses coordinate compression to track grid data on relative
/// position.
#[derive(Debug)]
struct CompressedGrid<T>{
    x_coords: Vec<isize>,
    y_coords: Vec<isize>,
    data: Vec<T>,
}
impl<T> CompressedGrid<T>
where 
    T: Clone,
{
    fn new(coords: impl IntoIterator<Item=(isize, isize)>, fill: T) -> Self {
        let mut x_coords = Vec::new();
        let mut y_coords = Vec::new();
        for (x,y) in coords {
            x_coords.push(x);
            y_coords.push(y);
        }
        x_coords.sort();
        x_coords.dedup();
        y_coords.sort();
        y_coords.dedup();
        let data = vec![fill; (x_coords.len()*y_coords.len()) as usize];
        Self{x_coords, y_coords, data}
    }
}

impl<T> CompressedGrid<T> {
    fn to_raw(&self, idx: (isize, isize)) -> (RawX, RawY) {
        let x = self.x_coords.binary_search(&idx.0).expect("indexing by invalid x coordinate");
        let y = self.y_coords.binary_search(&idx.1).expect("indexing by invalid y coordinate");
        (RawX(x), RawY(y))
    }
    fn get(&self, (x, y): (RawX, RawY)) -> Option<&T> {
        self.data.get(dbg!(y.0 * self.row_len() + x.0))
    }
    fn row_len(&self) -> usize {
        self.x_coords.len()
    }
    fn num_rows(&self) -> usize {
        self.y_coords.len()
    }
    fn row_ids(&self) -> impl Iterator<Item=RawY> {
        (0..self.num_rows()).map(RawY)
    }
    fn col_ids(&self) -> impl Iterator<Item=RawX> {
        (0..self.row_len()).map(RawX)
    }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,Ord,PartialOrd)]
struct RawX(usize);
impl std::ops::Add<usize> for RawX {
    type Output = RawX;

    fn add(self, rhs: usize) -> Self::Output {
        RawX(self.0 + rhs)
    }
}
impl std::ops::Sub<usize> for RawX {
    type Output = RawX;

    fn sub(self, rhs: usize) -> Self::Output {
        RawX(self.0 - rhs)
    }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,Ord,PartialOrd)]
struct RawY(usize);
impl std::ops::Add<usize> for RawY {
    type Output = RawY;

    fn add(self, rhs: usize) -> Self::Output {
        RawY(self.0 + rhs)
    }
}
impl std::ops::Sub<usize> for RawY {
    type Output = RawY;

    fn sub(self, rhs: usize) -> Self::Output {
        RawY(self.0 - rhs)
    }
}

impl<T> Index<(isize, isize)> for CompressedGrid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &T {
        let x = self.x_coords.binary_search(&index.0).expect("indexing by invalid x coordinate");
        let y = self.y_coords.binary_search(&index.1).expect("indexing by invalid y coordinate");
        &self.data[y * self.row_len() + x]
    }
}
impl<T> IndexMut<(isize, isize)> for CompressedGrid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut T {
        let x = self.x_coords.binary_search(&index.0).expect("indexing by invalid x coordinate");
        let y = self.y_coords.binary_search(&index.1).expect("indexing by invalid y coordinate");
        let row_len = self.row_len();
        &mut self.data[y * row_len + x]
    }

}
impl<T> Index<(RawX, RawY)> for CompressedGrid<T> {
    type Output = T;

    fn index(&self, index: (RawX, RawY)) -> &T {
        let x = index.0.0;
        let y = index.1.0;
        &self.data[y * self.row_len() + x]
    }
}
impl<T> IndexMut<(RawX, RawY)> for CompressedGrid<T> {
    fn index_mut(&mut self, index: (RawX, RawY)) -> &mut T {
        let x = index.0.0;
        let y = index.1.0;
        let row_len = self.row_len();
        &mut self.data[y * row_len + x]
    }
}

fn is_rect_inside_polygon(grid: &CompressedGrid<usize>, a: (isize, isize), b: (isize,isize)) -> bool {
    let a = grid.to_raw(a);
    let b = grid.to_raw(b);

    let [x_min, x_max] = std::cmp::minmax(a.0, b.0);
    let [y_min, y_max] = std::cmp::minmax(a.1, b.1);

    let s = grid[(x_max, y_max)];
    let left = grid.get((x_min-1, y_max)).copied().unwrap_or(0);
    let top = grid.get((x_max, y_min-1)).copied().unwrap_or(0);
    let corner = grid.get((x_min-1, y_min-1)).copied().unwrap_or(0);
    let area_inside_polygon = s - top - left + corner;

    let rect_area = ((x_max.0 - x_min.0) + 1) * ((y_max.0 - y_min.0) + 1);

    /*
    if debug() {
        eprintln!("({x_min},{y_min}) -> ({x_max},{y_max})");
        dbg!(&area_inside_polygon);
        dbg!(&rect_area);
    }
    */

    area_inside_polygon == rect_area
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let input = EXAMPLE;
    let coords: Vec<_> = input
        .lines()
        .map(|line| -> (isize, isize) {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
        .collect();

    // Create a grid that contains vertical columns.
    #[repr(u8)]
    #[derive(Debug,Copy,Clone,Eq,PartialEq)]
    enum WallTile {
        Empty = 0,
        Corner = 1,
        Surface = 2,
    }
    let mut cols = CompressedGrid::new(coords.clone(), WallTile::Empty);
    for ci in 0..coords.len() {
        let a = cols.to_raw(coords[ci]);
        let b = cols.to_raw(coords[(ci+1) % coords.len()]);

        if a.0 != b.0 {
            continue;
        }

        let x = a.0;
        let [min_y, max_y] = std::cmp::minmax(a.1, b.1);
        for y in min_y.0..=max_y.0 {
            let y = RawY(y);
            let tile = if y == min_y || y == max_y { WallTile::Corner } else { WallTile::Surface };
            cols[(x, y)] = tile;
        }
    }

    /*
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
    */

    // Create a grid where coordinates inside the polygon are true.
    let mut grid = CompressedGrid::new(coords.clone(), 0usize);
    // For each row start assuming we're outside the polygon, for each wall we
    // encounter invert whether we're inside or outside.
    for y in cols.row_ids() {
        let mut inside = false;
        for x in cols.col_ids() {
            let inside_for_cell = match cols[(x,y)] {
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
            grid[(x,y)] = inside_for_cell as usize;
        }
    }

    if debug() {
        for y in grid.row_ids() {
            for x in grid.col_ids() {
                eprint!("{}", grid[(x,y)]);
            }
            eprintln!("");
        }
        println!("");
    }

    // Turns the grid into a prefix sum to allow for efficient calculation of
    // the sum of any rectangle.
    for x in cols.col_ids() {
        for y in cols.row_ids() {
            let left = grid.get((x-1, y)).copied().unwrap_or(0);
            let above = grid.get((x, y-1)).copied().unwrap_or(0);
            let corner = grid.get((x-1, y-1)).copied().unwrap_or(0);

            eprintln!("updating ({x:?},{y:?}) with {}", dbg!(left)+dbg!(above)-dbg!(corner));

            grid[(x, y)] +=  left + above - corner;
        }
    }

    if debug() {
        for y in grid.row_ids() {
            for x in grid.col_ids() {
                eprint!("{:02} ", grid[(x,y)]);
            }
            eprintln!("");
        }
        println!("");
    }

    let mut max = 0;
    for i in 0..coords.len() {
        for j in i + 1 .. coords.len() {
            let a = coords[i];
            let b = coords[j];
            if a == (9,5) && b == (2,3) {
                DBG.store(true, std::sync::atomic::Ordering::SeqCst)
            } else {
                DBG.store(false, std::sync::atomic::Ordering::SeqCst)
            }

            /*
            let compressed_a = (raw_to_compressed_x(a.0), raw_to_compressed_y(a.1));
            let compressed_b = (raw_to_compressed_x(b.0), raw_to_compressed_y(b.1));

            if compressed_a.0 == compressed_b.0 || compressed_a.1 == compressed_b.1 {
                continue;
            }
            */

            dbg!((a, b));
            if dbg!(is_rect_inside_polygon(&grid, a, b)) {
                let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
                dbg!(area);
                max = max.max(area);
            }
        }
    }
    max
}

static DBG: AtomicBool = AtomicBool::new(true);
fn debug() -> bool {
    DBG.load(std::sync::atomic::Ordering::SeqCst)
}