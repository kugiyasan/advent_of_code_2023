use aoc_utils::*;
use std::{collections::HashMap, fs};

fn create_grid(input: &Vec<&str>) -> Grid<char> {
    let width = input[0].len();
    let height = input.len();
    let mut grid = Grid::new(width, height, '.');
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(Vec2::new(x as i32, y as i32), c);
        }
    }

    grid
}

fn pop_min_dist(queue: &mut Vec<Vec2>, start_pos: &Vec2) -> Option<Vec2> {
    let index = queue
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.manhattan_distance(start_pos))
        .map(|(i, _)| i);

    if let Some(i) = index {
        Some(queue.swap_remove(i))
    } else {
        None
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_up_compatible(c: char) -> bool {
    "LJ|".contains(c)
}

fn is_down_compatible(c: char) -> bool {
    "F7|".contains(c)
}

fn is_left_compatible(c: char) -> bool {
    "7J-".contains(c)
}

fn is_right_compatible(c: char) -> bool {
    "FL-".contains(c)
}

fn is_compatible(grid: &Grid<char>, start_pos: Vec2, curr: Vec2, neighbor: Vec2) -> bool {
    let c1 = grid.get(curr);
    let c2 = grid.get(neighbor);

    let delta = neighbor - curr;
    let direction = match (delta.x, delta.y) {
        (-1, 0) => Direction::Left,
        (1, 0) => Direction::Right,
        (0, -1) => Direction::Up,
        (0, 1) => Direction::Down,
        _ => panic!("invalid direction"),
    };

    let is_c1_compatible = if curr == start_pos {
        true
    } else {
        match direction {
            Direction::Up => is_up_compatible(*c1),
            Direction::Down => is_down_compatible(*c1),
            Direction::Left => is_left_compatible(*c1),
            Direction::Right => is_right_compatible(*c1),
        }
    };

    let is_c2_compatible = match direction {
        Direction::Up => is_down_compatible(*c2),
        Direction::Down => is_up_compatible(*c2),
        Direction::Left => is_right_compatible(*c2),
        Direction::Right => is_left_compatible(*c2),
    };

    is_c1_compatible && is_c2_compatible
}

/// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
fn dijkstra(grid: &Grid<char>, start_pos: Vec2) -> Vec<Vec2> {
    let mut queue = vec![start_pos];
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    dist.insert(start_pos, 0);

    // TODO implement custom BinaryHeap<T> where T = (Vec2, i32) and Ord<T> = T.1
    while let Some(curr) = pop_min_dist(&mut queue, &start_pos) {
        println!("curr = {:?}", curr);
        for neighbor in grid.neighbors_cross(curr) {
            if !is_compatible(grid, start_pos, curr, neighbor) {
                continue;
            }

            let alt = dist[&curr] + 1;

            if dist.get(&neighbor).map(|d| alt < *d).unwrap_or(true) {
                dist.insert(neighbor, alt);
                prev.insert(neighbor, curr);

                queue.push(neighbor);
            }
        }
    }

    // TODO split in another function
    println!("{:?}", dist);
    println!("{:?}", prev);
    let (mut curr, _) = dist.iter().max_by_key(|(_, v)| *v).unwrap();
    let mut path = vec![*curr];
    while let Some(p) = prev.get(curr) {
        curr = p;
        path.push(*curr);
    }

    path
}

fn main() {
    let path = "input";
    // let path = "example2";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();

    let grid = create_grid(&input);

    let (start_pos, _) = grid.enumerate().find(|(_p, c)| **c == 'S').unwrap();

    let path = dijkstra(&grid, start_pos);
    let answer = path.len() - 1;

    println!("{:?}", &grid);
    println!("{:?}", &path);
    println!("{:?}", answer);
    // println!("{:?}", &input[0..5]);
    // submit(&answer.to_string(), false);
}
