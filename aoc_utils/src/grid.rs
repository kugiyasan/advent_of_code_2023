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

impl<T> Grid<T> {
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
        let valid_x = 0 <= p.x && p.x < (self.width as i32);
        let valid_y = 0 <= p.y && p.y < (self.height as i32);

        valid_x && valid_y
    }

    fn point_to_index(&self, p: Vec2) -> usize {
        let x = p.x as usize;
        let y = p.y as usize;
        y * self.width + x
    }
}

impl<T> Grid<T> {
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.grid.chunks(self.width)
    }

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
                Vec2::new((i % self.height) as i32, (i / self.height) as i32),
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
