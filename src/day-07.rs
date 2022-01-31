mod utils;

fn main() {
    let positions: Vec<i32> = utils::read_puzzle_input()[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    {
        let mut least_cost = i32::MAX;
        let mut i = min_position;
        while i <= max_position {
            let cost = calculate_fuel_cost_basic(&positions, i);
            if cost < least_cost {
                least_cost = cost;
            }
            i += 1;
        }
        println!("Part 1: {}", least_cost);
    }
    {
        let mut least_cost = i32::MAX;
        let mut i = min_position;
        while i <= max_position {
            let cost = calculate_fuel_cost_advanced(&positions, i);
            if cost < least_cost {
                least_cost = cost;
            }
            i += 1;
        }
        println!("Part 2: {}", least_cost);
    }
}

fn calculate_fuel_cost_basic(positions: &Vec<i32>, location: i32) -> i32 {
    return positions
        .iter()
        .map(|p| i32::abs(location - p))
        .sum::<i32>();
}

fn calculate_fuel_cost_advanced(positions: &Vec<i32>, location: i32) -> i32 {
    return positions
        .iter()
        .map(|p| sum_numbers_between(1, i32::abs(location - p)))
        .sum::<i32>();
}

fn sum_numbers_between(a: i32, b: i32) -> i32 {
    return (b - a + 1) * (a + b) / 2;
}
