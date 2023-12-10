use core::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        let n = self.x * self.x + self.y * self.y;
        (n as f32).sqrt()
    }

    pub fn manhattan_distance(&self, rhs: &Self) -> i32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        dx.abs() + dy.abs()
    }

    pub fn dot_product(&self, rhs: Self) -> i32 {
        self.x * rhs.y + self.y * rhs.y
    }
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({: >8}, {: >8})", self.x, self.y));
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

macro_rules! Vec2_minmax {
    ($name: ident, $field: ident, $op: tt) => {
        impl Vec2 {
            pub fn $name(self, rhs: Self) -> Self {
                if self.$field $op rhs.$field {
                    self
                } else {
                    rhs
                }
            }
        }
    };
}

Vec2_minmax!(min_x, x, <);
Vec2_minmax!(max_x, x, >);
Vec2_minmax!(min_y, y, <);
Vec2_minmax!(max_y, y, >);

macro_rules! impl_Vec2 {
    ($trait: ty, $method_name: ident, $op: tt) => {
        impl $trait for Vec2 {
            type Output = Self;

            fn $method_name(mut self, rhs: Self) -> Self::Output {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self
            }
        }
    };
}

impl_Vec2!(Add, add, +=);
impl_Vec2!(Sub, sub, -=);
impl_Vec2!(Mul, mul, *=);
impl_Vec2!(Div, div, /=);

impl Neg for Vec2 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.x *= -1;
        self.y *= -1;
        self
    }
}
