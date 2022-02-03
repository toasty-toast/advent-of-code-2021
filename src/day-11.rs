mod utils;

fn main() {
    let mut state = utils::read_puzzle_input()
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut step_number = 0;
    let mut total_flashes = 0;
    loop {
        step_number += 1;
        let new_flashes = step(&mut state);
        if step_number <= 100 {
            total_flashes += new_flashes;
            if step_number == 100 {
                println!("Part 1: {}", total_flashes);
            }
        }
        if are_flashes_synced(&state) {
            println!("Part 2: {}", step_number);
            break;
        }
    }
}

fn step(state: &mut Vec<Vec<u32>>) -> u32 {
    let mut flash_count = 0;
    let mut flashed = Vec::<Vec<bool>>::new();
    for i in 0..state.len() {
        flashed.push(Vec::<bool>::new());
        for _ in 0..state[i].len() {
            flashed[i].push(false);
        }
    }

    for i in 0..state.len() {
        for j in 0..state[i].len() {
            if flashed[i][j] {
                continue;
            }
            increment(state, &mut flashed, &mut flash_count, i, j);
            if state[i][j] > 9 {
                flash(state, &mut flashed, &mut flash_count, i, j);
            }
        }
    }

    return flash_count;
}

fn increment(
    state: &mut Vec<Vec<u32>>,
    flashed: &mut Vec<Vec<bool>>,
    flash_count: &mut u32,
    row: usize,
    col: usize,
) {
    if flashed[row][col] {
        return;
    }

    state[row][col] += 1;
    if state[row][col] > 9 {
        flash(state, flashed, flash_count, row, col);
    }
}

fn flash(
    state: &mut Vec<Vec<u32>>,
    flashed: &mut Vec<Vec<bool>>,
    flash_count: &mut u32,
    row: usize,
    col: usize,
) {
    if flashed[row][col] {
        return;
    }

    state[row][col] = 0;
    flashed[row][col] = true;
    *flash_count += 1;

    // Increment the 3 spaces above this one.
    if row > 0 {
        if col > 0 {
            increment(state, flashed, flash_count, row - 1, col - 1);
        }
        increment(state, flashed, flash_count, row - 1, col);
        if col < state[row].len() - 1 {
            increment(state, flashed, flash_count, row - 1, col + 1);
        }
    }

    // Increment the spaces left and right of this one.
    if col > 0 {
        increment(state, flashed, flash_count, row, col - 1);
    }
    if col < state[row].len() - 1 {
        increment(state, flashed, flash_count, row, col + 1);
    }

    // Increment the spaces below this one.
    if row < state[row].len() - 1 {
        if col > 0 {
            increment(state, flashed, flash_count, row + 1, col - 1);
        }
        increment(state, flashed, flash_count, row + 1, col);
        if col < state[row].len() - 1 {
            increment(state, flashed, flash_count, row + 1, col + 1);
        }
    }
}

fn are_flashes_synced(state: &Vec<Vec<u32>>) -> bool {
    for row in state.iter() {
        for col in row.iter() {
            if *col != 0 {
                return false;
            }
        }
    }
    return true;
}
