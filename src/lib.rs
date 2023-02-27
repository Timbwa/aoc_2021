use anyhow::{Context, Result};
use clap::Parser;
/// Rust Program for running AOC solutions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path of the input file to use
    #[arg(short, long)]
    input_file_path: std::path::PathBuf,
    /// Run the latest day solutions or run all solutions
    #[arg(short, long, default_value_t = true)]
    run_latest: bool,
    /// Run the test input
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

pub fn run(cli: &mut Cli) -> Result<()> {
    if cli.test {
        let file_name = cli
            .input_file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".txt", "_test.txt");
        cli.input_file_path.set_file_name(file_name);
    }

    let content = std::fs::read_to_string(&cli.input_file_path)
        .with_context(|| format!("could not read file `{:?}`", &cli.input_file_path))?;

    let mut content_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        content_lines.push(line);
    }

    let result_1 = solution_1(&content_lines);
    let result_2 = solution_2(&content_lines);

    println!("Result 1 is {result_1:?}\nResults 2 is {result_2:?}");

    Ok(())
}

fn solution_1(lines: &[&str]) -> usize {
    let mut increased_count = 0;

    let values: Vec<usize> = lines
        .iter()
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    let mut previous = values[0];

    (1..values.len()).for_each(|index| {
        if values[index] > previous {
            increased_count += 1;
        }
        previous = values[index];
    });

    increased_count
}

fn solution_2(lines: &[&str]) -> usize {
    let values: Vec<usize> = lines
        .iter()
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    let mut increased_count = 0;

    let windows = values.windows(3);
    let mut previous_sum = windows
        .clone()
        .take(1)
        .fold(0, |acc, val| acc + val.iter().sum::<usize>());

    windows.skip(1).for_each(|window| {
        let window_sum = window.iter().sum::<usize>();
        if window_sum > previous_sum {
            increased_count += 1;
        }
        previous_sum = window_sum;
    });

    increased_count
}
