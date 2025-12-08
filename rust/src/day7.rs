use std::{fs::File, io::{BufRead, BufReader}};

use super::input_dir;

pub fn parse_manifold<R: BufRead>(mut file: R) -> Result<(Vec<Vec<bool>>, usize), String> {
    let mut line = String::new();

    file.read_line(&mut line).map_err(|e| format!("Error reading file: {e}"))?;
    let start_idx = line.find('S').ok_or_else(|| format!("Can't find start position"))?;

    let mut manifold = Vec::new();

    loop {
        line.clear();
        file.read_line(&mut line).map_err(|e| format!("Error reading file: {e}"))?;
        if line.len() == 0 { break; }

        manifold.push(line.chars().map(|c| c == '^').collect());
    }

    Ok((manifold, start_idx))
}

pub fn run_manifold(manifold: &[Vec<bool>], start_idx: usize) -> (u64, u64) {
    let mut n_splits = 0u64;
    let width = manifold[0].len();

    let mut beams = vec![0; width];
    beams[start_idx] = 1;

    for line in manifold {
        let mut new_beams = vec![0; width];

        for (i, (&beam, &is_splitter)) in beams.iter().zip(line.iter()).enumerate() {
            if beam == 0 { continue; }
            if is_splitter {
                new_beams[i-1] += beam;
                new_beams[i+1] += beam;
                n_splits += 1;
            } else {
                new_beams[i] += beam;
            }
        }
        beams = new_beams;
    }

    (n_splits, beams.iter().sum())
}


pub fn run(test: bool) -> Result<(), String> {
    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day7_test.txt" } else { "day7.txt" });

    let file = File::open(input_path).expect("Failed to open input file");
    let (manifold, start_idx) = parse_manifold(BufReader::new(file))?;

    let (n_splits, n_timelines) = run_manifold(&manifold, start_idx);
    println!("# splits: {n_splits}");
    println!("# timelines: {n_timelines}");

    Ok(())
}