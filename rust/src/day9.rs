use std::{fmt, fs::File, io::{BufRead, BufReader}, iter};

use ndarray::Array2;

use super::{input_dir, verbosity};


fn parse_tiles<R: BufRead>(file: R) -> Result<Vec<[u64; 2]>, String> {
    let mut coords = Vec::new();

    for line in file.lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?;

        coords.push(line.splitn(2, ',').map(|w| w.parse::<u64>().ok())
            .collect::<Option<Vec<u64>>>()
            .and_then(|coord| coord.as_array().map(|arr| arr.to_owned()))
            .ok_or_else(|| format!("Invalid coordinate: {line}"))?
        );
    }

    Ok(coords)
}


pub fn square_area(coord1: &[u64; 2], coord2: &[u64; 2]) -> u64 {
    coord1.iter().zip(coord2).map(|(&l, &r)| l.abs_diff(r) + 1).product()
}


fn poly_segments(poly: &[[u64; 2]]) -> impl Iterator<Item=(&[u64; 2], &[u64; 2])> {
    poly.iter().zip(&poly[1..]).chain(iter::once((poly.last().unwrap(), &poly[0])))
}


struct PolyGrid {
    origin: [u64; 2],
    grid: Array2<bool>,
}

impl PolyGrid {
    pub fn new(min: [u64; 2], max: [u64; 2]) -> Self {
        let size: [usize; 2] = [(max[1] + 1 - min[1]) as usize, (max[0] + 1 - min[0]) as usize];
        println!("grid size: {size:?}");
        println!("min: {min:?}  max: {max:?}");
        Self {
            origin: min,
            grid: Array2::from_elem(size, false),
        }
    }

    pub fn make(poly: &[[u64; 2]]) -> Self {
        let min: [u64; 2] = [0, 1].map(|i| poly.iter().map(|v| v[i]).min().unwrap());
        let max: [u64; 2] = [0, 1].map(|i| poly.iter().map(|v| v[i]).max().unwrap());

        let mut grid = Self::new(min, max);
        grid.draw_poly_inside(poly);
        grid.draw_poly_outside(poly);
        grid
    }

    pub fn draw_poly_inside(&mut self, poly: &[[u64; 2]]) {
        for (start, end) in poly_segments(poly) {
            //if start[0] != end[0] { continue; }
            // vertical segment

            let x = (start[0].max(end[0]) - self.origin[0]) as usize;
            let start_y = (start[1] - self.origin[1]) as usize;
            let end_y = (end[1] - self.origin[1]) as usize;
            let (start_y, end_y) = if start_y > end_y { (end_y, start_y) } else { (start_y, end_y) };

            for y in start_y..=end_y {
                //self.grid[[y, x]] = true;
                for x in x+1..self.grid.shape()[1] {
                    self.grid[[y, x]] ^= true;
                }
            }
        }
    }

    pub fn draw_poly_outside(&mut self, poly: &[[u64; 2]]) {
        for (start, end) in poly_segments(poly) {
            let start_x = (start[0] - self.origin[0]) as usize;
            let start_y = (start[1] - self.origin[1]) as usize;

            if start[0] == end[0] {
                // vertical segment
                let end_y = (end[1] - self.origin[1]) as usize;
                let (start_y, end_y) = if start_y > end_y { (end_y, start_y) } else { (start_y, end_y) };

                for y in start_y..=end_y {
                    self.grid[[y, start_x]] = true;
                }
            } else if start[1] == end[1] {
                // horzontal segment
                let end_x = (end[0] - self.origin[0]) as usize;
                let (start_x, end_x) = if start_x > end_x { (end_x, start_x) } else { (start_x, end_x) };
                for x in start_x..=end_x {
                    self.grid[[start_y, x]] = true;
                }
            } else {
                panic!("Diagonal line segment");
            }
        }
    }

    pub fn part2_rect_valid(&self, coord1: &[u64; 2], coord2: &[u64; 2]) -> bool {
        let mut coord1 = [(coord1[0] - self.origin[0]) as usize, (coord1[1] - self.origin[1]) as usize];
        let mut coord2 = [(coord2[0] - self.origin[0]) as usize, (coord2[1] - self.origin[1]) as usize];

        if coord1[0] > coord2[0] { std::mem::swap(&mut coord1[0], &mut coord2[0]); }
        if coord1[1] > coord2[1] { std::mem::swap(&mut coord1[1], &mut coord2[1]); }

        // vertical segments
        for x in &[coord1[0], coord2[0]] {
            for y in coord1[1]..=coord2[1] { if !self.grid[[y, *x]] { return false; } }
        }

        // horizontal segments
        for y in &[coord1[1], coord2[1]] {
            for x in coord1[0] + 1..coord2[0] { if !self.grid[[*y, x]] { return false; } }
        }

        true
    }
}

impl fmt::Display for PolyGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.grid.outer_iter() {
            writeln!(f, "{}", line.iter().map(|&b| if b { "X" } else { "." }).collect::<String>())?;
        }
        Ok(())
    }
}

pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day9_test.txt" } else { "day9.txt" });

    let file = File::open(input_path).expect("Failed to open input file");
    let tiles: Vec<[u64; 2]> = parse_tiles(BufReader::new(file))?;

    let mut max_area: u64 = 0;

    // part 1
    // this could be much better with some pruning
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let area = square_area(&tiles[i], &tiles[j]);
            max_area = max_area.max(area);

            if verbosity > 1 {
                println!("{:?} - {:?} area: {area}", &tiles[i], &tiles[j])
            }
        }
    }
    println!("Part 1 largest area: {}", max_area);

    // part 2

    let grid = PolyGrid::make(&tiles);

    if verbosity > 0 {
        println!("Grid:\n{}", grid);
    }

    max_area = 0;
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let area = square_area(&tiles[i], &tiles[j]);
            if area <= max_area { continue; }

            if grid.part2_rect_valid(&tiles[i], &tiles[j]) {
                max_area = area;
            }
        }
    }
    println!("Part 2 largest area: {}", max_area);

    Ok(())
}