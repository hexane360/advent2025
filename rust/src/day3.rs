use std::{fs::File, io::{BufRead, BufReader}};

use super::{input_dir, verbosity};

#[allow(unused)]
fn bank_voltage_part1(bank: &[u8]) -> u64 {
    let mut max = 0u64;
    let mut max_first = 0u8;

    for first in 0..bank.len()-1 {
        if bank[first] <= max_first {
            // prune the search, it can't be better
            continue
        }
        max_first = max_first.max(bank[first]);
        for second in first+1..bank.len() {
            let val = 10 * (bank[first] as u64) + (bank[second] as u64);
            max = max.max(val);
        }
    }
    max
}

fn bank_voltage_part2(bank: &[u8], n: u8) -> u64 {
    assert!(bank.len() >= n.into(), "Not enough batteries in bank!");
    // base case
    if n < 2 { return *bank.iter().max().unwrap() as u64; }

    let mut max = 0u64;
    let mut max_digit = 0u8;
    let mult_factor = 10u64.pow(n as u32 - 1);

    for i in 0..bank.len() + 1 - n as usize {
        if bank[i] <= max_digit {
            continue
        }
        max_digit = max_digit.max(bank[i]);
        let inner_val = bank_voltage_part2(&bank[i+1..], n - 1);
        max = max.max(mult_factor * (bank[i] as u64) + inner_val);
    }
    max
}


pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day3_test.txt" } else { "day3.txt" });

    let file = File::open(input_path).map_err(|e| format!("Failed to open input file: {e}"))?;

    let mut part1_sum = 0u64;
    let mut part2_sum = 0u64;

    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?;
        let bank: Vec<u8> = line.bytes()
            // ascii digit to u8
            .map(|c| if c >= 48 && c < 58 { Some(c - 48) } else { None } )
            .collect::<Option<Vec<u8>>>().ok_or_else(|| format!("Invalid bank '{line}'"))?;

        let part1_max = bank_voltage_part2(&bank, 2);
        part1_sum += part1_max;

        let part2_max = bank_voltage_part2(&bank, 12);
        part2_sum += part2_max;

        if verbosity > 0 {
            println!("{line}: part 1: {part1_max} part 2: {part2_max}")
        }
    }

    println!("Part 1 sum: {part1_sum}");
    println!("Part 2 sum: {part2_sum}");

    Ok(())
}