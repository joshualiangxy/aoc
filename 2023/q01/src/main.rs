use camino::Utf8PathBuf;
use std::fs::read_to_string;
use std::{env::current_dir, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "q01", version)]
struct App {
    #[clap(subcommand)]
    subcommands: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
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
    let input = read_to_string(full_path)?;
    let mut total: u32 = 0;
    for line in input.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let find_digit = |c: char| c.is_digit(10);
        let first_pos = line.find(find_digit);
        let last_pos = line.rfind(find_digit);

        match (first_pos, last_pos) {
            (Some(first_pos), Some(last_pos)) => {
                let mut chars = line.chars();
                let first_digit = chars.nth(first_pos).unwrap().to_digit(10).unwrap();
                let last_digit = if first_pos == last_pos {
                    first_digit
                } else {
                    chars
                        .nth(last_pos - first_pos - 1)
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                };
                total += first_digit * 10 + last_digit;
            }
            _ => unreachable!("Should never happen"),
        }
    }

    println!("total: {total}");
    Ok(())
}

fn part2(full_path: PathBuf) -> std::io::Result<()> {
    let input = read_to_string(full_path)?;
    let mut total: u32 = 0;
    let digits = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let find_digit = |c: char| c.is_digit(10);
        let mut first_pos = line.find(find_digit);
        let mut last_pos = line.rfind(find_digit);

        let mut first_digit = None;
        let mut last_digit = None;

        for (index, digit) in digits.iter().enumerate() {
            if index == 0 {
                continue;
            }

            match line.find(digit) {
                Some(pos) => {
                    if pos < first_pos.unwrap() {
                        first_pos = Some(pos);
                        first_digit = Some(index as u32);
                    }
                }
                None => (),
            };
            match line.rfind(digit) {
                Some(pos) => {
                    if pos > last_pos.unwrap() {
                        last_pos = Some(pos);
                        last_digit = Some(index as u32);
                    }
                }
                None => (),
            };
        }

        match (first_pos, last_pos) {
            (Some(first_pos), Some(last_pos)) => {
                let mut chars = line.chars();

                match (first_digit, last_digit) {
                    (Some(_first_digit), Some(_last_digit)) => (),
                    (Some(_first_digit), None) => {
                        last_digit = chars.nth(last_pos).and_then(|c| c.to_digit(10));
                    }
                    (None, Some(_last_digit)) => {
                        first_digit = chars.nth(first_pos).and_then(|c| c.to_digit(10));
                    }
                    (None, None) => {
                        first_digit = chars.nth(first_pos).and_then(|c| c.to_digit(10));
                        last_digit = if first_pos == last_pos {
                            first_digit
                        } else {
                            chars
                                .nth(last_pos - first_pos - 1)
                                .and_then(|c| c.to_digit(10))
                        };
                    }
                }

                total += first_digit.unwrap() * 10 + last_digit.unwrap();
            }
            _ => unreachable!("Should never happen"),
        }
    }

    println!("total: {total}");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = App::parse();
    let current_dir = current_dir()?;
    match args.subcommands {
        SubCommands::P1 { path } => {
            let full_path = current_dir.join(path);
            part1(full_path)?;
        }
        SubCommands::P2 { path } => {
            let full_path = current_dir.join(path);
            part2(full_path)?;
        }
    }
    Ok(())
}
