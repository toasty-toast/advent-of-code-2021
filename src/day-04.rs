mod utils;

#[derive(Default)]
struct BoardSpace {
    value: i32,
    marked: bool,
}

#[derive(Default)]
struct Board {
    spaces: Vec<Vec<BoardSpace>>,
}

impl Board {
    fn mark_value(&mut self, value: i32) {
        for row in self.spaces.iter_mut() {
            for space in row.iter_mut() {
                if space.value == value {
                    space.marked = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for row in self.spaces.iter() {
            let mut is_any_unmarked = false;
            for space in row.iter() {
                if !space.marked {
                    is_any_unmarked = true;
                    break;
                }
            }
            if !is_any_unmarked {
                return true;
            }
        }

        for i in 0..self.spaces[0].len() {
            let mut is_any_unmarked = false;
            for row in self.spaces.iter() {
                if !row[i].marked {
                    is_any_unmarked = true;
                    break;
                }
            }
            if !is_any_unmarked {
                return true;
            }
        }

        return false;
    }

    fn sum_unmarked_spaces(&self) -> i32 {
        let mut score = 0;
        for row in self.spaces.iter() {
            for space in row.iter() {
                if !space.marked {
                    score += space.value;
                }
            }
        }
        return score;
    }
}

fn main() {
    let input_lines: Vec<String> = utils::read_puzzle_input();
    let draws = load_draws(&input_lines[0]);
    let mut boards = load_boards(&input_lines[2..]);

    let mut first_winner_score = -1;
    let mut last_winner_score = -1;
    for draw in draws {
        if boards.len() == 0 {
            break;
        }

        for board in boards.iter_mut() {
            board.mark_value(draw);
        }

        let mut new_boards = Vec::<Board>::new();
        for board in boards {
            if board.is_winner() {
                if first_winner_score == -1 {
                    first_winner_score = board.sum_unmarked_spaces() * draw;
                }
                last_winner_score = board.sum_unmarked_spaces() * draw;
            } else {
                new_boards.push(board);
            }
        }
        boards = new_boards;
    }

    println!("Part 1: {}", first_winner_score);
    println!("Part 2: {}", last_winner_score);
}

fn load_draws(line: &String) -> Vec<i32> {
    return line
        .split(",")
        .map(|val| val.parse::<i32>().unwrap())
        .collect();
}

fn load_boards(lines: &[String]) -> Vec<Board> {
    let mut boards = Vec::<Board>::new();
    let mut board = Board::default();
    let mut board_row = 0;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }

        board.spaces.push(
            line.split(" ")
                .map(|val| val.trim())
                .filter(|val| val.len() > 0)
                .map(|val| BoardSpace {
                    value: val.parse::<i32>().unwrap(),
                    marked: false,
                })
                .collect::<Vec<BoardSpace>>(),
        );

        board_row += 1;
        if board_row > 4 {
            board_row = 0;
            boards.push(board);
            board = Board::default();
        }
    }
    return boards;
}
