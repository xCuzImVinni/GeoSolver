use crate::relations::Relation;

#[derive(Debug, Clone)]
pub struct Proof {
    pub given: Vec<Relation>,
    pub proven: Option<Relation>,
    pub reasoning: Vec<String>,
}

impl Proof {
    pub fn new(given: Vec<Relation>) -> Self {
        Proof {
            given,
            proven: None,
            reasoning: Vec::new(),
        }
    }

    pub fn set_proven(&mut self, relation: Relation) {
        self.proven = Some(relation);
    }

    pub fn add_step(&mut self, step: &str) {
        self.reasoning.push(step.to_string());
    }

    pub fn print(&self) {
        println!("Given:");
        for g in &self.given {
            println!("  {:?}", g);
        }

        println!("\nProof Status:");
        match &self.proven {
            Some(r) => println!("  Proven: {:?}", r),
            None => println!("  No proof found."),
        }

        println!("\nReasoning:");
        for (i, step) in self.reasoning.iter().enumerate() {
            println!("  {}. {}", i + 1, step);
        }
    }
}
