fn main() {
    let input = AoC_helpers::open_file_into_string("./input.txt");
    let part_1_result = process_part_one(&input, 80);
    println!("Part 1 Result = {}", part_1_result);
    println!();

    let part_2_result = process_part_two(&input, 256);

    println!("Part 2 Result = {}", part_2_result);
}

fn handle_fish(current_age: u64) -> (u64, bool) {
    if current_age == 0 {
        return (6, true);
    }

    let new_age = current_age - 1;

    (new_age, false)
}

fn process_part_one(input: &String, num_iters: usize) -> usize {
    let mut fish = input.split(",")
        .map(|s| s.trim_end().to_string())
        .filter(|s| s != "")
        .filter(|s| s != "\n")
        .filter(|s| s != " ")
        .map(|s| u64::from_str_radix(&s, 10).unwrap())
        .collect::<Vec<u64>>();

    for i in 0..num_iters {
        //if i % 32 == 0 {
       //     println!();
        //}
        //print!(".");

        let len = fish.len();

        for j in 0..len {
            let (fish_age, spawn) = handle_fish(fish[j]);
            fish[j] = fish_age;
            if spawn {
                fish.push(8);
            }
        }
        //println!("New frac: {}", ((fish.len() - len) as f64 / fish.len() as f64))
    }

    fish.len()
}

fn process_part_two(input: &String, num_iter: usize) -> u64 {
    let mut fish = input.split(",")
        .map(|s| s.trim_end().to_string())
        .filter(|s| s != "")
        .filter(|s| s != "\n")
        .filter(|s| s != " ")
        .map(|s| u64::from_str_radix(&s, 10).unwrap())
        .collect::<Vec<u64>>();

    let mut delay_arr = [0 as u64; 9];
    for j in 0..9 {
        delay_arr[j] = fish.clone().iter().filter(|&v| *v == j as u64).fold(0, |acc, v| acc + 1);
    }

    for i in 0..num_iter {
        let delay_arr_prime = delay_arr.clone();
        for j in 0..=7 {
            delay_arr[j] = delay_arr_prime[j+1];
        }
        delay_arr[6] += delay_arr_prime[0];
        delay_arr[8] = delay_arr_prime[0];

    }
    delay_arr.to_vec().iter().sum()
}


#[test]
fn part_one_test() {
    let test_input = "3,4,3,1,2".to_string();
    assert_eq!(5934, process_part_one(&test_input, 80))
}

#[test]
fn part_two_test() {
    let test_input = "3,4,3,1,2".to_string();
    assert_eq!(5934, process_part_two(&test_input, 80));
    assert_eq!(26984457539, process_part_two(&test_input, 256));
}
