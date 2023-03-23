use std::ops::{Add, Div, Mul, Sub};

use crate::structs::{Point, self};

impl Add for Point {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, b: Self) -> Point {
        Point {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl Add<Point> for &Point {
    type Output = Point;
    fn add(self, b: Point) -> Point {
        Point {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;
    fn add(self, b: &Self) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl Add<f32> for Point {
    type Output = Self;
    fn add(self, b: f32) -> Self {
        Self {
            x: self.x + b,
            y: self.y + b,
            z: self.z + b,
        }
    }
}

impl Add<f32> for &Point {
    type Output = Point;
    fn add(self, b: f32) -> Point {
        Point {
            x: self.x + b,
            y: self.y + b,
            z: self.z + b,
        }
    }
}

impl Add<&Point> for f32 {
    type Output = Point;
    fn add(self, b: &Point) -> Point {
        Point {
            x: self + b.x,
            y: self + b.y,
            z: self + b.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, b: Self) -> Point {
        Point {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Self;
    fn sub(self, b: &Self) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

impl Sub<Point> for &Point {
    type Output = Point;
    fn sub(self, b: Point) -> Point {
        Point {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

impl Sub<f32> for Point {
    type Output = Self;
    fn sub(self, b: f32) -> Self {
        Self {
            x: self.x - b,
            y: self.y - b,
            z: self.z - b,
        }
    }
}

impl Sub<f32> for &Point {
    type Output = Point;
    fn sub(self, b: f32) -> Point {
        Point {
            x: self.x - b,
            y: self.y - b,
            z: self.z - b,
        }
    }
}

impl Sub<&Point> for f32 {
    type Output = Point;
    fn sub(self, b: &Point) -> Point {
        Point {
            x: self - b.x,
            y: self - b.y,
            z: self - b.z,
        }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Self {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        }
    }
}

impl Mul for &Point {
    type Output = Point;
    fn mul(self, b: Self) -> Point {
        Point {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        }
    }
}

impl Mul<Point> for &Point {
    type Output = Point;
    fn mul(self, b: Point) -> Point {
        Point {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        }
    }
}

impl Mul<&Point> for Point {
    type Output = Self;
    fn mul(self, b: &Self) -> Self {
        Self {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, b: f32) -> Self {
        Self {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
        }
    }
}

impl Mul<f32> for &Point {
    type Output = Point;
    fn mul(self, b: f32) -> Point {
        Point {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, b: Point) -> Point {
        Point {
            x: self * b.x,
            y: self * b.y,
            z: self * b.z,
        }
    }
}

impl Mul<&Point> for f32 {
    type Output = Point;
    fn mul(self, b: &Point) -> Point {
        Point {
            x: self * b.x,
            y: self * b.y,
            z: self * b.z,
        }
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        Self {
            x: self.x / b.x,
            y: self.y / b.y,
            z: self.z / b.z,
        }
    }
}

impl Div for &Point {
    type Output = Point;
    fn div(self, b: Self) -> Point {
        Point {
            x: self.x / b.x,
            y: self.y / b.y,
            z: self.z / b.z,
        }
    }
}

impl Div<Point> for &Point {
    type Output = Point;
    fn div(self, b: Point) -> Point {
        Point {
            x: self.x / b.x,
            y: self.y / b.y,
            z: self.z / b.z,
        }
    }
}

impl Div<&Point> for Point {
    type Output = Self;
    fn div(self, b: &Self) -> Self {
        Self {
            x: self.x / b.x,
            y: self.y / b.y,
            z: self.z / b.z,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;
    fn div(self, b: f32) -> Self {
        Self {
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
        }
    }
}

impl Div<f32> for &Point {
    type Output = Point;
    fn div(self, b: f32) -> Point {
        Point {
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
        }
    }
}

impl Div<&Point> for f32 {
    type Output = Point;
    fn div(self, b: &Point) -> Point {
        Point {
            x: self / b.x,
            y: self / b.y,
            z: self / b.z,
        }
    }
}
