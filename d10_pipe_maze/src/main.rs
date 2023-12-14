mod direction;

use crate::direction::is_compatible;
use aoc_utils::*;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

/// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
fn dijkstra(grid: &Grid<char>, start_pos: Vec2) -> (HashMap<Vec2, i32>, HashMap<Vec2, Vec2>) {
    let mut queue = vec![start_pos];
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    dist.insert(start_pos, 0);

    // TODO implement custom BinaryHeap<T> where T = (Vec2, i32) and Ord<T> = T.1
    while let Some(curr) = pop_min_dist(&mut queue, &start_pos) {
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

    (dist, prev)
}

fn find_farthest_position(grid: &Grid<char>, start_pos: Vec2) -> usize {
    let (dist, prev) = dijkstra(grid, start_pos);

    let (mut curr, _) = dist.iter().max_by_key(|(_, v)| *v).unwrap();
    let mut path = vec![*curr];
    while let Some(p) = prev.get(curr) {
        curr = p;
        path.push(*curr);
    }

    path.len() - 1
}

fn find_loop(grid: &Grid<char>, start_pos: Vec2) -> HashSet<Vec2> {
    let (dist, _) = dijkstra(grid, start_pos);
    println!("{:?}", dist);

    dist.keys().map(|p| *p).collect()
}

fn find_inside_loop_count(grid: &Grid<char>, start_pos: Vec2) -> usize {
    let the_loop = find_loop(&grid, start_pos);
    // let the_loop: HashMap<Vec2, i32> = find_loop(&grid, start_pos)
    //     .into_iter()
    //     .map(|p| {
    //         let c = *grid.get(p);
    //         // TODO check for S
    //         if c == '-' || c == '|' {
    //             (p, 2)
    //         } else {
    //             (p, 1)
    //         }
    //     })
    //     .collect();

    let inside_loop_count = grid
        .enumerate()
        .filter(|(p, _)| !the_loop.contains(p))
        .filter(|(p, _)| {
            let mut left = grid.get_row(p.y as usize).collect::<Vec<_>>();
            if p.y == 4 {
                println!("{:?}", left);
            }
            left.truncate(p.x as usize);
            // TODO remove chars that aren't in the_loop
            let left = left
                .into_iter()
                .enumerate()
                .filter(|(i, c)| the_loop.contains(&Vec2::new(*i as i32, p.y)))
                .map(|t| t.1)
                .collect::<String>();

            let fj = Regex::new("F-*J").unwrap();
            let a = fj.find_iter(&left).count();

            let l7 = Regex::new("L-*7").unwrap();
            let b = l7.find_iter(&left).count();

            let pipe = Regex::new(r"\|").unwrap();
            let c = pipe.find_iter(&left).count();

            // println!("{} {} {}", a, b, c);
            (a + b + c) % 2 == 1
        })
        .map(|n| println!("{:?}", n))
        .count();

    // println!("{:?}", the_loop);
    // println!("{:?}", inside_loop_count);
    inside_loop_count
}

fn read_file_and_solve(path: &str) {
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();

    let mut grid = create_grid(&input);

    let (start_pos, _) = grid.enumerate().find(|(_p, c)| **c == 'S').unwrap();
    // println!("{:?}", start_pos);
    let c = match path {
        "input" => '-',
        "example3" => 'F',
        "example4" => 'F',
        "example5" => '7',
        _ => 'S',
    };
    grid.set(start_pos, c);

    // let answer = find_farthest_position(&grid, start_pos);

    let answer = find_inside_loop_count(&grid, start_pos);

    // println!("{:?}", &grid);
    println!("{:?}", answer);
    // println!("{:?}", &input[0..5]);
}

fn main() {
    let path = "example3";
    read_file_and_solve(path);

    let path = "example4";
    read_file_and_solve(path);

    let path = "example5";
    read_file_and_solve(path);

    let path = "input";
    read_file_and_solve(path);

    // submit(&answer.to_string(), false);
}
