use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use std::{collections::HashSet, env::current_dir, fs::read_to_string, path::PathBuf};

#[derive(Debug, Parser)]
#[clap(name = "q03", version)]
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

fn get_start_end_idx(
    start_row_idx: usize,
    end_row_idx: usize,
    start_col_idx: usize,
    end_col_idx: usize,
    height: usize,
    width: usize,
) -> (usize, usize, usize, usize) {
    let row_start = if start_row_idx == 0 {
        start_row_idx
    } else {
        start_row_idx - 1
    };
    let row_end = if end_row_idx == height - 1 {
        end_row_idx
    } else {
        end_row_idx + 1
    };
    let col_start = if start_col_idx == 0 {
        start_col_idx
    } else {
        start_col_idx - 1
    };
    let col_end = if end_col_idx == width {
        end_col_idx
    } else {
        end_col_idx + 1
    };

    (row_start, row_end, col_start, col_end)
}

fn is_part_number(
    graph: &Vec<Vec<char>>,
    start_col_idx: usize,
    end_col_idx: usize,
    row_idx: usize,
) -> bool {
    let (row_start, row_end, col_start, col_end) = get_start_end_idx(
        row_idx,
        row_idx,
        start_col_idx,
        end_col_idx,
        graph.len(),
        graph[row_idx].len(),
    );

    for i in row_start..=row_end {
        for j in col_start..=col_end {
            let c = graph[i][j];
            if !c.is_digit(10) && c != '.' {
                return true;
            }
        }
    }

    false
}

fn part1(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let input = binding.trim();

    let mut total = 0;
    let mut graph = vec![];

    for line in input.split('\n') {
        graph.push(line.chars().collect::<Vec<_>>());
    }

    for i in 0..graph.len() {
        let mut curr_num = None;
        let mut start_idx = None;
        let mut end_idx = None;

        for j in 0..graph[i].len() {
            if graph[i][j].is_digit(10) {
                let curr_digit = graph[i][j].to_digit(10).unwrap();
                end_idx = Some(j);

                if let Some(num) = curr_num {
                    curr_num = Some(num * 10 + curr_digit);
                } else {
                    curr_num = Some(curr_digit);
                    start_idx = Some(j);
                }

                if j < graph[i].len() - 1 {
                    continue;
                }
            }

            if let Some(num) = curr_num {
                if is_part_number(&graph, start_idx.unwrap(), end_idx.unwrap(), i) {
                    total += num;
                }

                curr_num = None;
                start_idx = None;
                end_idx = None;
            }
        }
    }

    println!("{total}");
    Ok(())
}

fn get_gear_ratio(graph: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> Option<u32> {
    let (start_row, end_row, start_col, end_col) = get_start_end_idx(
        row_idx,
        row_idx,
        col_idx,
        col_idx,
        graph.len(),
        graph[row_idx].len(),
    );
    let mut covered_indices: HashSet<(usize, usize)> = HashSet::new();
    let mut total_numbers = 0;
    let mut gear_ratio = 1;

    for i in start_row..=end_row {
        for j in start_col..=end_col {
            if graph[i][j].is_digit(10) {
                if covered_indices.contains(&(i, j)) {
                    continue;
                }
                if total_numbers >= 2 {
                    return None;
                }

                let mut curr_col_idx = j;
                while curr_col_idx > 0 && graph[i][curr_col_idx - 1].is_digit(10) {
                    curr_col_idx -= 1;
                }

                let mut curr_num = graph[i][curr_col_idx].to_digit(10).unwrap();
                covered_indices.insert((i, curr_col_idx));

                while curr_col_idx < graph[i].len() - 1 && graph[i][curr_col_idx + 1].is_digit(10) {
                    curr_col_idx += 1;
                    curr_num = curr_num * 10 + graph[i][curr_col_idx].to_digit(10).unwrap();
                    covered_indices.insert((i, curr_col_idx));
                }

                gear_ratio *= curr_num;
                total_numbers += 1;
            }
        }
    }

    if total_numbers == 2 {
        Some(gear_ratio)
    } else {
        None
    }
}

fn part2(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let input = binding.trim();

    let mut total = 0;
    let mut graph = vec![];

    for line in input.split('\n') {
        graph.push(line.chars().collect::<Vec<_>>());
    }

    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if graph[i][j] == '*' {
                match get_gear_ratio(&graph, i, j) {
                    Some(gear_ratio) => total += gear_ratio,
                    None => (),
                }
            }
        }
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
