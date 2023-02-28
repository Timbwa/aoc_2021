use std::path::PathBuf;

use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn solution_1(&self, lines: &Vec<String>) -> usize {
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

    fn solution_2(&self, lines: &Vec<String>) -> usize {
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

    fn file_path(&self, is_test: bool) -> PathBuf {
        let mut file_path = PathBuf::new();
        file_path.push("input");
        let mut file_name = String::from("day_01.txt");

        if is_test {
            file_name = file_name.replace(".txt", "_test.txt");
        }

        file_path.push(file_name);

        file_path
    }
}
