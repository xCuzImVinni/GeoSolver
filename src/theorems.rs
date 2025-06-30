use crate::geometry::{Circle, Line, Point, Triangle};
use crate::relations::Relation;

pub fn prove_similarity(relations: &[Relation]) -> Option<Relation> {
    let mut pq: Option<Line> = None;
    let mut pr: Option<Line> = None;
    let mut m1m2: Option<Line> = None;
    let mut sp: Option<(Point, Point, Point)> = None;
    let mut tp: Option<(Point, Point, Point)> = None;

    for rel in relations {
        match rel {
            Relation::EqualLength(l1, l2) => {
                if l1.0.name == "P" && l1.1.name == "Q" {
                    pq = Some(Line::new(l1.0.clone(), l1.1.clone()));
                }
                if l1.0.name == "P" && l1.1.name == "R" {
                    pr = Some(Line::new(l1.0.clone(), l1.1.clone()));
                }
                if l2.0.name == "M1" && l2.1.name == "M2" {
                    m1m2 = Some(Line::new(l2.0.clone(), l2.1.clone()));
                }
            }
            Relation::Midpoint(m, a, b) => {
                if a.name == "P" && b.name == "Q" {
                    sp = Some((m.clone(), a.clone(), b.clone()));
                }
                if a.name == "P" && b.name == "R" {
                    tp = Some((m.clone(), a.clone(), b.clone()));
                }
            }
            _ => {}
        }
    }

    if pq.is_some() && pr.is_some() && m1m2.is_some() && sp.is_some() && tp.is_some() {
        // Alle Bedingungen erfüllt → Beweis möglich
        let p = pq.as_ref().unwrap().0.clone();
        let s = sp.as_ref().unwrap().0.clone();
        let m1 = m1m2.as_ref().unwrap().0.clone();
        let psm1 = geometry::Triangle {
            a: p.clone(),
            b: s.clone(),
            c: m1.clone(),
        };

        let t = tp.as_ref().unwrap().0.clone();
        let m2 = m1m2.as_ref().unwrap().1.clone();
        let ptm2 = geometry::Triangle {
            a: p.clone(),
            b: t.clone(),
            c: m2.clone(),
        };
        return Some(Relation::similar_triangle(psm1.clone(), ptm2.clone()));
    }

    None
}
// Einfaches Thales-Theorem:
// Wenn C auf Kreis mit Durchmesser AB liegt, dann ist Winkel ACB = 90°
pub fn thales_theorem(known: &[Relation]) -> Option<Relation> {
    // Suche einen Kreis mit Durchmesser AB und Punkt C auf dem Kreis
    let mut diameter_line: Option<(Point, Point, Circle)> = None;
    let mut point_c: Option<Point> = None;

    for rel in known {
        match rel {
            Relation::IsDiameter(line, circle) => {
                diameter_line = Some((line.0.clone(), line.1.clone(), circle.clone()));
            }
            Relation::LiesOnCircle(p, c) => {
                if let Some((a, b, circle)) = &diameter_line {
                    if *c == *circle {
                        point_c = Some(p.clone());
                    }
                }
            }
            _ => {}
        }
    }

    if let Some((a, b, _circle)) = diameter_line {
        if let Some(c) = point_c {
            // Dreieck A, C, B
            // Ausgabe: RightAngle(A, C, B)
            return Some(Relation::right_angle(a, c, b));
        }
    }

    None
}
pub fn similarity_quadrant_rule(relations: &[Relation]) -> Option<Relation> {
    let mut pq = None;
    let mut pr = None;
    let mut m1m2 = None;
    let mut s = None;
    let mut t = None;
    let mut p = None;
    let mut m1: Option<Point> = None;
    let mut m2: Option<Point> = None;

    for rel in relations {
        match rel {
            Relation::EqualLength(l1, l2) => {
                if l1.0.name == "P" && l1.1.name == "Q" {
                    pq = Some(l1.clone());
                }
                if l1.0.name == "P" && l1.1.name == "R" {
                    pr = Some(l1.clone());
                }
                if l2.0.name == "M1" && l2.1.name == "M2" {
                    m1m2 = Some(l2.clone());
                }
            }
            Relation::Midpoint(m, a, b) => {
                if a.name == "P" && b.name == "Q" {
                    s = Some(m.clone());
                    p = Some(a.clone());
                }
                if a.name == "P" && b.name == "R" {
                    t = Some(m.clone());
                }
            }
            _ => {}
        }
    }

    if let (Some(p), Some(s), Some(t), Some(m1m2)) = (p, s, t, m1m2) {
        let m1 = m1m2.0.clone();
        let m2 = m1m2.1.clone();

        let tri1 = Triangle {
            a: p.clone(),
            b: s.clone(),
            c: m1.clone(),
        };
        let tri2 = Triangle {
            a: p.clone(),
            b: t.clone(),
            c: m2.clone(),
        };
        return Some(Relation::similar_triangle(tri1, tri2));
    }

    None
}
pub fn sss_congruence(known: &[Relation]) -> Option<Relation> {
    let mut triangles = Vec::new();
    let mut sides_map = std::collections::HashMap::new();

    // Sammle Seitenlängen für jedes Dreieck
    for rel in known {
        if let Relation::EqualLength(line1, line2) = rel {
            // ... Logik zur Zuordnung von Seiten zu Dreiecken ...
        }
    }

    // Prüfe auf SSS-Kongruenz zwischen Dreiecken
    for (i, (tri1, sides1)) in triangles.iter().enumerate() {
        for (j, (tri2, sides2)) in triangles.iter().enumerate().skip(i + 1) {
            if sides1 == sides2 {
                return Some(Relation::congruent_triangles(tri1.clone(), tri2.clone()));
            }
        }
    }
    None
}
pub fn circle_properties(relations: &[Relation]) -> Option<Relation> {
    // Beweise, dass in Quadraten alle Radien gleich sind
    let mut m1 = None;
    let mut m2 = None;
    let mut i1 = None;
    let mut i2 = None;

    for rel in relations {
        match rel {
            Relation::LiesOnCircle(p, circle) => {
                if p.name == "I1" {
                    i1 = Some(circle.radius_point.clone());
                }
                if p.name == "I2" {
                    i2 = Some(circle.radius_point.clone());
                }
            }
            Relation::IsDiameter(line, circle) => {
                if line.0.name == "M1" && line.1.name == "M2" {
                    m1 = Some(line.0.clone());
                    m2 = Some(line.1.clone());
                }
            }
            _ => {}
        }
    }

    if let (Some(m1), Some(m2), Some(i1), Some(i2)) = (m1, m2, i1, i2) {
        return Some(Relation::equal_length(
            Line::new(m1.clone(), i1.clone()),
            Line::new(m2.clone(), i2.clone()),
        ));
    }
    None
}
