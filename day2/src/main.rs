fn main() {
    let input = AoC_helpers::open_file_into_string("./input.txt");
    let input = AoC_helpers::split_by_newline(input);

    let input = input.iter()
                          .map(|element| split_strings_into_command_dist_pairs(element))
                          .collect::<Vec<(Command, u32)>>();

    let part1_result = process_part_one(&input);
    let part2_result = process_part_two(&input);

    println!("Part 1 Result = {}", part1_result);
    println!("Part 2 Result = {}", part2_result)
}

fn process_part_one(input: &Vec<(Command, u32)>) -> i64 {
    let mut depth: i64 = 0;
    let mut horiz: i64 = 0;

    for step in input {
        match step.0 {
            Command::Forward => horiz += step.1 as i64,
            Command::Up => depth -= step.1 as i64,
            Command::Down => depth += step.1 as i64,
        }
    }

    depth * horiz
}

fn process_part_two(input: &Vec<(Command, u32)>) -> i64 {
    let mut depth: i64 = 0;
    let mut horiz: i64 = 0;
    let mut aim: i64 = 0;

    for step in input {
        match step.0 {
            Command::Forward => { horiz += step.1 as i64; depth += (step.1 as i64 * aim)},
            Command::Up => aim -= step.1 as i64,
            Command::Down => aim += step.1 as i64,
        }
    }

    depth * horiz
}

enum Command {
    Forward,
    Down,
    Up,
}

fn split_strings_into_command_dist_pairs(input: &str) -> (Command, u32) {
    let temp = input.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
    let command = match temp[0].as_str() {
        "forward" => Command::Forward,
        "down" => Command::Down,
        "up" => Command::Up,
        _ => panic!("Unexpected command"),
    };

    let dist = temp[1].parse::<u32>().unwrap();

    (command, dist)
}

#[test]
fn part_one_example() {
    let test_string = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    let test_vec = AoC_helpers::split_by_newline(test_string.to_string());

    let test_vec= test_vec.iter()
        .map(|element| split_strings_into_command_dist_pairs(element))
        .collect::<Vec<(Command, u32)>>();

    assert_eq!(150, process_part_one(&test_vec))
}

#[test]
fn part_two_example() {
    let test_string = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    let test_vec = AoC_helpers::split_by_newline(test_string.to_string());

    let test_vec= test_vec.iter()
                          .map(|element| split_strings_into_command_dist_pairs(element))
                          .collect::<Vec<(Command, u32)>>();

    assert_eq!(900, process_part_two(&test_vec));
}
