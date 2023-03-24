use super::Day;

pub struct Day02;

impl Day for Day02 {
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

    fn file_name(&self) -> String {
        String::from("day_02.txt")
    }
}
