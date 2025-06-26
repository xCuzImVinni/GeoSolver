mod geometry;
mod inference;
pub mod relations;
mod theorems;
use crate::geometry::*;
use crate::inference::*;
use crate::relations::Relation;
fn main() {
    let a = Point("A");
    let b = Point("B");
    let c = Point("C");
    let o = Point("O");

    let circle = Circle {
        center: o.clone(),
        radius_point: a.clone(),
    };

    let mut known = vec![Relation::LiesOn(c.clone(), circle.clone())];

    infer_all(&mut known);

    for r in known {
        println!("{:?}", r);
    }
}
