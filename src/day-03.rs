mod utils;

fn main() {
    let values: Vec<String> = utils::read_puzzle_input();

    let gamma_rate = calculate_gamma_rate(&values);
    let epsilon_rate = calculate_epsilon_rate(&values);
    println!("Part 1: {}", gamma_rate * epsilon_rate);

    let oxygen_rating = calculate_oxygen_rating(&values);
    let co2_rating = calculate_co2_rating(&values);
    println!("Part 2: {}", oxygen_rating * co2_rating);
}

fn calculate_gamma_rate(bit_strings: &Vec<String>) -> u32 {
    let mut result_bit_string: String = "".to_string();
    for i in 0..bit_strings[0].len() {
        let mut num_zeroes = 0;
        let mut num_ones = 0;
        for bit_string in bit_strings.iter() {
            let ch = bit_string.chars().nth(i).unwrap();
            if ch == '0' {
                num_zeroes += 1;
            } else if ch == '1' {
                num_ones += 1;
            }
        }
        if num_ones > num_zeroes {
            result_bit_string.push_str("1");
        } else {
            result_bit_string.push_str("0");
        }
    }
    return u32::from_str_radix(result_bit_string.as_str(), 2).unwrap();
}

fn calculate_epsilon_rate(bit_strings: &Vec<String>) -> u32 {
    let mut result_bit_string: String = "".to_string();
    for i in 0..bit_strings[0].len() {
        let mut num_zeroes = 0;
        let mut num_ones = 0;
        for bit_string in bit_strings.iter() {
            let ch = bit_string.chars().nth(i).unwrap();
            if ch == '0' {
                num_zeroes += 1;
            } else if ch == '1' {
                num_ones += 1;
            }
        }
        if num_ones > num_zeroes {
            result_bit_string.push_str("0");
        } else {
            result_bit_string.push_str("1");
        }
    }
    return u32::from_str_radix(result_bit_string.as_str(), 2).unwrap();
}

fn calculate_oxygen_rating(bit_strings: &Vec<String>) -> u32 {
    let mut bit_strings = bit_strings.to_vec();
    let bit_string_len = bit_strings[0].len();
    for i in 0..bit_string_len {
        let mut num_zeroes = 0;
        let mut num_ones = 0;
        for bit_string in bit_strings.iter() {
            let ch = bit_string.chars().nth(i).unwrap();
            if ch == '0' {
                num_zeroes += 1;
            } else if ch == '1' {
                num_ones += 1;
            }
        }

        if num_ones >= num_zeroes {
            bit_strings = bit_strings
                .iter()
                .filter(|bit_string| bit_string.chars().nth(i).unwrap() == '1')
                .map(|bit_string| bit_string.to_string())
                .collect();
        } else {
            bit_strings = bit_strings
                .iter()
                .filter(|bit_string| bit_string.chars().nth(i).unwrap() == '0')
                .map(|bit_string| bit_string.to_string())
                .collect();
        }

        if bit_strings.len() == 1 {
            return u32::from_str_radix(bit_strings[0].as_str(), 2).unwrap();
        }
    }
    return 0;
}

fn calculate_co2_rating(bit_strings: &Vec<String>) -> u32 {
    let mut bit_strings = bit_strings.to_vec();
    let bit_string_len = bit_strings[0].len();
    for i in 0..bit_string_len {
        let mut num_zeroes = 0;
        let mut num_ones = 0;
        for bit_string in bit_strings.iter() {
            let ch = bit_string.chars().nth(i).unwrap();
            if ch == '0' {
                num_zeroes += 1;
            } else if ch == '1' {
                num_ones += 1;
            }
        }

        if num_zeroes <= num_ones {
            bit_strings = bit_strings
                .iter()
                .filter(|bit_string| bit_string.chars().nth(i).unwrap() == '0')
                .map(|bit_string| bit_string.to_string())
                .collect();
        } else {
            bit_strings = bit_strings
                .iter()
                .filter(|bit_string| bit_string.chars().nth(i).unwrap() == '1')
                .map(|bit_string| bit_string.to_string())
                .collect();
        }

        if bit_strings.len() == 1 {
            return u32::from_str_radix(bit_strings[0].as_str(), 2).unwrap();
        }
    }
    return 0;
}
