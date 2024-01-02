use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Component, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn in_bounds(&self, point1: Point, point2: Point) -> bool {
        self.x >= point1.x && self.x <= point2.x && self.y >= point1.y && self.y <= point2.y
    }
}

impl std::ops::Add<Vec3> for Point {
    type Output = Point;
    fn add(self, rhs: Vec3) -> Self::Output {
        Point {
            x: (self.x as f32 + rhs.x) as i32,
            y: (self.y as f32 + rhs.y) as i32,
        }
    }
}

impl std::ops::AddAssign<Vec3> for Point {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x = (self.x as f32 + rhs.x) as i32;
        self.y = (self.y as f32 + rhs.y) as i32;
    }
}

impl std::ops::Add<&Rotation> for &Point {
    type Output = Point;
    fn add(self, rhs: &Rotation) -> Self::Output {
        match rhs {
            &Rotation::Up => Point::new(self.x, self.y + 1),
            &Rotation::Down => Point::new(self.x, self.y - 1),
            &Rotation::Left => Point::new(self.x - 1, self.y),
            &Rotation::Right => Point::new(self.x + 1, self.y),
        }
    }
}

impl PartialEq<Vec3> for Point {
    fn eq(&self, rhs: &Vec3) -> bool {
        self.x == rhs.x as i32 && self.y == rhs.y as i32
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, rhs: &Point) -> bool {
        self.x as i32 == rhs.x && self.y as i32 == rhs.y
    }
}

impl std::ops::Add<Point> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Point) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x as f32,
            y: self.y + rhs.y as f32,
            z: self.z,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[x: {}, y: {}]", self.x, self.y)
    }
}

#[derive(Component, PartialEq, Eq, Clone)]
pub enum Rotation {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Rotation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        match rng.gen_range(0..4) {
            0 => Rotation::Up,
            1 => Rotation::Down,
            2 => Rotation::Left,
            _ => Rotation::Right,
        }
    }
}
