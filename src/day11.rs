use std::collections::{HashMap, hash_map::Entry};

use aoc_runner_derive::aoc;
#[derive(Debug)]
struct IDMap<'a> {
    map: HashMap<&'a str, usize>,
    next_id: usize,
}
impl<'a> IDMap<'a> {
    fn new() -> Self {
        IDMap {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    fn get_id(&mut self, name: &'a str) -> usize {
        match self.map.entry(name) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let id = self.next_id;
                entry.insert(id);
                self.next_id += 1;
                id
            }
        }
    }
}

fn get_path_counts(
    device_id: usize,
    devices: &[Vec<usize>],
    path_counts: &mut [Option<usize>],
) -> usize {
    if let Some(count) = path_counts[device_id] {
        return count;
    }
    let num_paths = devices[device_id]
        .iter()
        .map(|&neighbor| get_path_counts(neighbor, devices, path_counts))
        .sum();
    path_counts[device_id] = Some(num_paths);
    num_paths
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    //let input = EXAMPLE;
    let mut id_map = IDMap::new();
    let mut devices = Vec::new();
    for line in input.lines() {
        let (device, neighbors_input) = line.split_once(": ").unwrap();
        let device_id = id_map.get_id(device);
        let neighbors: Vec<_> = neighbors_input
            .split_whitespace()
            .map(|neighbor| id_map.get_id(neighbor))
            .collect();
        devices.resize(id_map.next_id, Vec::new());
        devices[device_id] = neighbors;
    }

    let mut path_counts = vec![None; id_map.next_id];
    let out = id_map.get_id("out");
    path_counts[out] = Some(1);
    let you = id_map.get_id("you");
    get_path_counts(you, &devices, &mut path_counts)
}

#[derive(Debug, Copy, Clone)]
enum Needs {
    Dac = 0,
    Fft = 1,
    DacAndFft = 2,
    None = 3,
}

fn get_path_counts_from_srv(
    device_id: usize,
    devices: &[Vec<usize>],
    mut needs: Needs,
    path_counts: &mut [[Option<usize>; 4]],
    dac: usize,
    fft: usize,
) -> usize {
    if device_id == dac {
        needs = match needs {
            Needs::Dac => Needs::None,
            Needs::Fft => Needs::Fft,
            Needs::DacAndFft => Needs::Fft,
            Needs::None => Needs::None,
        };
    }
    if device_id == fft {
        needs = match needs {
            Needs::Dac => Needs::Dac,
            Needs::Fft => Needs::None,
            Needs::DacAndFft => Needs::Dac,
            Needs::None => Needs::None,
        };
    }
    if let Some(count) = path_counts[device_id][needs as usize] {
        return count;
    }

    let num_paths = devices[device_id]
        .iter()
        .map(|&neighbor| get_path_counts_from_srv(neighbor, devices, needs, path_counts, dac, fft))
        .sum();
    path_counts[device_id][needs as usize] = Some(num_paths);
    num_paths
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    //let input = EXAMPLE;
    let mut id_map = IDMap::new();
    let mut devices = Vec::new();
    for line in input.lines() {
        let (device, neighbors_input) = line.split_once(": ").unwrap();
        let device_id = id_map.get_id(device);
        let neighbors: Vec<_> = neighbors_input
            .split_whitespace()
            .map(|neighbor| id_map.get_id(neighbor))
            .collect();
        devices.resize(id_map.next_id, Vec::new());
        devices[device_id] = neighbors;
    }

    let out = id_map.get_id("out");
    let srv = id_map.get_id("svr");
    let dac = id_map.get_id("dac");
    let fft = id_map.get_id("fft");
    let mut path_counts = vec![[None; 4]; id_map.next_id];
    path_counts[out] = [Some(0), Some(0), Some(0), Some(1)]; // initialize the out path count as one that needs neither Dac nor Fft.
    dbg!(&id_map);
    dbg!(&devices.len());
    dbg!(&path_counts.len());
    get_path_counts_from_srv(srv, &devices, Needs::DacAndFft, &mut path_counts, dac, fft)
}
