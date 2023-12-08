use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use std::{
    cmp::{max, min, Ordering},
    env::current_dir,
    fs::read_to_string,
    path::PathBuf,
    str::Lines,
};

#[derive(Debug, Parser)]
#[clap(name = "q05", version)]
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

#[derive(Debug)]
struct Transformer {
    destination: u64,
    source: u64,
    range: u64,
}

impl Transformer {
    fn new(destination: u64, source: u64, range: u64) -> Transformer {
        Transformer {
            destination,
            source,
            range,
        }
    }

    fn transform(&self, original: u64) -> Option<u64> {
        if original < self.source || original >= self.source + self.range {
            return None;
        }

        Some(self.destination + original - self.source)
    }
}

#[derive(Copy, Clone, Debug)]
struct Range(u64, u64);

impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range(start, end)
    }

    fn overlap(&self, range: &Range) -> Option<Range> {
        let lower = max(self.0, range.0);
        let higher = min(self.1, range.1);

        if lower >= higher {
            return None;
        }

        Some(Range(lower, higher))
    }

    fn difference(&self, range: &Range) -> Vec<Range> {
        let left = if self.0 >= range.0 {
            None
        } else {
            let left_bound = self.0;
            let right_bound = min(self.1, range.0);
            Some(Range(left_bound, right_bound))
        };
        let right = if self.1 <= range.1 {
            None
        } else {
            let left_bound = max(self.0, range.1);
            let right_bound = self.1;
            Some(Range(left_bound, right_bound))
        };

        vec![left, right]
            .into_iter()
            .filter_map(std::convert::identity)
            .collect()
    }
}

#[derive(Debug)]
struct RangeTransformer {
    source: Range,
    destination: u64,
}

impl RangeTransformer {
    fn new(destination: u64, source: u64, range: u64) -> RangeTransformer {
        let source = Range::new(source, source + range);
        RangeTransformer {
            source,
            destination,
        }
    }

    fn transform(&self, original: Range) -> (Option<Range>, Vec<Range>) {
        let transformed_range = self.source.overlap(&original).map(|overlap| {
            let offset = overlap.0 - self.source.0;
            let range = overlap.1 - overlap.0;
            let start = self.destination + offset;
            let end = start + range;

            Range::new(start, end)
        });
        let leftover_ranges: Vec<Range> = original.difference(&self.source);

        (transformed_range, leftover_ranges)
    }
}

fn create_mapping_p1(lines: &mut Lines) -> Vec<Transformer> {
    let mut mappings = Vec::new();

    lines.next();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mapping_nums: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<u64>().unwrap())
            .collect();

        mappings.push(Transformer::new(
            mapping_nums[0],
            mapping_nums[1],
            mapping_nums[2],
        ));
    }

    mappings
}

fn part1(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let input = binding.trim();
    let mut lines = input.lines();

    let mut seeds: Vec<u64> = lines
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|seed_num| seed_num.parse::<u64>().ok())
        .collect();

    lines.next();

    for _ in 0..7 {
        let mappers = create_mapping_p1(&mut lines);
        seeds = seeds
            .iter()
            .map(|&seed| -> u64 {
                mappers
                    .iter()
                    .find_map(|mapper| mapper.transform(seed))
                    .unwrap_or(seed)
            })
            .collect();
    }

    let result = seeds
        .iter()
        .fold(u64::max_value(), |acc, &x| if x < acc { x } else { acc });

    println!("{result}");
    Ok(())
}

fn create_mapping_p2(lines: &mut Lines) -> Vec<RangeTransformer> {
    let mut mappings = Vec::new();

    lines.next();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mapping_nums: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<u64>().unwrap())
            .collect();

        mappings.push(RangeTransformer::new(
            mapping_nums[0],
            mapping_nums[1],
            mapping_nums[2],
        ));
    }

    mappings
}

fn part2(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let input = binding.trim();
    let mut lines = input.lines();

    let seed_nums: Vec<u64> = lines
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|seed_num| seed_num.parse::<u64>().ok())
        .collect();
    let mut seed_ranges: Vec<Range> = Vec::new();

    lines.next();

    for i in (0..seed_nums.len()).step_by(2) {
        let start = seed_nums[i];
        let end = seed_nums[i] + seed_nums[i + 1];
        seed_ranges.push(Range::new(start, end));
    }

    for _ in 0..7 {
        let mappers = create_mapping_p2(&mut lines);
        seed_ranges = seed_ranges
            .iter()
            .flat_map(|&seed_range| {
                let mut transformed = Vec::new();
                let mut ranges_to_process = vec![seed_range];
                for mapper in mappers.iter() {
                    if ranges_to_process.is_empty() {
                        break;
                    }

                    let mut new_range = Vec::new();
                    for range in ranges_to_process {
                        let (transformed_range, leftover_ranges) = mapper.transform(range);
                        leftover_ranges
                            .into_iter()
                            .for_each(|range| new_range.push(range));

                        if let Some(range) = transformed_range {
                            transformed.push(range);
                        }
                    }

                    ranges_to_process = new_range;
                }

                ranges_to_process.append(&mut transformed);
                ranges_to_process
            })
            .collect();

        seed_ranges.sort_by(|a, b| {
            if a.0 == b.0 {
                Ordering::Equal
            } else if a.0 < b.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    let result =
        seed_ranges.iter().fold(
            u64::max_value(),
            |acc, range| if range.0 < acc { range.0 } else { acc },
        );

    println!("{result}");

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
