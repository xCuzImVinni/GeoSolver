#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point(pub &'static str);

#[derive(Debug, Clone)]
pub struct Line(pub Point, pub Point);

#[derive(Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius_point: Point,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}
