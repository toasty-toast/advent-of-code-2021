mod utils;

struct Point {
    x: u32,
    y: u32,
}

struct Line {
    start: Point,
    end: Point,
}

fn main() {
    let lines = load_lines(utils::read_puzzle_input());
    // for row in map.iter() {
    //     for value in row.iter() {
    //         if *value == 0 {
    //             print!(".");
    //         } else {
    //             print!("{}", *value);
    //         }
    //     }
    //     println!();
    // }
    println!("Part 1: {}", count_overlaps(build_map(&lines, false)));
    println!("Part 2: {}", count_overlaps(build_map(&lines, true)));

    // let map = build_map(&lines, true);
    // for row in map.iter() {
    //     for value in row.iter() {
    //         if *value == 0 {
    //             print!(".");
    //         } else {
    //             print!("{}", *value);
    //         }
    //     }
    //     println!();
    // }
}

fn build_map(lines: &Vec<Line>, include_diagonals: bool) -> Vec<Vec<u32>> {
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines.iter() {
        if line.start.x > max_x {
            max_x = line.start.x;
        }
        if line.end.x > max_x {
            max_x = line.end.x;
        }
        if line.end.y > max_y {
            max_y = line.end.y;
        }
        if line.start.y > max_y {
            max_y = line.start.y;
        }
    }
    let mut map = (0..max_y + 1)
        .map(|_| vec![0; (max_x + 1) as usize])
        .collect::<Vec<Vec<u32>>>();

    for line in lines.iter() {
        if line.start.x == line.end.x {
            for i in get_values_in_range(line.start.y, line.end.y) {
                map[i as usize][line.start.x as usize] += 1;
            }
        } else if line.start.y == line.end.y {
            for i in get_values_in_range(line.start.x, line.end.x) {
                map[line.start.y as usize][i as usize] += 1;
            }
        } else if include_diagonals {
            let x_values = get_values_in_range(line.start.x, line.end.x);
            let y_values = get_values_in_range(line.start.y, line.end.y);
            for i in 0..x_values.len() {
                map[y_values[i] as usize][x_values[i] as usize] += 1;
            }
        }
    }

    return map;
}

fn count_overlaps(map: Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for row in map.iter() {
        for value in row.iter() {
            if *value >= 2 {
                count += 1;
            }
        }
    }
    return count;
}

fn get_values_in_range(first: u32, second: u32) -> Vec<u32> {
    if first < second {
        return (first..(second + 1)).collect();
    } else {
        return (second..(first + 1)).rev().collect();
    }
}

fn load_lines(input: Vec<String>) -> Vec<Line> {
    let mut lines = Vec::<Line>::new();
    for line in input {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let start_parts: Vec<u32> = parts[0]
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let end_parts: Vec<u32> = parts[1]
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        lines.push(Line {
            start: Point {
                x: start_parts[0],
                y: start_parts[1],
            },
            end: Point {
                x: end_parts[0],
                y: end_parts[1],
            },
        });
    }
    return lines;
}
