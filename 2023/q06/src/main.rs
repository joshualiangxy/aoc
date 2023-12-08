use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use std::{env::current_dir, fs::read_to_string, path::PathBuf};

#[derive(Debug, Parser)]
#[clap(name = "q04", version)]
struct App {
    #[clap(subcommand)]
    parts: Parts,
}

#[derive(Debug, Subcommand)]
enum Parts {
    /// Problem for part 1
    P1 {
        /// The path to read from
        path: Utf8PathBuf,
    },
    /// Problem for part 2
    P2 {
        /// The path to read from
        path: Utf8PathBuf,
    },
}

fn part1(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let mut lines = binding.trim().lines();

    let total_race_times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|time_str| time_str.parse::<u32>().ok())
        .collect();
    let distances_to_beat: Vec<u32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|distance_str| distance_str.parse::<u32>().ok())
        .collect();
    let result = total_race_times
        .into_iter()
        .zip(distances_to_beat.into_iter())
        .map(|(total_race_time, distance_to_beat)| {
            (0..=total_race_time).into_iter().fold(0, |acc, x| {
                let distance = x * (total_race_time - x);
                if distance > distance_to_beat {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .fold(1, |acc, x| acc * x);

    println!("{result}");
    Ok(())
}

fn part2(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let mut lines = binding.trim().lines();

    let total_race_time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|time_str| time_str.parse::<u64>().ok())
        .fold(0, |acc, x| {
            let num_digits = u64::ilog10(x) + 1;
            acc * 10u64.pow(num_digits) + x
        });
    let distance_to_beat = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|distance_str| distance_str.parse::<u64>().ok())
        .fold(0, |acc, x| {
            let num_digits = u64::ilog10(x) + 1;
            acc * 10u64.pow(num_digits) + x
        });

    let num_ways = (0..=total_race_time).into_iter().fold(0, |acc, x| {
        let distance = x * (total_race_time - x);
        if distance > distance_to_beat {
            acc + 1
        } else {
            acc
        }
    });

    println!("{num_ways}");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = App::parse();
    let current_dir = current_dir()?;

    match args.parts {
        Parts::P1 { path } => {
            let full_path = current_dir.join(path);
            part1(full_path)?;
        }
        Parts::P2 { path } => {
            let full_path = current_dir.join(path);
            part2(full_path)?;
        }
    }

    Ok(())
}
