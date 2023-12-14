use crate::Vec2;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            grid: vec![value; width * height],
            width,
            height,
        }
    }
}

impl Grid<char> {
    /// TODO: from a double iterator?
    pub fn from_str(lines: Vec<&str>) -> Self {
        let grid = lines
            .iter()
            .flat_map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        let width = lines[0].len();
        let height = lines.len();

        Grid::from_vec(grid, width, height)
    }
}

impl<T> Grid<T> {
    pub fn from_vec(grid: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(grid.len(), width * height);
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, p: Vec2) -> &T {
        &self.grid[self.point_to_index(p)]
    }

    pub fn set(&mut self, p: Vec2, value: T) {
        let i = self.point_to_index(p);
        self.grid[i] = value;
    }

    pub fn is_in_bound(&self, p: Vec2) -> bool {
        self.is_x_in_bound(p.x) && self.is_y_in_bound(p.y)
    }

    fn is_x_in_bound(&self, x: i32) -> bool {
        0 <= x && x < (self.width as i32)
    }

    fn is_y_in_bound(&self, y: i32) -> bool {
        0 <= y && y < (self.height as i32)
    }

    fn point_to_index(&self, p: Vec2) -> usize {
        let x = p.x as usize;
        let y = p.y as usize;
        y * self.width + x
    }
}

impl<T> Grid<T> {
    pub fn rows_chunks(&self) -> impl Iterator<Item = &[T]> {
        self.grid.chunks(self.width)
    }

    // TODO: immutable iterator breaks this method
    // pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    //     (0..self.height).map(|y| self.get_row(y))
    // }
    //
    // pub fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    //     (0..self.width).map(|x| self.get_column(x))
    // }

    pub fn get_row(&self, row_index: usize) -> impl Iterator<Item = &T> {
        let start = row_index * self.width;
        let end = start + self.width;
        self.grid[start..end].iter()
    }

    pub fn get_column(&self, column_index: usize) -> impl Iterator<Item = &T> {
        self.grid.iter().skip(column_index).step_by(self.width)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Vec2, &T)> {
        self.grid.iter().enumerate().map(|(i, cell)| {
            (
                Vec2::new((i % self.width) as i32, (i / self.width) as i32),
                cell,
            )
        })
    }
}

impl<T> Grid<T> {
    pub fn neighbors(&self, origin: Vec2) -> impl Iterator<Item = &T> {
        let coords = [
            Vec2::new(-1, -1),
            Vec2::new(-1, 0),
            Vec2::new(-1, 1),
            Vec2::new(0, -1),
            Vec2::new(0, 1),
            Vec2::new(1, -1),
            Vec2::new(1, 0),
            Vec2::new(1, 1),
        ];

        coords.into_iter().map(move |p| self.get(origin + p))
    }

    pub fn neighbors_cross(&self, origin: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        let coords = [
            Vec2::new(-1, 0),
            Vec2::new(0, -1),
            Vec2::new(0, 1),
            Vec2::new(1, 0),
        ];

        coords
            .into_iter()
            .map(move |p| origin + p)
            .filter(|&p| self.is_in_bound(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_str() {
        let lines = vec!["123", "456", "789"];
        let grid = Grid::from_str(lines);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn test_grid_cols() {
        // let lines = vec!["123", "456", "789"];
        // let grid = Grid::from_str(lines);
        //
        // let actual = grid
        //     .cols()
        //     .map(|col| col.collect::<Vec<_>>())
        //     .collect::<Vec<_>>();
        //
        // let expected = vec![
        //     vec!["1", "4", "7"],
        //     vec!["2", "5", "8"],
        //     vec!["3", "6", "9"],
        // ];
        // assert_eq!(actual)
    }
}
