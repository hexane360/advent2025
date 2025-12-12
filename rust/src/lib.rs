#![feature(int_roundings)]
#![feature(ascii_char)]

use std::{sync::OnceLock, path::{Path, PathBuf}, sync::atomic::{AtomicU8, Ordering}};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;

static VERBOSITY: AtomicU8 = AtomicU8::new(0);
static INPUT_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn set_verbosity(value: u8) {
    VERBOSITY.store(value, Ordering::Release);
}

pub fn verbosity() -> u8 {
    VERBOSITY.load(Ordering::Relaxed)
}

pub fn input_dir() -> &'static Path {
    INPUT_DIR.get_or_init(|| {
        let buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.join("../input").canonicalize().expect("Failed to get absolute path")
    })
}