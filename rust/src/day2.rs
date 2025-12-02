use std::{fs::File, io::{BufReader, BufRead}};

use super::{input_dir, verbosity};

fn parse_range<'a>(range: &'a str) -> Result<(u64, u64), String> {
    range.trim().split_once('-').and_then(|(l, r)| match (l.parse(), r.parse()) {
        (Ok(l), Ok(r)) => Some((l, r)),
        _ => None,
    }).ok_or_else(|| format!("Invalid range '{}'", range))
}

fn is_repeated(s: &str, n: usize) -> bool {
    if s.len() % n != 0 { return false; }
    s[n..].bytes().zip(
        std::iter::repeat(&s[..n]).flat_map(|s| s.bytes())
    ).all(|(l, r)| l == r)
}

fn check_range(range: (u64, u64), verbosity: u8) -> u64 {
    let mut sum: u64 = 0;
    if verbosity > 0 {
        println!("range: ({}, {})", range.0, range.1);
    }
    for id in range.0..=range.1 {
        let s = id.to_string();
        for repeat_len in 1..=s.len()/2 {
            if is_repeated(&s, repeat_len) {
                if verbosity > 0 {
                    println!("  invalid id {}", id);
                }
                sum = sum.checked_add(id).expect("Overflow");
                break;
            }
        }
    }
    sum
}

pub fn run() -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push("day2.txt");

    let file = File::open(input_path).expect("Failed to open input file");

    let mut sum: u64 = 0;

    for range in BufReader::new(file).split(b',') {
        let range = parse_range(
            range.as_ref().map_err(|e| format!("Error reading file: {}", e))
                .and_then(|b| str::from_utf8(b).map_err(|_| format!("Invalid utf-8 in file")))?
        )?;
        sum = sum.checked_add(check_range(range, verbosity)).ok_or_else(|| format!("Overflow"))?;
    }
    println!("sum: {}", sum);
    Ok(())
}