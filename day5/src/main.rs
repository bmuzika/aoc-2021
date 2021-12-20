use std::collections::HashMap;
use AoC_helpers::split_by_newline;

fn main() {
    let input = AoC_helpers::open_file_into_string("./input.txt");
    let part_1_result = process_part_one(&input);
    let part_2_result = process_part_two(&input);

    println!("Part 1 Result = {}", part_1_result);
    println!("Part 2 Result = {}", part_2_result);
}

impl Segment {
    fn print(&self) {
        println!("{}, {} -> {}, {}", self.x1, self.y1, self.x2, self.y2)
    }
}
struct Segment {
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
}

fn separate_input(input: &String) -> (Vec<Segment>) {
    let lines = split_by_newline(input);
    let lines = lines.iter()
        .map(|s| s.split(" -> ")
            .map(|s| s.to_string())
            .filter(|s| s != "")
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut line_segments = lines.iter()
        .map(|line| line.iter()
            .map(|pair| pair.split(",")
                .map(|s| s.to_string())
                .filter(|s| s != "")
                .map(|s| u64::from_str_radix(&s, 10).unwrap())
                .collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>())
        .map(|vp| Segment{x1: vp[0][0], y1: vp[0][1], x2: vp[1][0], y2: vp[1][1]})
        .collect::<Vec<Segment>>();

    line_segments
}

fn process_part_one(input: &String) -> u64 {
    let lines = separate_input(&input);

    let mut grid_acc = HashMap::new();

    for line in lines {
        if line.x1 == line.x2 {
            // Scan over y
            let mut y1 = 0;
            let mut y2 = 0;
            if line.y1 < line.y2 {
                y1 = line.y1;
                y2 = line.y2;
            } else {
                y1 = line.y2;
                y2 = line.y1;
            }
            for y in y1..=y2 {
                let x = line.x1;
                let v = grid_acc.get(&(x, y));
                match v {
                    Some(v) => grid_acc.insert((x, y), v+1),
                    _ => grid_acc.insert((x, y), 1),
                };
            }
        }

        if line.y1 == line.y2 {
            // Scan over x
            let mut x1 = 0;
            let mut x2 = 0;
            if line.x1 < line.x2 {
                x1 = line.x1;
                x2 = line.x2;
            } else {
                x1 = line.x2;
                x2 = line.x1;
            }
            for x in x1..=x2 {
                let y = line.y1;
                let v = grid_acc.get(&(x, y));
                match v {
                    Some(v) => grid_acc.insert((x, y), v+1),
                    _ => grid_acc.insert((x, y), 1),
                };
            }
        }
    }

    let mut acc = 0;
    for point in grid_acc {
        //println!("{}, {} => {}", point.0.0, point.0.1, point.1);
        if point.1 >= 2 {
            acc += 1;
        }
    }
    acc
}

fn process_part_two(input: &String) -> u64 {
    let lines = separate_input(&input);

    let mut grid_acc = HashMap::new();

    for line in lines {
        if line.x1 == line.x2 {
            // Scan over y
            let mut y1 = 0;
            let mut y2 = 0;
            if line.y1 < line.y2 {
                y1 = line.y1;
                y2 = line.y2;
            } else {
                y1 = line.y2;
                y2 = line.y1;
            }
            for y in y1..=y2 {
                let x = line.x1;
                let v = grid_acc.get(&(x, y));
                match v {
                    Some(v) => grid_acc.insert((x, y), v+1),
                    _ => grid_acc.insert((x, y), 1),
                };
            }
        }

        else if line.y1 == line.y2 {
            // Scan over x
            let mut x1 = 0;
            let mut x2 = 0;
            if line.x1 < line.x2 {
                x1 = line.x1;
                x2 = line.x2;
            } else {
                x1 = line.x2;
                x2 = line.x1;
            }
            for x in x1..=x2 {
                let y = line.y1;
                let v = grid_acc.get(&(x, y));
                match v {
                    Some(v) => grid_acc.insert((x, y), v+1),
                    _ => grid_acc.insert((x, y), 1),
                };
            }
        }

        else {
            // Step from (x1, y1) stepping by one value to (x2, y2)
            let mut x_inc = 0;
            let mut y_inc = 0;
            if line.x1 < line.x2 {
                x_inc = 1;
            } else {
                x_inc = -1;
            }

            if line.y1 < line.y2 {
                y_inc = 1;
            } else {
                y_inc = -1;
            }

            let mut x = line.x1;
            let mut y = line.y1;

            let mut points = Vec::new();
            loop {
                points.push((x, y));
                x = (x as i32 + x_inc) as u64;
                y = (y as i32 + y_inc) as u64;

                if (x == line.x2) && (y == line.y2) {
                    points.push((x, y));
                    break;
                }
            }

            for p in points {
                let v = grid_acc.get(&(p.0, p.1));
                match v {
                    Some(v) => grid_acc.insert((p.0, p.1), v+1),
                    _ => grid_acc.insert((p.0, p.1), 1),
                };
            }
        }
    }

    let mut acc = 0;
    for point in grid_acc {
        //println!("{}, {} => {}", point.0.0, point.0.1, point.1);
        if point.1 >= 2 {
            acc += 1;
        }
    }
    acc
}

#[test]
fn part_one_test() {
    let input = AoC_helpers::open_file_into_string("./test_case_1.txt");
    assert_eq!(5, process_part_one(&input))
}

#[test]
fn part_two_test() {
    let input = AoC_helpers::open_file_into_string("./test_case_1.txt");
    assert_eq!(12, process_part_two(&input))
}
