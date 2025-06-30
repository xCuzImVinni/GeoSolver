use crate::relations::Relation;
use crate::theorems::*;

pub fn infer_all(known: &mut Vec<Relation>) {
    let rules: Vec<fn(&[Relation]) -> Option<Relation>> = vec![
        circle_properties,
        prove_congruence,
        prove_similarity,
        thales_theorem,
        similarity_quadrant_rule,
    ];

    let mut changed = true;
    while changed {
        changed = false;
        for rule in &rules {
            if let Some(new_relation) = rule(known) {
                if !known.contains(&new_relation) {
                    println!("Neue Relation: {:?}", new_relation);
                    known.push(new_relation);
                    changed = true;
                }
            }
        }
    }
}
