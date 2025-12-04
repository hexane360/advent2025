use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use ndarray::{Array1, Array2, ArrayBase, Axis, Data, Dim, stack};
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};

use super::{input_dir, verbosity};


fn load_array<P: AsRef<Path>>(path: P) -> Result<Array2<u8>, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open input file: {e}"))?;
    let mut rows = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?;
        rows.push(Array1::from_iter(line.bytes().map(|c| (c == b'@') as u8)))
    }

    let views: Vec<_> = rows.iter().map(|arr| arr.view()).collect();
    stack(Axis(0), &views).map_err(|_| format!("Mismatched shapes in rows"))
}

fn print_array<'a, S>(arr: ArrayBase<S, Dim<[usize; 2]>>)
where S: Data<Elem = u8> + 'a
{
    for line in arr.outer_iter() {
        let line: String = line.iter().map(|c| if *c > 0 { '@' } else { '.' }).collect();
        println!("{}", line)
    }
    println!("")
}

#[allow(unused)]
fn print_counts<'a, S>(arr: ArrayBase<S, Dim<[usize; 2]>>)
where S: Data<Elem = u8> + 'a
{
    for line in arr.outer_iter() {
        let line: String = line.iter().map(|c| {
            assert!(*c < 10);
            (c + 48) as char
        }).collect();
        println!("{}", line)
    }
    println!("")
}

pub fn process(mut arr: Array2<u8>, verbosity: u8) -> u64 {
    let n_start: u64 = arr.iter().map(|c| *c as u64).sum();
    let mut n = n_start;

    if verbosity > 0 {
        println!("Initial state:");
        print_array(arr.view());
    }

    // kernel is separable, use 2 1D convolutions
    let kernel1: Array2<u8> = Array2::ones((3, 1));
    let kernel2: Array2<u8> = Array2::ones((1, 3));

    let mut i = 0;

    loop {
        let convolved = ConvExt::conv(
            &ConvExt::conv(&arr, &kernel1, ConvMode::Same, PaddingMode::Zeros).unwrap(),
            &kernel2, ConvMode::Same, PaddingMode::Zeros
        ).unwrap();
        if verbosity > 1 {
            print_counts(convolved.view());
        }

        // 4 neighbors + self
        let mut available = convolved.mapv(|c| if c < 5 { 1u8 } else { 0u8 });
        available &= &arr;
        let n_available: u64 = available.iter().map(|c| *c as u64).sum();
        if n_available == 0 { break; }

        i += 1;
        println!("Step {i}, removed {n_available:3} box(es)");
        arr ^= &available;
        n -= n_available;
        if verbosity > 0 { print_array(arr.view()); }
    }

    println!("Finished in {i} step(s), final state:");
    print_array(arr);
    println!("{n_start} -> {n} boxes (removed {})", n_start - n);
    n_start - n
}


pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day4_test.txt" } else { "day4.txt" });

    let arr = load_array(&input_path)?;
    process(arr, verbosity);

    Ok(())
}