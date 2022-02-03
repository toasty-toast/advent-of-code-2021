mod utils;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Character {
    Parentheses,
    Bracket,
    Brace,
    AngleBracket,
}

fn main() {
    let mut lines = utils::read_puzzle_input();

    let mut syntax_error_score = 0;
    for i in (0..lines.to_vec().len()).rev() {
        match get_first_invalid_char(&lines[i]) {
            Some(c) => {
                syntax_error_score += get_syntax_error_score(&c);
                lines.remove(i);
            }
            None => {}
        }
    }
    println!("Part 1: {}", syntax_error_score);

    let mut completion_scores = lines
        .iter()
        .map(|line| get_completion_score(&get_completion(&line)))
        .collect::<Vec<u64>>();
    completion_scores.sort();
    println!(
        "Part 2: {}",
        completion_scores.get(completion_scores.len() / 2).unwrap()
    );
}

fn get_first_invalid_char(line: &String) -> Option<char> {
    let mut char_stack = VecDeque::<Character>::new();
    for c in line.chars() {
        match c {
            '(' => char_stack.push_back(Character::Parentheses),
            '[' => char_stack.push_back(Character::Bracket),
            '{' => char_stack.push_back(Character::Brace),
            '<' => char_stack.push_back(Character::AngleBracket),
            ')' => {
                if char_stack.pop_back().unwrap() != Character::Parentheses {
                    return Some(')');
                }
            }
            ']' => {
                if char_stack.pop_back().unwrap() != Character::Bracket {
                    return Some(']');
                }
            }
            '}' => {
                if char_stack.pop_back().unwrap() != Character::Brace {
                    return Some('}');
                }
            }
            '>' => {
                if char_stack.pop_back().unwrap() != Character::AngleBracket {
                    return Some('>');
                }
            }
            _ => panic!("Unexpected character in input"),
        }
    }
    return None;
}

fn get_syntax_error_score(character: &char) -> u32 {
    return match character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
}

fn get_completion(line: &String) -> Vec<char> {
    let mut char_stack = VecDeque::<Character>::new();
    for c in line.chars() {
        match c {
            '(' => {
                char_stack.push_back(Character::Parentheses);
            }
            '[' => {
                char_stack.push_back(Character::Bracket);
            }
            '{' => {
                char_stack.push_back(Character::Brace);
            }
            '<' => {
                char_stack.push_back(Character::AngleBracket);
            }
            ')' => {
                char_stack.pop_back();
            }
            ']' => {
                char_stack.pop_back();
            }
            '}' => {
                char_stack.pop_back();
            }
            '>' => {
                char_stack.pop_back();
            }
            _ => panic!("Unexpected character in input"),
        }
    }

    let mut completion_chars = Vec::<char>::new();
    while !char_stack.is_empty() {
        let next_completion_char = match char_stack.pop_back().unwrap() {
            Character::Parentheses => ')',
            Character::Bracket => ']',
            Character::Brace => '}',
            Character::AngleBracket => '>',
        };
        completion_chars.push(next_completion_char);
    }
    return completion_chars;
}

fn get_completion_score(chars: &Vec<char>) -> u64 {
    let mut score = 0;
    for c in chars.iter() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
    }
    return score;
}
