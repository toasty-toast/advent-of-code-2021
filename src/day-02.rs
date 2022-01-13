mod utils;

struct Command {
    direction: String,
    distance: i32,
}

fn main() {
    let commands = load_commands(utils::read_puzzle_input());

    let mut x = 0;
    let mut y = 0;
    for command in commands.iter() {
        match command.direction.as_str() {
            "down" => y += command.distance,
            "up" => y -= command.distance,
            "forward" => x += command.distance,
            _ => panic!("Invalid command direction"),
        }
    }
    println!("Part 1: {}", x * y);

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for command in commands.iter() {
        match command.direction.as_str() {
            "down" => aim += command.distance,
            "up" => aim -= command.distance,
            "forward" => {
                x += command.distance;
                y += aim * command.distance;
            }
            _ => panic!("Invalid command direction"),
        }
    }
    println!("Part 2: {}", x * y);
}

fn load_commands(input_lines: Vec<String>) -> Vec<Command> {
    let mut commands = Vec::<Command>::new();
    for line in input_lines.iter() {
        let parts: Vec<&str> = line.split(" ").collect();
        commands.push(Command {
            direction: parts[0].to_string(),
            distance: parts[1].to_string().parse::<i32>().unwrap(),
        });
    }
    return commands;
}
