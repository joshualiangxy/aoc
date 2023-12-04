use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use std::{collections::HashSet, env::current_dir, fs::read_to_string, path::PathBuf};

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
    let input = binding.trim();
    let mut total = 0;

    for line in input.split('\n') {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning_nums_str, nums_str) = line.split_once(" | ").unwrap();
        let mut maybe_points = None;

        let winning_nums = winning_nums_str
            .trim()
            .split(' ')
            .filter_map(|num_str| num_str.parse::<u32>().ok())
            .collect::<HashSet<u32>>();

        nums_str
            .trim()
            .split(' ')
            .filter_map(|num_str| num_str.parse::<u32>().ok())
            .for_each(|num| {
                if !winning_nums.contains(&num) {
                    return;
                }
                if let Some(points) = maybe_points {
                    maybe_points = Some(points * 2);
                } else {
                    maybe_points = Some(1);
                }
            });

        total += maybe_points.unwrap_or(0);
    }

    println!("{total}");
    Ok(())
}

fn part2(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let input = binding.trim();
    let mut total = 0;
    let mut card_multipliers = Vec::new();

    for (card_num, line) in input.split('\n').enumerate() {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning_nums_str, nums_str) = line.split_once(" | ").unwrap();
        let mut num_matches = 0;
        if card_num >= card_multipliers.len() {
            card_multipliers.push(1);
        } else {
            card_multipliers[card_num] += 1;
        }

        let winning_nums = winning_nums_str
            .trim()
            .split(' ')
            .filter_map(|num_str| num_str.parse::<u32>().ok())
            .collect::<HashSet<u32>>();

        nums_str
            .trim()
            .split(' ')
            .filter_map(|num_str| num_str.parse::<u32>().ok())
            .for_each(|num| {
                if winning_nums.contains(&num) {
                    num_matches += 1;
                }
            });

        for i in card_num + 1..card_num + 1 + num_matches {
            if i >= card_multipliers.len() {
                card_multipliers.push(card_multipliers[card_num]);
            } else {
                card_multipliers[i] += card_multipliers[card_num];
            }
        }

        total += card_multipliers[card_num];
    }

    println!("{total}");
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
