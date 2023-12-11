use aoc_utils::{Grid, Vec2};

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

pub fn is_compatible(grid: &Grid<char>, start_pos: Vec2, curr: Vec2, neighbor: Vec2) -> bool {
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
