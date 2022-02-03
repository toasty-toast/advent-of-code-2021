mod utils;

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    row: usize,
    col: usize,
}

fn main() {
    let heightmap: Vec<Vec<u32>> = utils::read_puzzle_input()
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut low_points = Vec::<Point>::new();
    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            let point = Point { row: i, col: j };
            if is_low_point(&heightmap, &point) {
                low_points.push(point);
            }
        }
    }

    let total_risk = low_points
        .iter()
        .map(|p| heightmap[p.row][p.col] + 1)
        .sum::<u32>();
    println!("Part 1: {}", total_risk);

    let basins = find_basins(&heightmap);
    let mut basin_sizes = basins.iter().map(|b| b.len()).collect::<Vec<usize>>();
    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!(
        "Part 2: {:?}",
        basin_sizes.iter().take(3).fold(1, |a, b| a * b)
    );
}

fn is_low_point(heightmap: &Vec<Vec<u32>>, point: &Point) -> bool {
    let value = heightmap[point.row][point.col];
    if point.row > 0 && heightmap[point.row - 1][point.col] <= value {
        return false;
    }
    if point.row < heightmap.len() - 1 && heightmap[point.row + 1][point.col] <= value {
        return false;
    }
    if point.col > 0 && heightmap[point.row][point.col - 1] <= value {
        return false;
    }
    if point.col < heightmap[point.row].len() - 1 && heightmap[point.row][point.col + 1] <= value {
        return false;
    }
    return true;
}

fn find_basins(heightmap: &Vec<Vec<u32>>) -> Vec<Vec<Point>> {
    let mut seen_points = HashSet::<Point>::new();
    let mut basins = Vec::<Vec<Point>>::new();
    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            let point = Point { row: i, col: j };
            if seen_points.contains(&point) {
                continue;
            }
            let mut basin = Vec::<Point>::new();
            get_basin(&heightmap, &point, &mut seen_points, &mut basin);
            if basin.len() > 0 {
                basins.push(basin);
            }
        }
    }
    return basins;
}

fn get_basin(
    heightmap: &Vec<Vec<u32>>,
    point: &Point,
    seen_points: &mut HashSet<Point>,
    basin: &mut Vec<Point>,
) {
    if seen_points.contains(point) {
        return;
    }

    seen_points.insert(*point);
    if heightmap[point.row][point.col] != 9 {
        basin.push(*point);
    } else {
        return;
    }

    if point.row > 0 {
        get_basin(
            heightmap,
            &Point {
                row: point.row - 1,
                col: point.col,
            },
            seen_points,
            basin,
        );
    }
    if point.row < heightmap.len() - 1 {
        get_basin(
            heightmap,
            &Point {
                row: point.row + 1,
                col: point.col,
            },
            seen_points,
            basin,
        );
    }
    if point.col > 0 {
        get_basin(
            heightmap,
            &Point {
                row: point.row,
                col: point.col - 1,
            },
            seen_points,
            basin,
        );
    }
    if point.col < heightmap[point.row].len() - 1 {
        get_basin(
            heightmap,
            &Point {
                row: point.row,
                col: point.col + 1,
            },
            seen_points,
            basin,
        );
    }
}
