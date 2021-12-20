use AoC_helpers::{split_by_newline, string_into_vec_of_uint};

fn main() {
    let input = AoC_helpers::open_file_into_string("./input.txt");

    let part1_result = process_part_one(&input);
    let part2_result = process_part_two(&input);

    println!("Part 1 Result: {}", part1_result);
    println!("Part 2 Result: {}", part2_result);
}

fn process_part_one(input: &String) -> u64 {
    let (numbers, mut boards) = separate_input(&input);

    for n in numbers {
        for idx in 0..boards.len() {
            boards[idx].try_mark(n);
            if boards[idx].check_for_bingo() {
                // Do some math
                let score = boards[idx].score_board();
                println!("Last Num = {}, Score = {}", n, score);
                println!("Prod: {}", n*score);
                return score * n;
            }
        }
    }

    return 0;
}

fn process_part_two(input: &String) -> u64 {
    let (numbers, mut boards) = separate_input(&input);
    let mut index_of_last_winner = 0;
    let mut final_number = 0;

    for n in numbers {
        for idx in 0..boards.len() {
            if boards[idx].bingo_had == true {
                continue;
            }
            boards[idx].try_mark(n);
            if boards[idx].check_for_bingo() {
                index_of_last_winner = idx;
                final_number = n;
            }
        }
    }

    let score = boards[index_of_last_winner].score_board();
    println!("Last Num = {}, Score = {}", final_number, score);
    println!("Prod: {}", final_number*score);
    return score * final_number;
}

impl GameBoard {
    fn try_mark(&mut self, value: u64) {
        for r in 0..self.b.len() {
            for c in 0..self.b[r].len() {
                if value == self.b[r][c].0 {
                    self.mark(r, c);
                }
            }
        }
    }

    fn mark(&mut self, row: usize, column: usize) {
        self.b[row][column].1 = true;
    }

    fn check_for_bingo(&mut self) -> bool {
        // Check rows for bingo
        'outer: for r in 0..self.b.len() {
            for c in 0..self.b[r].len() {
                if self.b[r][c].1 == false {
                    continue 'outer;
                }
            }
            self.bingo_had = true;
            return true;
        }

        // Check cols for bingo
        let col_len = self.b[0].len();
        let num_rows = self.b.len();
        'outer2: for c in 0..col_len {
            for r in 0..num_rows {
                if self.b[r][c].1 == false {
                    continue 'outer2;
                }
            }
            self.bingo_had = true;
            return true;
        }
        self.bingo_had = false;
        false
    }

    fn score_board(&self) -> u64 {
        let mut acc = 0;
        for r in 0..self.b.len() {
            for c in 0..self.b[r].len() {
                if self.b[r][c].1 == false {
                    acc += self.b[r][c].0;
                }
            }
        }
        acc
    }

    fn print(&self, row: usize, column: usize) {
        print!("Val: {}, Marked? {}", self.b[row][column].0, self.b[row][column].1)
    }

    fn println(&self, row: usize, column: usize) {
        println!("Val: {}, Marked? {}", self.b[row][column].0, self.b[row][column].1)
    }

}

struct GameBoard {
    b: Vec<Vec<(u64, bool)>>,
    bingo_had: bool,
}

fn separate_input(input: &String) -> (Vec<u64>, Vec<GameBoard>) {
    let chunked = input.split("\n\n")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect::<Vec<String>>();

    let numbers = string_into_vec_of_uint(chunked[0].clone().replace(',', "\n"));

    let gbs = chunked.iter()
        .skip(1)
        .map(|s| string_to_board(s))
        .collect::<Vec<GameBoard>>();

    return (numbers, gbs)
}

fn string_to_board(input: &String) -> GameBoard {
    let board = split_by_newline(input).iter().map(|row| {
        row.split(" ")
            .filter(|&element| element != "")
            .map(|s| (u64::from_str_radix(&s, 10).unwrap(), false))
            .collect::<Vec<(u64, bool)>>()
    }).collect::<Vec<Vec<_>>>();

    GameBoard{b: board, bingo_had: false}
}

#[test]
fn part_one_test() {
    let input = AoC_helpers::open_file_into_string("./test_case_1.txt");
    assert_eq!(4512, process_part_one(&input))
}

#[test]
fn part_two_test() {
    let input = AoC_helpers::open_file_into_string("./test_case_1.txt");
    assert_eq!(1924, process_part_two(&input))
}
