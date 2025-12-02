use core::fmt;
use std::{fs::File, io::{BufRead, BufReader}};

use super::{verbosity, input_dir};

static DIAL_SIZE: i64 = 100;

fn try_parse_line<'a>(line: &'a str) -> Option<(i64, i64)> {
    if line.len() < 2 { return None; }
    Some((
        match line.chars().next().unwrap().to_ascii_uppercase() {
            'L' => -1,
            'R' =>  1,
            _ => return None,
        },
        (&line[1..]).parse().ok()?,
    ))
}

pub fn process<I, E>(lines: I, start_pos: i64, verbosity: u8) -> Result<(i64, i64), String>
where I: IntoIterator<Item = Result<String, E>>,
      E: fmt::Display
{
    let mut pos = start_pos;
    let mut stop_count = 0;
    let mut pass_count = 0;

    for line in lines.into_iter() {
        let line = line.map_err(|e| format!("Error reading file: {}", e))?;
        let (sign, value) = try_parse_line(&line).ok_or_else(|| format!("Invalid line {:?}", line))?;

        let offset = if sign * pos < 0 { DIAL_SIZE - pos } else { pos };

        pass_count += (offset + value) / DIAL_SIZE;
        pos += sign * value;
        pos = pos.rem_euclid(DIAL_SIZE);

        if pos == 0 { stop_count += 1 }

        if verbosity > 0 {
            println!("{:<4} pos: {:>2}, passes: {:>4}, stops: {:>4}", line, pos, pass_count, stop_count);
        }
    }

    Ok((stop_count, pass_count))
}


pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day1_test.txt" } else { "day1.txt" });

    println!("input_path: {:?}", input_path);

    let file = File::open(input_path).expect("Failed to open input file");
    let (stop_count, pass_count) = process(BufReader::new(file).lines(), 50, verbosity)?;

    println!("Stopped at 0 {} times", stop_count);
    println!("Passed 0 {} times", pass_count);

    Ok(())
}