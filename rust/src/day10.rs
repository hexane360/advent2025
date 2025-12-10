use std::{collections::{HashSet, VecDeque}, fmt, fs::File, hash::Hash, io::{BufRead, BufReader}, str::FromStr, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;
use highs::{ColProblem, HighsModelStatus, Sense};

use super::{input_dir, verbosity};

static MACHINE_RE: OnceLock<Regex> = OnceLock::new();
static BUTTONS_RE: OnceLock<Regex> = OnceLock::new();

struct MachineSpec {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    voltages: Vec<u64>,
}

impl MachineSpec {
    pub fn sort_buttons(&mut self) {
        self.buttons.sort_by_key(|b| -(b.len() as isize))
    }
}

impl FromStr for MachineSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = MACHINE_RE.get_or_init(|| Regex::new(
            r"^\[(?<lights>[.#]+)\](?<buttons>(?: \(\d+(?:,\d+)*\))+) \{(?<voltages>[\d,]+)\}$"
        ).expect("Invalid machine regex"));

        let buttons_re = BUTTONS_RE.get_or_init(|| Regex::new(
            r"\((\d+(?:,\d+)*)\)"
        ).expect("Invalid buttons regex"));

        re.captures(s.trim()).and_then(|caps| {
            let lights = caps["lights"].chars().map(|c| c == '#').collect();
            let voltages = caps["voltages"].split(',').map(|v| v.trim().parse::<u64>().ok()).collect::<Option<_>>()?;
            let buttons = buttons_re.captures_iter(&caps["buttons"]).map(|caps| {
                caps[1].split(',').map(|v| v.trim().parse::<usize>().ok()).collect::<Option<_>>()
            }).collect::<Option<_>>()?;

            Some(Self {
                lights, buttons, voltages
            })
        }).ok_or_else(|| format!("Invalid machine specification: {s}"))
    }
}

impl fmt::Display for MachineSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] ", self.lights.iter().map(|&l| if l { '#' } else { '.' } ).collect::<String>())?;
        for button in self.buttons.iter() {
            write!(f, "({}) ", button.iter().map(|&b| b.to_string()).join(","))?;
        }
        write!(f, "{{{}}}", self.voltages.iter().map(|&v| v.to_string()).join(","))
    }
}

#[derive(Clone)]
struct MachinePart1<'a> {
    spec: &'a MachineSpec,
    lights: Vec<bool>,
    moves: usize,
}

impl<'a> MachinePart1<'a> {
    pub fn new(spec: &'a MachineSpec) -> Self {
        Self {
            spec, lights: vec![false; spec.lights.len()], moves: 0
        }
    }

    pub fn search(self) -> Option<Self> {
        let mut visited: HashSet<Self> = HashSet::new();
        let mut queue: VecDeque<Self> = VecDeque::new();
        queue.push_back(self);

        while let Some(v) = queue.pop_front() {
            for child in v.children() {
                if child.solved() { return Some(child); }
                if visited.contains(&child) { continue; }
                visited.insert(child.clone());
                queue.push_back(child);
            }
        }
        None
    }

    pub fn children<'b>(&'b self) -> impl IntoIterator<Item = Self> + 'b {
        self.spec.buttons.iter().map(move |button| {
            let mut new_lights = self.lights.clone();
            for &idx in button { new_lights[idx] ^= true; }
            Self { spec: self.spec, lights: new_lights, moves: self.moves + 1 }
        })
    }

    pub fn solved(&self) -> bool { return self.lights.iter().zip(self.spec.lights.iter()).all(|(&l, &r)| l == r) }
}

impl<'a, 'b> PartialEq<MachinePart1<'b>> for MachinePart1<'a> {
    fn eq(&self, other: &MachinePart1<'b>) -> bool {
        std::ptr::eq(self.spec as *const MachineSpec, other.spec as *const MachineSpec)
            && self.lights == other.lights
    }
}
impl<'a> Eq for MachinePart1<'a> { }

impl<'a> Hash for MachinePart1<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.spec as *const MachineSpec).hash(state);
        self.lights.hash(state);
    }
}

pub fn run(test: bool) -> Result<(), String> {
    let verbosity = verbosity();

    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day10_test.txt" } else { "day10.txt" });

    let file = File::open(input_path).expect("Failed to open input file");

    let mut specs: Vec<MachineSpec> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?;
        let mut machine = MachineSpec::from_str(&line)?;
        machine.sort_buttons();
        specs.push(machine);
    }

    let mut part1_total = 0u64;
    for spec in specs.iter() {
        let sol = MachinePart1::new(spec).search().ok_or_else(|| format!("No solution for machine: {spec}"))?;
        part1_total += sol.moves as u64;
        if verbosity > 0 { println!("{}: {} moves", spec, sol.moves) };
    }
    println!("Part 1 total: {part1_total}\n");

    let mut part2_total = 0u64;
    for spec in specs.iter() {
        let mut problem = ColProblem::new();

        // this is sus
        let voltages = spec.voltages.iter().map(|&voltage| problem.add_row(voltage as f64..=voltage as f64)).collect_vec();
        for button in &spec.buttons {
            problem.add_integer_column(1.0, 0.., button.iter().map(|&i| (voltages[i], 1.0)))
        }
        let solution = problem.optimise(Sense::Minimise).solve(); // .map_err(|e| format!("couldn't solve: {e}"))?;
        if solution.status() != HighsModelStatus::Optimal {
            return Err(format!("Solver gave solution status: {:?}", solution.status()));
        }

        let buttons = solution.get_solution().columns().iter().map(|&v| v.round() as u64).collect_vec();
        let sum = buttons.iter().sum::<u64>();
        part2_total += sum;

        if verbosity > 0 {
            println!("{spec} solution: {buttons:?} ({sum} moves)");
        }
    }
    println!("Part 2 total: {part2_total}");
    Ok(())
}