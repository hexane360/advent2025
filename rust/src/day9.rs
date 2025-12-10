use std::{collections::HashMap, fmt, fs::File, io::{BufRead, BufReader}};

use itertools::Itertools;
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


struct PolyGrid {
    x_map: HashMap<u64, usize>,
    y_map: HashMap<u64, usize>,
    grid: Array2<bool>,
}

impl PolyGrid {
    pub fn new(x_map: HashMap<u64, usize>, y_map: HashMap<u64, usize>) -> Self {
        let size = [y_map.len(), x_map.len()];
        Self {
            x_map, y_map,
            grid: Array2::from_elem(size, false),
        }
    }

    pub fn compress_point(&self, pt: &[u64; 2]) -> [usize; 2] {
        [*self.x_map.get(&pt[0]).expect("Missing point in x_map"), *self.y_map.get(&pt[1]).expect("Missing point in y_map")]
    }

    pub fn make(poly: &[[u64; 2]]) -> Self {
        let x_map = compress_points(poly, 0);
        let y_map = compress_points(poly, 1);
        let mut grid = Self::new(x_map, y_map);

        let mut compressed_poly: Vec<[usize; 2]> = poly.iter().map(|pt| grid.compress_point(pt)).collect();
        // close polygon
        compressed_poly.push(compressed_poly[0]);

        grid.draw_poly_inside(&compressed_poly);
        grid.draw_poly_outside(&compressed_poly);
        grid
    }

    fn draw_poly_inside(&mut self, poly: &[[usize; 2]]) {
        for (start, end) in poly.iter().zip(&poly[1..]) {
            let x = start[0].max(end[0]);
            let (start_y, end_y) = if start[1] > end[1] { (end[1], start[1]) } else { (start[1], end[1]) };

            for y in start_y..=end_y {
                //self.grid[[y, x]] = true;
                for x in x+1..self.grid.shape()[1] {
                    self.grid[[y, x]] ^= true;
                }
            }
        }
    }

    fn draw_poly_outside(&mut self, poly: &[[usize; 2]]) {
        for (start, end) in poly.iter().zip(&poly[1..]) {
            if start[0] == end[0] {
                // vertical segment
                let (start_y, end_y) = if start[1] > end[1] { (end[1], start[1]) } else { (start[1], end[1]) };

                for y in start_y..=end_y {
                    self.grid[[y, start[0]]] = true;
                }
            } else if start[1] == end[1] {
                // horzontal segment
                let (start_x, end_x) = if start[0] > end[0] { (end[0], start[0]) } else { (start[0], end[0]) };
                for x in start_x..=end_x {
                    self.grid[[start[1], x]] = true;
                }
            } else {
                panic!("Diagonal line segment");
            }
        }
    }

    pub fn part2_rect_valid(&self, coord1: &[u64; 2], coord2: &[u64; 2]) -> bool {
        let mut coord1 = self.compress_point(coord1);
        let mut coord2 = self.compress_point(coord2);
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


pub fn compress_points(points: &[[u64; 2]], idx: usize) -> HashMap<u64, usize> {
    let mut indices = points.iter().map(|v| v[idx]).unique().collect_vec();
    indices.sort();
    indices.into_iter().enumerate().map(|(i, v)| (v, i)).collect()
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