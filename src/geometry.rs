#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub name: &'static str,
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(name: &'static str, x: f64, y: f64) -> Self {
        Self { name, x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line(pub Point, pub Point);

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self(a, b)
    }

    pub fn length(&self) -> f64 {
        let dx = self.1.x - self.0.x;
        let dy = self.1.y - self.0.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn midpoint(&self) -> Point {
        let mx = (self.0.x + self.1.x) / 2.0;
        let my = (self.0.y + self.1.y) / 2.0;
        Point::new("M", mx, my)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius_point: Point,
}
impl Circle {
    pub fn new(center: Point, radius_point: Point) -> Self {
        Self {
            center,
            radius_point,
        }
    }
    pub fn reflection_point(center: &Point, original: &Point, new_name: &'static str) -> Point {
        let x = 2.0 * center.x - original.x;
        let y = 2.0 * center.y - original.y;
        Point::new(new_name, x, y)
    }

    pub fn radius(&self) -> f64 {
        let dx = self.center.x - self.radius_point.x;
        let dy = self.center.y - self.radius_point.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn diameter_line(&self) -> Line {
        let other = Circle::reflection_point(&self.center, &self.radius_point, "B"); // symmetrischer Punkt
        Line::new(self.radius_point.clone(), other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    pub fn perimeter(&self) -> f64 {
        let ab = Line::new(self.a.clone(), self.b.clone()).length();
        let bc = Line::new(self.b.clone(), self.c.clone()).length();
        let ca = Line::new(self.c.clone(), self.a.clone()).length();
        ab + bc + ca
    }
}
