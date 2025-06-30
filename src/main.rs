mod geometry;
mod inference;
mod proof;
pub mod relations;
mod theorems;

use crate::geometry::*;
use crate::inference::*;
use crate::relations::Relation;
use proof::Proof;

use theorems::prove_similarity;

fn midpoint(a: &Point, b: &Point, name: &'static str) -> Point {
    Point::new(name, (a.x + b.x) / 2.0, (a.y + b.y) / 2.0)
}

fn main() {
    // Definition der relevanten Punkte symbolisch
    let m1 = Point::new("M1", 0.0, 0.0); // Mittelpunkt von k1
    let m2 = Point::new("M2", 4.0, 0.0); // Mittelpunkt von k2

    // Schnittpunkte (symbolisch, Koordinaten egal)
    let i1 = Point::new("I1", 0.0, 2.0);
    let i2 = Point::new("I2", 4.0, 2.0);

    // Kreise k1, k2
    let k1 = Circle {
        center: m1.clone(),
        radius_point: i1.clone(),
    };
    let k2 = Circle {
        center: m2.clone(),
        radius_point: i2.clone(),
    };

    // Punkte P auf k1, Q & R auf k2
    let p = Point::new("P", 2.0, 3.0);
    let q = Point::new("Q", 0.0, 3.0);
    let r = Point::new("R", 4.0, 3.0);

    // Mittelpunkte der Strecken PQ und PR
    let s = Point::new("S", 1.0, 3.0);
    let t = Point::new("T", 3.0, 3.0);

    // Strecken
    let pq = Line::new(p.clone(), q.clone());
    let pr = Line::new(p.clone(), r.clone());
    let m1m2 = Line::new(m1.clone(), m2.clone());

    // Alle bekannten Relationen
    let mut known = vec![
        // Punkte liegen auf Kreisen
        LiesOnCircle(p.clone(), k1.clone()),
        LiesOnCircle(q.clone(), k2.clone()),
        LiesOnCircle(r.clone(), k2.clone()),
        // Gleich lange Strecken
        EqualLength(pq.clone(), m1m2.clone()),
        EqualLength(pr.clone(), m1m2.clone()),
        // Mittelpunkte
        Midpoint(s.clone(), p.clone(), q.clone()),
        Midpoint(t.clone(), p.clone(), r.clone()),
    ];
    known.push(Relation::equal_length(
        Line::new(m1.clone(), i1.clone()),
        Line::new(m2.clone(), i1.clone()),
    ));
    known.push(Relation::right_angle(i1.clone(), m1.clone(), i2.clone()));
    known.push(Relation::right_angle(i1.clone(), m2.clone(), i2.clone()));

    // Führe Inferenz aus
    infer_all(&mut known);

    // Beweisschritte ausgeben
    for rel in &known {
        println!("{}", rel.description());
    }
}
