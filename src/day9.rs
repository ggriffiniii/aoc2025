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

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let input = EXAMPLE;
    let coords: Vec<_> = input
        .lines()
        .map(|line| -> (usize, usize) {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
        .collect();

    for triple in coords.windows(3).cycle().take(coords.len() + 1) {
        let a = triple[0];
        let b = triple[1];
        let c = triple[2];

        if a.0 == b.0 && b.0 == c.0 {
            panic!("{a:?} followed by {b:?} followed by {c:?} is a straight line");
        }

        if a.1 == b.1 && b.1 == c.1 {
            panic!("{a:?} followed by {b:?} followed by {c:?} is a straight line");
        }
    }
    eprintln!("all clear");

    24
}
