mod utils;

fn main() {
    let initial_states: Vec<usize> = utils::read_puzzle_input()[0]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut timers: Vec<u64> = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    for state in initial_states.iter() {
        timers[*state] += 1;
    }

    for day in 1..257 {
        let old_timers = timers.to_vec();
        for i in 0..9 {
            timers[i] = old_timers[(i + 1) % 9];
        }
        timers[6] += old_timers[0];

        if day == 80 {
            println!("Part 1: {}", timers.iter().sum::<u64>());
        }
    }
    println!("Part 2: {}", timers.iter().sum::<u64>());
}
