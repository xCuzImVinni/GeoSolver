use crate::geometry::*;
use crate::relations::Relation;

pub fn thales_theorem() -> impl Fn(&[Relation]) -> Option<Relation> {
    |relations| {
        for relation in relations {
            if let Relation::LiesOnCircle(c, circle) = relation {
                let a = circle.radius_point.clone();
                let o = circle.center.clone();
                let b = reflection_point(&o, &a);

                return Some(Relation::RightAngle(a, c.clone(), b));
            }
        }
        None
    }
}

fn reflection_point(_m: &Point, _p: &Point) -> Point {
    Point("B") // stub
}
