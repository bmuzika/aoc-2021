use std::fs::File;
use std::io::Read;

fn main() {
    let input = open_file_into_string("./input.txt");
    let input = aoc_string_into_vec_of_uint(input);

    let part1_result = process_part_one(&input);
    let part2_result = process_part_two(&input, 3);

    println!("Part 1 Result = {}", part1_result);
    println!("Part 2 Result = {}", part2_result)
}

fn open_file_into_string(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    file_contents
}

fn aoc_string_into_vec_of_uint(input: String) -> Vec<u64> {
    let result = input.split('\n')
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    result
}

fn process_part_one(input: &Vec<u64>) -> u16 {
    let mut acc = 0;

    for i in 1..input.len() {
        if input[i] > input[i-1] {
            acc += 1;
        }
    }

    acc
}

fn process_part_two(input: &Vec<u64>, window_size: usize) -> u16 {
    let mut acc = 0;

    for i in window_size..input.len() {
        if input[i] as i64 - input[i - 3] as i64 > 0 {
            acc += 1;
        }
    }

    acc
}
#[test]
fn part_one_example() {
    let test_vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, process_part_one(&test_vec))
}

#[test]
fn part_two_example() {
    let test_vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(5, process_part_two(&test_vec, 3));
}