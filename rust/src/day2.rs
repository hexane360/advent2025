use std::{fs::File, io::{BufReader, BufRead}};

use super::{input_dir, verbosity};

struct MultiIter<T, I>
where I: Iterator<Item = T> {
    iters: Vec<I>
}

impl<T, I> MultiIter<T, I>
where I: Iterator<Item = T>
{
    pub fn new<IntoI>(iters: Vec<IntoI>) -> Self
    where IntoI: IntoIterator<Item = T, IntoIter = I>
    {
        Self {
            iters: iters.into_iter().map(|i| i.into_iter()).collect()
        }
    }
}

impl<T, I> Iterator for MultiIter<T, I>
where I: Iterator<Item = T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iters.iter_mut().map(|i| i.next()).collect()
    }
}

fn split_n<'a, T>(slice: &'a [T], n: usize) -> Option<Vec<&'a [T]>> {
    if slice.len() % n != 0 { return None; }
    let len = slice.len() / n;
    Some((0..n).map(|i| &slice[i * len..(i + 1) * len]).collect())
}

fn all_equal<T: PartialEq>(chars: &[T]) -> bool {
    if chars.len() == 0 { true } else {
        let first_char = &chars[0];
        chars[1..].iter().all(|c| *c == *first_char)
    }
}

fn parse_range<'a>(range: &'a str) -> Result<(u64, u64), String> {
    range.trim().split_once('-').and_then(|(l, r)| match (l.parse(), r.parse()) {
        (Ok(l), Ok(r)) => Some((l, r)),
        _ => None,
    }).ok_or_else(|| format!("Invalid range '{}'", range))
}

fn is_repeated(s: &str, n: usize) -> bool {
    match split_n(s.as_bytes(), n) {
        None => false,
        Some(slices) => MultiIter::new(slices).all(|chars| all_equal(&chars))
    }
}

fn check_range(range: (u64, u64), verbosity: u8) -> u64 {
    let mut sum: u64 = 0;
    if verbosity > 0 {
        println!("range: ({}, {})", range.0, range.1);
    }
    for id in range.0..=range.1 {
        let s = id.to_string();
        for repeat_n in 2..=s.len() {
            if is_repeated(&s, repeat_n) {
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