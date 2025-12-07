use std::{fs::File, io::{BufRead, BufReader}};

use super::{input_dir, verbosity};


pub fn read_intervals<R: BufRead>(file: &mut R, verbosity: u8) -> Result<Vec<(u64, u64)>, String> {
    let mut intervals = Vec::new();

    for line in file.lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?;
        let trim = line.trim();
        if trim.len() == 0 {
            break
        }

        let interval: (u64, u64) = trim.split_once('-').and_then(|(l, r)| match (l.parse(), r.parse()) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        }).ok_or_else(|| format!("Invalid range '{}'", trim))?;

        // maintain sort order
        intervals.insert(
            intervals.partition_point(|(start, _)| *start < interval.0),
            interval
        );
    }

    Ok(deduplicate(intervals, verbosity))
}

pub fn deduplicate(intervals: Vec<(u64, u64)>, verbosity: u8) -> Vec<(u64, u64)> {
    let mut new = Vec::new();

    let (mut start, mut end) = match intervals.first() {
        Some(v) => v,
        None => return new,
    };

    if verbosity > 1 {
        println!("starting with range ({start}, {end})");
    }

    for interval in &intervals[1..] {
        if interval.0 <= end {
            // interval overlaps with previous interval
            if verbosity > 1 {
                println!("overlapping range ({start}, {end})");
            }
            end = end.max(interval.1);
            continue
        }
        if verbosity > 1 {
            println!("disjoint range {interval:?}, pushing previous range ({start}, {end})");
        }
        new.push((start, end));
        (start, end) = *interval;
    }

    new.push((start, end));

    if verbosity > 1 {
        println!("pushing final range ({start}, {end})");
    }

    new
}

pub fn check_id(intervals: &[(u64, u64)], id: u64, verbosity: u8) -> bool {
    if verbosity > 0 {
        print!("checking id {id}: ");
    }

    let idx = intervals.partition_point(|(start, _)| *start <= id);

    if idx == 0 {
        if verbosity > 0 {
            println!("before start of intervals")
        }
        false
    } else {
        let &(lower, upper) = &intervals[idx - 1];
        let inside = id <= upper;
        if verbosity > 0 {
            println!("{}inside interval ({lower}, {upper})", if inside { "" } else { "not " });
        }
        inside
    }
}

pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day5_test.txt" } else { "day5.txt" });

    let file = File::open(input_path).expect("Failed to open input file");
    let mut buf = BufReader::new(file);

    let intervals = read_intervals(&mut buf, verbosity)?;

    if verbosity > 1 {
        println!("Final intervals {intervals:?}");
    }

    let mut sum = 0u64;

    for line in buf.lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?; 
        let id = line.trim().parse().map_err(|_| format!("Invalid id: {line}"))?;

        if check_id(&intervals, id, verbosity) {
            sum += 1;
        }
    }

    println!("# fresh IDs: {sum}");

    // since intervals are disjoint, we're good
    let total_fresh: u64 = intervals.iter().map(|(start, end)| end - start + 1).sum();
    println!("total # of fresh: {total_fresh}");

    Ok(())
}