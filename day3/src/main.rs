fn main() {
    let input = AoC_helpers::open_file_into_string("./input.txt");

    let string_len = (&input).split('\n').map(|s| s.to_string()).collect::<Vec<String>>()[0].len();
    let input2 = AoC_helpers::string_into_vec_of_uint_radix(input, 2);

    let part1_result = process_part1(&input2, string_len as u16);
    let part2_result = process_part2(&input2, string_len as u16);

    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);

}

fn process_part1(input: &Vec<u64>, width: u16) -> u64 {
    let mut counts = [0; 16];
    let _a = input.iter().map(|e| {
        for i in 0..16 {
            if ((e >> i) & 0x01) == 1 { counts[i] += 1}
        }
    }).count();

    let mut gamma:u16 = 0;

    for i in 0..width+1 {
        if counts[i as usize] > (input.len()/2) { gamma += (1 << i)}
    }

    let epsilon = !gamma & ((1 << width) - 1);

    (gamma as u64 * epsilon as u64)
}

fn process_part2(input: &Vec<u64>, width: u16) -> u64 {
    let mut oxygen_values = input.clone();
    let mut co2_values = input.clone();

    for j in 0..width {
        let i = width - 1 - j;

        // Find most common bit at i
        let mut count_1 = 0;
        let mut count_0 = 0;

        for v in &oxygen_values {
            if ((v >> i) & 0x01) == 1 {
                count_1 += 1;
            } else {
                count_0 += 1
            }
        }
        let value_to_keep = if count_1 >= count_0 { 1 } else { 0 };

        // Remove values that don't have that bit at i
        oxygen_values = oxygen_values.into_iter().filter(|&v| ((v >> i) & 0x01) == value_to_keep).collect();

        // Check if values.len() == 1
        if oxygen_values.len() == 1 {
            break;
        }
    }

    for j in 0..width {
        let i = width - 1 - j;

        // Find most common bit at i
        let mut count_1 = 0;
        let mut count_0 = 0;

        for v in &co2_values {
            if ((v >> i) & 0x01) == 1 {
                count_1 += 1;
            } else {
                count_0 += 1
            }
        }
        let value_to_keep = if count_1 >= count_0 { 0 } else { 1 };

        // Remove values that don't have that bit at i
        co2_values = co2_values.into_iter().filter(|&v| ((v >> i) & 0x01) == value_to_keep).collect();

        // Check if values.len() == 1
        if co2_values.len() == 1 {
            break;
        }
    }

    oxygen_values[0] * co2_values[0]
}


#[test]
fn part1_example() {
    let test_vec = vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];

    assert_eq!(198, process_part1(&test_vec, 5));
}

#[test]
fn part2_example() {
    let test_vec = vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];

    assert_eq!(230, process_part2(&test_vec, 5));
}