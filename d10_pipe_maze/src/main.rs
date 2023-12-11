mod direction;

use crate::direction::is_compatible;
use aoc_utils::*;
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

    let inside_loop_count = grid
        .enumerate()
        .filter(|(_, &c)| c == '.')
        .filter(|(p, _)| {
            // println!("{:?}", p);
            let left = (0..p.x - 1)
                .filter(|&x| the_loop.contains(&Vec2::new(x, p.y)))
                .count();

            let right = (p.x + 1..grid.width() as i32)
                .filter(|&x| the_loop.contains(&Vec2::new(x, p.y)))
                .count();

            let up = (0..p.y - 1)
                .filter(|&y| the_loop.contains(&Vec2::new(p.x, y)))
                .count();

            let down = (p.y + 1..grid.height() as i32)
                .filter(|&y| the_loop.contains(&Vec2::new(p.x, y)))
                .count();

            // println!("{} {} {} {}", up, down, left, right);
            // inside if odd
            left % 2 == 1 && right % 2 == 1 && up % 2 == 1 && down % 2 == 1
        })
        // .map(|n| println!("{:?}", n))
        .count();

    println!("{:?}", the_loop);
    println!("{:?}", inside_loop_count);
    inside_loop_count
}

fn main() {
    // let path = "input";
    let path = "example4";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();

    let grid = create_grid(&input);

    let (start_pos, _) = grid.enumerate().find(|(_p, c)| **c == 'S').unwrap();
    println!("{:?}", start_pos);

    // let answer = find_farthest_position(&grid, start_pos);

    let answer = find_inside_loop_count(&grid, start_pos);

    // println!("{:?}", &grid);
    println!("{:?}", answer);
    // println!("{:?}", &input[0..5]);
    // submit(&answer.to_string(), false);
}
