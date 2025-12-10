use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Problem to run
    #[command(subcommand)]
    problem: Problem,
}

#[derive(Subcommand)]
enum Problem {
    Day1 {
        #[arg(long)]
        test: bool,
    },
    Day2 { },
    Day3 {
        #[arg(long)]
        test: bool,
    },
    Day4 {
        #[arg(long)]
        test: bool,
    },
    Day5 {
        #[arg(long)]
        test: bool,
    },
    Day6 {
        #[arg(long)]
        test: bool,
    },
    Day7 {
        #[arg(long)]
        test: bool,
    },
    Day8 {
        #[arg(long)]
        test: bool,
    },
    Day9 {
        #[arg(long)]
        test: bool,
    },
}

impl Problem {
    fn run(&self) -> Result<(), String> {
        match self {
            Self::Day1 { test } => { advent::day1::run(*test) },
            Self::Day2 { } => { advent::day2::run() },
            Self::Day3 { test } => { advent::day3::run(*test) },
            Self::Day4 { test } => { advent::day4::run(*test) },
            Self::Day5 { test } => { advent::day5::run(*test) },
            Self::Day6 { test } => { advent::day6::run(*test) },
            Self::Day7 { test } => { advent::day7::run(*test) },
            Self::Day8 { test } => { advent::day8::run(*test) },
            Self::Day9 { test } => { advent::day9::run(*test) },
        }
    }
}

fn main() -> Result<(), String> {
    let cli = Args::parse();
    advent::set_verbosity(cli.verbose);
    cli.problem.run()
}