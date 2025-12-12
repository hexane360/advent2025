use std::{fs::File, io::{BufRead, BufReader}, iter::Peekable};

use ndarray::Array2;

use super::input_dir;


#[derive(Debug)]
pub struct Tree {
    // [y, x]
    pub size: [u64; 2],
    pub presents: Vec<u64>,
}


pub fn parse_presents<'a, I: Iterator<Item=&'a String>>(
    lines: &mut Peekable<I>
) -> Result<Vec<Array2<bool>>, String> {
    let mut presents = Vec::new();

    while let Some(line) = lines.peek() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let (before, after) = line.split_once(':').ok_or_else(|| format!("Invalid line: {line}"))?;

        if !after.is_empty() || before.contains('x') {
            // start of trees
            break
        }
        lines.next();

        let mut present: Vec<bool> = Vec::new();
        let mut width: Option<usize> = None;
        let mut height: usize = 0;

        while let Some(line) = lines.next() {
            if line.trim().is_empty() { break; }
            let row: Vec<_> = line.trim().chars().map(|c| c == '#').collect();

            match width {
                Some(w) => { if row.len() != w { return Err(format!("Invalid present, uneven widths")); } },
                None => { width = Some(row.len()) },
            }

            present.extend(row);
            height += 1;
        }

        let width = width.ok_or_else(|| format!("Invalid present, missing any rows"))?;
        presents.push(Array2::from_shape_vec([height, width], present).unwrap());
    }

    Ok(presents)
}


pub fn parse_trees<'a, I: Iterator<Item=&'a String>>(
    lines: &mut Peekable<I>
) -> Result<Vec<Tree>, String> {
    let mut trees = Vec::new();

    for line in lines {
        if line.trim().is_empty() { continue; }

        trees.push(line.split_once(": ").and_then(|(before, after)| {
            let size = before.split('x').rev().map(|s| s.trim().parse::<u64>().ok()).collect::<Option<Vec<u64>>>()?;
            let presents = after.split_whitespace().map(|s| s.parse::<u64>().ok()).collect::<Option<Vec<u64>>>()?;
            let size = size.try_into().ok()?;

            Some(Tree { size, presents })
        }).ok_or_else(|| format!("Invalid tree: {line}"))?);
    }

    Ok(trees)
}


pub fn run(test: bool) -> Result<(), String> {
    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day12_test.txt" } else { "day12.txt" });

    let file = File::open(input_path).expect("Failed to open input file");

    let (presents, trees) = {
        let lines: Vec<String> = BufReader::new(file).lines()
            .map(|r| r.map_err(|e| format!("Error reading file: {e}"))).collect::<Result<_, _>>()?;

        let mut iter = lines.iter().peekable();
        (parse_presents(&mut iter)?, parse_trees(&mut iter)?)
    };

    let areas: Vec<_> = presents.iter()
        .map(|present| present.iter().map(|&v| v as u64).sum::<u64>()).collect();

    let mut n_impossible = 0u64;
    let mut n_possible = 0u64;
    let mut n_unknown = 0u64;

    for tree in trees {
        let n_presents: u64 = tree.presents.iter().sum();
        let required_area: u64 = tree.presents.iter().enumerate().map(|(i, n)| n * areas[i]).sum();
        let n_tiles: u64 = tree.size.iter().map(|v| v / 3).product();

        if required_area > tree.size.iter().product() {
            n_impossible += 1;
            println!("Impossible: Area doesn't fit");
        } else if n_presents <= n_tiles {
            n_possible += 1;
            println!("Trivial: Bounding boxes fit");
        } else {
            n_unknown += 1;
            println!("Unknown");
        }
    }
    println!("Possible: {n_possible}, Impossible: {n_impossible}, unknown: {n_unknown}");

    Ok(())
}