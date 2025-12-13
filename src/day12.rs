use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Shape {
    row_len: usize,
    data: Vec<bool>,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities: Vec<usize>,
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    let mut sections: Vec<_> = input.split("\n\n").collect();
    let regions = sections.pop().unwrap();
    let shapes: Vec<_> = sections
        .into_iter()
        .map(|shape_input| {
            let (_shape_id, shape) = shape_input.split_once(":\n").unwrap();
            let row_len = shape.split_once("\n").unwrap().0.len();
            let data: Vec<_> = shape
                .as_bytes()
                .iter()
                .filter_map(|&b| match b {
                    b'.' => Some(false),
                    b'#' => Some(true),
                    _ => None,
                })
                .collect();
            // Every shape in the example and my input data is 3x3 box
            assert_eq!(row_len, 3);
            assert_eq!(data.len(), 9);
            Shape { data, row_len }
        })
        .collect();

    let regions: Vec<_> = regions
        .lines()
        .map(|region_input| {
            let (size, quantities) = region_input.split_once(": ").unwrap();
            let (width, height) = size.split_once("x").unwrap();
            let quantities = quantities
                .split_whitespace()
                .map(|qty| qty.parse().unwrap())
                .collect();
            Region {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                quantities,
            }
        })
        .collect();

    regions
        .iter()
        .filter(|region| {
            if fits_without_transformations(region) {
                true
            } else if will_never_fit(region, &shapes) {
                false
            } else {
                todo!()
            }
        })
        .count()
}

fn fits_without_transformations(region: &Region) -> bool {
    let total_shapes: usize = region.quantities.iter().sum();
    // We asserted when parsing the input that all shapes were 3x3 boxes.
    let shapes_that_fit_within_region = (region.width / 3) * (region.height / 3);
    total_shapes <= shapes_that_fit_within_region
}

fn will_never_fit(region: &Region, shapes: &[Shape]) -> bool {
    let region_area = region.width * region.height;
    let total_shape_area: usize = region
        .quantities
        .iter()
        .copied()
        .enumerate()
        .map(|(shape_id, qty)| {
            let shape_area = shapes[shape_id].data.iter().copied().filter(|&x| x).count();
            shape_area * qty
        })
        .sum();
    total_shape_area > region_area
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    42
}
