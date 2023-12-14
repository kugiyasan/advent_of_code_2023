use aoc_utils::*;
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let part2 = true;
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();
    let grid = Grid::from_str(input);

    let empty_rows = (0..grid.height())
        .map(|x| grid.get_row(x).all(|&c| c == '.'))
        .collect::<Vec<_>>();
    let empty_cols = (0..grid.width())
        .map(|x| grid.get_column(x).all(|&c| c == '.'))
        .collect::<Vec<_>>();

    let galaxies = grid
        .enumerate()
        .filter_map(|(p, &c)| if c == '#' { Some(p) } else { None })
        .collect::<Vec<_>>();

    println!("{:?}", empty_rows);
    println!("{:?}", empty_cols);
    println!("{:?}", galaxies);

    let mut sum = 0;

    for a in 0..galaxies.len() {
        for b in a + 1..galaxies.len() {
            let a = galaxies[a];
            let b = galaxies[b];

            let x_expansion = (a.min_x(b).x..a.max_x(b).x)
                .filter(|&x| empty_cols[x as usize])
                .count();

            let y_expansion = (a.min_y(b).y..a.max_y(b).y)
                .filter(|&y| empty_rows[y as usize])
                .count();

            let dist = if !part2 {
                a.manhattan_distance(&b) as usize + x_expansion + y_expansion
            } else {
                // let expansion_size = 10 - 1;
                let expansion_size = 1_000_000 - 1;
                a.manhattan_distance(&b) as usize
                    + x_expansion * expansion_size
                    + y_expansion * expansion_size
            };
            sum += dist;
        }
    }

    println!("{:?}", sum);
    // println!("{:?}", &input[0..5]);
    // submit(&sum.to_string(), part2);
}
