mod utils;

fn main() {
    let input_lines = utils::read_puzzle_input();
    let depths: Vec<i32> = input_lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let mut previous_depth = -1;
    let mut number_of_increases = 0;
    for i in 0..depths.len() {
        if i > 0 && depths[i] > previous_depth {
            number_of_increases += 1;
        }
        previous_depth = depths[i];
    }
    println!("Part 1: {}", number_of_increases);

    previous_depth = -1;
    number_of_increases = 0;
    for i in 2..depths.len() {
        let depth = depths[i - 2] + depths[i - 1] + depths[i];
        if previous_depth != -1 && depth > previous_depth {
            number_of_increases += 1;
        }
        previous_depth = depth;
    }
    println!("Part 2: {}", number_of_increases);
}
