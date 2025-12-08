use std::{ascii, fs::File, io::{BufRead, BufReader, Seek, SeekFrom}, mem};

use ndarray::{Array2, Axis};

use super::{input_dir, verbosity};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Times,
    Plus,
}

struct Problem {
    arguments: Vec<Vec<u64>>,
    operators: Vec<Operator>,
}

impl Problem {
    pub fn parse_part1<R: BufRead>(mut file: R) -> Result<Problem, String> {
        let mut n_rows: usize = 0;

        let mut arguments: Vec<u64> = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();
            file.read_line(&mut line).map_err(|e| format!("Error reading file: {e}"))?;

            if let Some('*' | '+') | None = line.trim_start().chars().next() {
                break;
            }

            for word in line.split_whitespace() {
                arguments.push(word.parse().map_err(|_| format!("Invalid argument: '{word}'"))?);
            }

            n_rows += 1;
        }

        if line.len() == 0 {
            return Err("File missing operator row".to_owned());
        }

        let operators: Vec<Operator> = line.split_whitespace().map(|word| {
            match word {
                "*" => Ok(Operator::Times),
                "+" => Ok(Operator::Plus),
                _ => Err(format!("Invalid operator '{word}'")),
            }
        }).collect::<Result<Vec<Operator>, String>>()?;

        let arguments = Array2::from_shape_vec((n_rows, operators.len()), arguments).map_err(|_| format!("Invalid shape"))?;

        Ok(Self {
            arguments: arguments.axis_iter(Axis(1)).map(|v| v.to_vec()).collect(), // transpose, keep row major
            operators,
        })
    }

    pub fn parse_part2<R: BufRead>(mut file: R) -> Result<Problem, String> {
        let mut n_rows: usize = 0;
        let mut lines: Vec<ascii::Char> = Vec::new();

        let mut line = String::new();
        loop {
            line.clear();
            file.read_line(&mut line).map_err(|e| format!("Error reading file: {e}"))?;
            if line.ends_with('\n') { line.pop(); }

            if let Some('*' | '+') | None = line.trim_start().chars().next() {
                break;
            }
            lines.extend(line.as_ascii().ok_or_else(|| format!("Non-ascii chars in file"))?);
            n_rows += 1;
        }

        let operators: Vec<Operator> = line.split_whitespace().map(|word| {
            match word {
                "*" => Ok(Operator::Times),
                "+" => Ok(Operator::Plus),
                _ => Err(format!("Invalid operator '{word}'")),
            }
        }).collect::<Result<Vec<Operator>, String>>()?;

        let mat = Array2::from_shape_vec((n_rows, line.len()), lines).map_err(|_| format!("Invalid shape"))?;

        let mut arguments: Vec<Vec<u64>> = Vec::new();
        let mut buf: Vec<u64> = Vec::new();

        for col in mat.axis_iter(Axis(1)) {
            let v = col.to_vec();
            let col = v.as_slice().as_str().trim();
            if col.len() == 0 {
                arguments.push(mem::replace(&mut buf, Vec::new()));
            } else {
                buf.push(col.parse().map_err(|_| format!("Invalid argument {col}"))?);
            }
        }
        arguments.push(buf);

        Ok(Self {
            arguments, operators
        })
    }

    pub fn solve(&self, verbosity: u8) -> u64 {
        let mut total: u64 = 0;

        for (args, &operator) in self.arguments.iter().zip(&self.operators) {
            let answer: u64 = match operator {
                Operator::Plus => args.iter().sum(),
                Operator::Times => args.iter().product(),
            };
            total += answer;
            if verbosity > 0 {
                match operator {
                    Operator::Plus => println!("Sum of {:?} = {answer}", args.as_slice()),
                    Operator::Times => println!("Product of {:?} = {answer}", args.as_slice()),
                }
            }
        }
        total
    }
}

pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day6_test.txt" } else { "day6.txt" });

    let mut file = File::open(input_path).expect("Failed to open input file");

    let problem = Problem::parse_part1(BufReader::new(&mut file))?;
    println!("Grand total (Part 1): {}", problem.solve(verbosity));

    file.seek(SeekFrom::Start(0)).unwrap();
    let problem = Problem::parse_part2(BufReader::new(file))?;
    println!("Grand total (Part 2): {}", problem.solve(verbosity));
    Ok(())
}