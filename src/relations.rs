use crate::geometry::{Circle, Line, Point, Triangle};

#[derive(Debug, Clone, PartialEq)]
pub enum Relation {
    LiesOnCircle(Point, Circle),
    LiesOnLine(Point, Line),
    LiesOnTriangle(Point, Triangle),
    RightAngle(Point, Point, Point), // Angle ABC = 90 degrees
    IsDiameter(Line, Circle),
    EqualLength(Line, Line),
    Midpoint(Point, Point, Point),
    SimilarTriangle(Triangle, Triangle),
    CongruentTriangles(Triangle, Triangle),
    EqualAngles(Point, Point, Point, Point, Point, Point), // ∠ABC = ∠DEF
}

impl Relation {
    pub fn lies_on_circle(p: Point, c: Circle) -> Self {
        Relation::LiesOnCircle(p, c)
    }

    pub fn lies_on_line(p: Point, l: Line) -> Self {
        Relation::LiesOnLine(p, l)
    }

    pub fn lies_on_triangle(p: Point, t: Triangle) -> Self {
        Relation::LiesOnTriangle(p, t)
    }

    pub fn right_angle(a: Point, b: Point, c: Point) -> Self {
        Relation::RightAngle(a, b, c)
    }

    pub fn is_diameter(l: Line, c: Circle) -> Self {
        Relation::IsDiameter(l, c)
    }

    pub fn equal_length(a: Line, b: Line) -> Self {
        Relation::EqualLength(a, b)
    }

    pub fn midpoint(m: Point, a: Point, b: Point) -> Self {
        Relation::Midpoint(m, a, b)
    }

    pub fn similar_triangle_from_points(
        p1: Point,
        p2: Point,
        p3: Point,
        q1: Point,
        q2: Point,
        q3: Point,
    ) -> Self {
        let t1 = Triangle {
            a: p1,
            b: p2,
            c: p3,
        };
        let t2 = Triangle {
            a: q1,
            b: q2,
            c: q3,
        };
        Relation::SimilarTriangle(t1, t2)
    }

    pub fn similar_triangle(t1: Triangle, t2: Triangle) -> Self {
        Relation::SimilarTriangle(t1, t2)
    }

    pub fn equal_angles(a: Point, b: Point, c: Point, d: Point, e: Point, f: Point) -> Self {
        Relation::EqualAngles(a, b, c, d, e, f)
    }

    pub fn congruent_triangles(t1: Triangle, t2: Triangle) -> Self {
        Relation::CongruentTriangles(t1, t2)
    }
}

pub fn description(&self) -> String {
    match self {
        Relation::LiesOnCircle(p, _) => format!("{} lies on circle", p.name),
        Relation::LiesOnLine(p, _) => format!("{} lies on line", p.name),
        Relation::LiesOnTriangle(p, _) => format!("{} lies on triangle", p.name),
        Relation::RightAngle(a, b, c) => format!("Angle {}{}{} = 90°", a.name, b.name, c.name),
        Relation::IsDiameter(_, _) => "Line is a diameter of circle".to_string(),
        Relation::EqualLength(l1, l2) => format!(
            "|{}{}| = |{}{}|",
            l1.0.name, l1.1.name, l2.0.name, l2.1.name
        ),
        Relation::Midpoint(m, a, b) => {
            format!("{} is midpoint of {} and {}", m.name, a.name, b.name)
        }
        Relation::SimilarTriangle(t1, t2) => format!(
            "△{}{}{} ~ △{}{}{}",
            t1.a.name, t1.b.name, t1.c.name, t2.a.name, t2.b.name, t2.c.name
        ),
    }
}
