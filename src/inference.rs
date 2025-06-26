use crate::relations::Relation;
use crate::theorems::*;

pub fn infer_all(known: &mut Vec<Relation>) {
    let rules: Vec<Box<dyn Fn(&[Relation]) -> Option<Relation>>> = vec![Box::new(thales_theorem())];

    let mut changed = true;
    while changed {
        changed = false;
        for rule in &rules {
            if let Some(new_relation) = rule(known) {
                if !known.contains(&new_relation) {
                    known.push(new_relation);
                    changed = true;
                }
            }
        }
    }
}
