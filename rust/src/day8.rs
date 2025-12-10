use std::{collections::HashMap, fs::File, hash::Hash, io::{BufRead, BufReader}};

use itertools::Itertools;
use petgraph::unionfind::UnionFind;

use super::{input_dir, verbosity};


fn parse_coords<R: BufRead>(file: R) -> Result<Vec<[u64; 3]>, String> {
    let mut coords = Vec::new();

    for line in file.lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?;

        coords.push(line.splitn(3, ',').map(|w| w.parse::<u64>().ok())
            .collect::<Option<Vec<u64>>>()
            .and_then(|coord| coord.as_array().map(|arr| arr.to_owned()))
            .ok_or_else(|| format!("Invalid coordinate: {line}"))?
        );
    }

    Ok(coords)
}


pub fn sqdist(coord1: &[u64; 3], coord2: &[u64; 3]) -> u64 {
    coord1.iter().zip(coord2).map(|(&l, &r)| l.abs_diff(r).pow(2)).sum()
}


fn count_occurrences<T: Eq + Hash, I: IntoIterator<Item = T>>(iter: I) -> Vec<u32> {
    let mut components: HashMap<T, u32> = HashMap::new();
    for label in iter.into_iter() {
        *components.entry(label).or_default() += 1;
    }
    let mut components: Vec<u32> = components.values().copied().collect();
    components.sort_by(|a, b| b.cmp(a));
    components
}


pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day8_test.txt" } else { "day8.txt" });

    let file = File::open(input_path).expect("Failed to open input file");
    let coords = parse_coords(BufReader::new(file))?;

    if verbosity > 0 {
        println!("coords:");
        for coord in coords.iter() {
            println!("{:3}", coord.iter().format(", "));
        }
    }

    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(coords.len() * (coords.len() - 1) / 2);

    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            pairs.push((i as u32, j as u32));
        }
    }
    pairs.sort_by_cached_key(|&(i, j)| sqdist(&coords[i as usize], &coords[j as usize]));

    let mut union_find: UnionFind<u32> = UnionFind::new(coords.len());

    let (part1_pairs, part2_pairs) = pairs.split_at(if test { 10 } else { 1000 });

    for &(i, j) in part1_pairs.iter() {
        union_find.union(i, j);
    }
    let components = count_occurrences(union_find.clone().into_labeling());

    println!("Part 1 sizes: {:?}", components);
    println!("Part 1 product: {}", components[..3].iter().product::<u32>());

    let mut n_components = components.len();

    for &(i, j) in part2_pairs.iter() {
        if union_find.union(i, j) {
            n_components -= 1;
            if n_components == 1 {
                let product = coords[i as usize][0] * coords[j as usize][0];
                println!("Part 2 product: {product}");
                break
            }
        }
    }

    Ok(())
}