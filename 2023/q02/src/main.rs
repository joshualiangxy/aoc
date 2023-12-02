use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use std::{env::current_dir, fs::read_to_string, path::PathBuf};

#[derive(Debug, Parser)]
#[clap(name = "q02", version)]
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
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut total = 0;
    let input = read_to_string(full_path)?;

    for line in input.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (game_id_str, subsets_str) = line.split_once(": ").unwrap();
        let (_, game_id) = game_id_str.split_once(' ').unwrap();
        let game_id = game_id.parse::<u32>().unwrap();

        let subsets = subsets_str.split("; ");
        let mut is_possible = true;

        for subset in subsets {
            for cube in subset.split(", ") {
                let (num, colour) = cube.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();

                let limit = match colour {
                    "red" => red_limit,
                    "green" => green_limit,
                    "blue" => blue_limit,
                    _ => unreachable!("Should never happen"),
                };

                if num > limit {
                    is_possible = false;
                    break;
                }
            }

            if !is_possible {
                break;
            }
        }

        if is_possible {
            total += game_id;
        }
    }

    println!("total: {total}");
    Ok(())
}

fn part2(full_path: PathBuf) -> std::io::Result<()> {
    let mut total = 0;
    let input = read_to_string(full_path)?;

    for line in input.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (_, subsets_str) = line.split_once(": ").unwrap();

        let subsets = subsets_str.split("; ");

        let mut min_red = None;
        let mut min_blue = None;
        let mut min_green = None;

        for subset in subsets {
            for cube in subset.split(", ") {
                let (num, colour) = cube.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();

                match colour {
                    "red" => {
                        if let Some(min_value) = min_red {
                            if min_value < num {
                                min_red = Some(num);
                            }
                        } else {
                            min_red = Some(num);
                        }
                    }
                    "green" => {
                        if let Some(min_value) = min_green {
                            if min_value < num {
                                min_green = Some(num);
                            }
                        } else {
                            min_green = Some(num);
                        }
                    }
                    "blue" => {
                        if let Some(min_value) = min_blue {
                            if min_value < num {
                                min_blue = Some(num);
                            }
                        } else {
                            min_blue = Some(num);
                        }
                    }
                    _ => unreachable!("Should never happen"),
                };
            }
        }

        let power = min_red.unwrap_or(0) * min_blue.unwrap_or(0) * min_green.unwrap_or(0);
        total += power;
    }

    println!("total: {total}");
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
