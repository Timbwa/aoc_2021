use super::Day;

pub struct Day03;

enum Rating {
    OxygenGenerator,
    CO2Scrubbber,
}

impl Day03 {
    fn convert_binary_to_decimal(&self, binary: &String) -> u32 {
        let value = u32::from_str_radix(binary.as_str(), 2).unwrap();
        value
    }

    fn determine_rating_value(
        &self,
        lines: &Vec<String>,
        bit_position: usize,
        rating: Rating,
    ) -> Vec<String> {
        if lines.len() == 1 {
            return lines.clone();
        }
        let mut position_0_count = 0;
        let mut position_1_count = 0;
        let criteria_vec: Vec<String> = lines.clone();

        for line in lines {
            let characters: Vec<char> = line.chars().collect();
            let head = characters[bit_position];

            match head {
                '0' => position_0_count += 1,
                '1' => position_1_count += 1,
                _ => {}
            }
        }

        match rating {
            Rating::OxygenGenerator => {
                if position_0_count > position_1_count {
                    criteria_vec
                        .iter()
                        .filter(|number| {
                            (number.chars().collect::<Vec<char>>())[bit_position] == '0'
                        })
                        .cloned()
                        .collect()
                } else {
                    criteria_vec
                        .iter()
                        .filter(|number| {
                            (number.chars().collect::<Vec<char>>())[bit_position] == '1'
                        })
                        .cloned()
                        .collect()
                }
            }
            Rating::CO2Scrubbber => {
                if position_0_count > position_1_count {
                    criteria_vec
                        .iter()
                        .filter(|number| {
                            (number.chars().collect::<Vec<char>>())[bit_position] == '1'
                        })
                        .cloned()
                        .collect()
                } else {
                    criteria_vec
                        .iter()
                        .filter(|number| {
                            (number.chars().collect::<Vec<char>>())[bit_position] == '0'
                        })
                        .cloned()
                        .collect()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Day03;

    #[test]
    fn converts_binary_to_decimal() {
        let expected = 5;
        let binary_str = String::from("0101");

        let day3 = Day03;
        let actual = day3.convert_binary_to_decimal(&binary_str);

        assert_eq!(actual, expected);
    }
}

impl Day for Day03 {
    fn solution_1(&self, lines: &Vec<String>) -> usize {
        let mut gamma = String::new();
        let mut epsilion = String::new();

        let line_length = lines[0].len();
        for i in 0..line_length {
            let mut gamma_0_count = 0;
            let mut gamma_1_count = 0;

            for line in lines {
                let characters: Vec<char> = line.chars().collect();
                let head = characters[i];

                match head {
                    '0' => gamma_0_count += 1,
                    '1' => gamma_1_count += 1,
                    _ => {}
                }
            }
            if gamma_0_count > gamma_1_count {
                gamma.push('0');
            } else {
                gamma.push('1');
            }
            if gamma_0_count < gamma_1_count {
                epsilion.push('0');
            } else {
                epsilion.push('1');
            }
        }
        let gamma = self.convert_binary_to_decimal(&gamma);
        let epsilion = self.convert_binary_to_decimal(&epsilion);

        (gamma * epsilion) as usize
    }

    fn solution_2(&self, lines: &Vec<String>) -> usize {
        let mut oxygen_criteria_vec: Vec<String> = lines.clone();
        let mut co2_subsriber_vec: Vec<String> = lines.clone();
        let mut oxygen_criteria_rating = 0;
        let mut co2_subscriber_rating = 0;

        let line_length = lines[0].len();
        for i in 0..(line_length + 1) {
            oxygen_criteria_vec =
                self.determine_rating_value(&oxygen_criteria_vec, i, Rating::OxygenGenerator);
            co2_subsriber_vec =
                self.determine_rating_value(&co2_subsriber_vec, i, Rating::CO2Scrubbber);

            if oxygen_criteria_vec.len() == 1 {
                oxygen_criteria_rating =
                    self.convert_binary_to_decimal(oxygen_criteria_vec.first().unwrap());
            }
            if co2_subsriber_vec.len() == 1 {
                co2_subscriber_rating =
                    self.convert_binary_to_decimal(co2_subsriber_vec.first().unwrap());
            }
        }

        (oxygen_criteria_rating * co2_subscriber_rating) as usize
    }

    fn file_name(&self) -> String {
        String::from("day_03.txt")
    }
}
