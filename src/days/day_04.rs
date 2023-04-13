use super::Day;

pub struct Day04;

impl Day04 {
    fn split_random_numbers_and_boards<'a>(
        &'a self,
        lines: &'a Vec<String>,
    ) -> Option<(Vec<u32>, &'a [String])> {
        if let Some((random_numbers, boards)) = lines.split_first() {
            let random_numbers = random_numbers
                .split(",")
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            Some((random_numbers, boards))
        } else {
            None
        }
    }

    fn get_boards(&self, board_strings: &[String]) -> Vec<Vec<u32>> {
        let mut board_vecs: Vec<Vec<u32>> = Vec::new();
        let mut index: i16 = -1;
        for line in board_strings {
            match line.is_empty() {
                true => {
                    let board_vec: Vec<u32> = Vec::new();
                    board_vecs.push(board_vec);
                    index += 1;
                }
                false => {
                    let board_row = line
                        .split_whitespace()
                        .map(|number| number.parse::<u32>().expect("Invalid Number"))
                        .collect::<Vec<u32>>();
                    if let Some(board_vec) =
                        board_vecs.get_mut(usize::try_from(index).expect("Invalid Index"))
                    {
                        board_vec.extend(board_row);
                    }
                }
            }
        }

        board_vecs
    }

    fn check_for_row_bingo(&self, board: &Vec<u32>, bingo_numbers: &[u32]) -> bool {
        let iter = board.chunks(5);
        let mut is_bingo = true;
        for row in iter {
            // println!("row chunk= {row:?}");
            // println!("bingo = {bingo_numbers:?}");
            for value in row {
                if bingo_numbers.len() == 12 {
                    // println!("{}", !bingo_numbers.contains(&value));
                }
                if !bingo_numbers.contains(&value) {
                    is_bingo = false;
                    break;
                }
            }
        }
        if bingo_numbers.len() == 12 {
            // println!("ROW = {row:?} |=>| {bingo_numbers:?}");
        }
        // if is_bingo {
        //     // println!("ROW | {bingo_numbers:?}");
        // }
        is_bingo
    }

    fn check_for_col_bingo(&self, board: &Vec<u32>, bingo_numbers: &[u32]) -> bool {
        let start_col_indices = [0, 1, 2, 3, 4];
        let mut is_bingo = true;

        for index in start_col_indices {
            let col = vec![
                board[index + 0],
                board[index + 5],
                board[index + 10],
                board[index + 15],
                board[index + 20],
            ];

            if bingo_numbers.len() == 12 {
                // println!("COL = {col:?}");
            }

            for value in col {
                if !bingo_numbers.contains(&value) {
                    is_bingo = false;
                    break;
                }
            }
        }
        is_bingo
    }
}

impl Day for Day04 {
    fn file_name(&self) -> String {
        String::from("day_04.txt")
    }

    fn solution_1(&self, lines: &Vec<String>) -> usize {
        if let Some((random_numbers, boards)) = self.split_random_numbers_and_boards(lines) {
            for split_index in 1..(random_numbers.len() + 1) {
                let boards = self.get_boards(boards);
                let (current_bingo_numbers, _) = random_numbers.split_at(split_index);
                println!("Current Bingo Numbers: {current_bingo_numbers:?}");
                for (index, board) in boards.iter().enumerate() {
                    // println!(
                    //     "r = {}",
                    //     self.check_for_row_bingo(&board, current_bingo_numbers)
                    // );
                    // println!(
                    //     "c = {}",
                    //     self.check_for_col_bingo(&board, current_bingo_numbers)
                    // );
                    // dbg!(&current_bingo_numbers);

                    let is_row_bingo = self.check_for_row_bingo(&board, current_bingo_numbers);
                    let is_col_bingo = self.check_for_col_bingo(&board, current_bingo_numbers);

                    if is_row_bingo {
                        println!("ROW BINGO!!: {current_bingo_numbers:?}");
                        println!("Board {index:?}");
                    }
                    if is_col_bingo {
                        println!("COL BINGO!!: {current_bingo_numbers:?}");
                        println!("Board {index:?}");
                    }

                    if is_row_bingo || is_col_bingo {
                        let mut unmarked = board.clone();
                        unmarked.retain(|val| current_bingo_numbers.contains(&val));

                        // println!("{current_bingo_numbers:?}");
                        // println!("{unmarked:?}");
                        return (unmarked.iter().sum::<u32>()
                            * current_bingo_numbers.last().unwrap())
                        .try_into()
                        .unwrap();
                    }
                }
            }
        }
        0
    }

    fn solution_2(&self, lines: &Vec<String>) -> usize {
        0
    }
}
