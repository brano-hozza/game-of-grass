use bevy::prelude::*;

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

#[derive(Component)]
pub enum Rotation {
    Up,
    Down,
    Left,
    Right,
}
