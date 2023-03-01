use std::path::PathBuf;

use super::Day;

pub struct Day02;

impl Day for Day02 {
    fn file_path(&self, is_test: bool) -> PathBuf {
        let mut file_path = PathBuf::new();
        file_path.push("input");
        let mut file_name = String::from("day_02.txt");

        if is_test {
            file_name = file_name.replace(".txt", "_test.txt");
        }

        file_path.push(file_name);

        file_path
    }

    fn solution_1(&self, lines: &Vec<String>) -> usize {
        let mut depth = 0;
        let mut horizontal = 0;

        for line in lines.iter() {
            let slices: Vec<&str> = line.split_whitespace().collect();
            let instruction = slices[0];
            let amount = slices[1].parse::<usize>().unwrap();
            match instruction {
                "forward" => {
                    horizontal += amount;
                }
                "up" => {
                    depth -= amount;
                }
                "down" => {
                    depth += amount;
                }
                &_ => {}
            }
        }

        depth * horizontal
    }

    fn solution_2(&self, lines: &Vec<String>) -> usize {
        let mut aim = 0;
        let mut depth = 0;
        let mut horizontal = 0;

        for line in lines.iter() {
            let slices: Vec<&str> = line.split_whitespace().collect();
            let instruction = slices[0];
            let amount = slices[1].parse::<usize>().unwrap();
            match instruction {
                "forward" => {
                    horizontal += amount;
                    depth += aim * amount;
                }
                "up" => {
                    aim -= amount;
                }
                "down" => {
                    aim += amount;
                }
                &_ => {}
            }
        }

        depth * horizontal
    }
}
