mod utils;

fn main() {
    let mut pattern_sets = Vec::<Vec<Vec<char>>>::new();
    let mut digit_sets = Vec::<Vec<Vec<char>>>::new();
    for line in utils::read_puzzle_input().iter() {
        let (patterns, digits) = parse_input_line(line);
        pattern_sets.push(patterns);
        digit_sets.push(digits);
    }

    let mut unique_patterns_count = 0;
    for digit_set in digit_sets.iter() {
        for digit in digit_set.iter() {
            if is_pattern_unique_digit(digit) {
                unique_patterns_count += 1;
            }
        }
    }
    println!("Part 1: {}", unique_patterns_count);
}

fn parse_input_line(line: &String) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let input_parts = line
        .split("|")
        .map(|p| p.trim().to_string())
        .collect::<Vec<String>>();
    let patterns = input_parts[0]
        .split(" ")
        .map(|part| {
            part.split(" ")
                .map(|p| p.chars().collect())
                .collect::<Vec<Vec<char>>>()
        })
        .flatten()
        .collect();
    let digits = input_parts[1]
        .split(" ")
        .map(|p| p.chars().collect())
        .collect::<Vec<Vec<char>>>();
    return (patterns, digits);
}

fn is_pattern_unique_digit(pattern: &Vec<char>) -> bool {
    return pattern.len() == 2 || pattern.len() == 3 || pattern.len() == 4 || pattern.len() == 7;
}
