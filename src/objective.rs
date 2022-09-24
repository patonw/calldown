use std::collections::HashMap;

pub trait Objective {
    fn loss(&self, solution: &[&str]) -> f32;
}

pub struct GeneralAlignmentObjective {
    generals: Vec<String>,
}

impl GeneralAlignmentObjective {
    pub fn new(generals: Vec<String>) -> Self {
        Self {
            generals
        }
    }
}

impl Objective for GeneralAlignmentObjective {
    fn loss(&self, solution: &[&str]) -> f32 {
        let mut result = 0.0f32;
        for (a,b) in self.generals.iter().zip(solution) {
            if a.eq(b) {
                result -= 10.0
            }
        }
        result
    }
}

pub struct ExclusionObjective {
    exclusions: Vec<Vec<String>>,
}

impl ExclusionObjective {
    pub fn new(exclusions: Vec<Vec<String>>) -> Self {
        Self {
            exclusions
        }
    }
}

impl Objective for ExclusionObjective {
    fn loss(&self, solution: &[&str]) -> f32 {
        let mut result = 0.0;

        for (a,b) in self.exclusions.iter().zip(solution) {
            if a.contains(&b.to_string()) {
                result += 1000.0
            }
        }

        result
    }
}

pub struct SpacingObjective;

impl Default for SpacingObjective {
    fn default() -> Self {
        Self
    }
}

impl SpacingObjective {
    pub fn new() -> Self {
        Self
    }

    fn loss_func(&self, delta: usize) -> f32 {
        let denom = (delta as f32).exp2();
        100.0 / denom
    }
}

impl Objective for SpacingObjective {
    fn loss(&self, solution: &[&str]) -> f32 {
        let mut result = 0.0f32;
        let mut cache = HashMap::<String, usize>::new();
        for (i, &v) in solution.iter().enumerate() {
            if let Some(prev) = cache.insert(v.to_string(), i) {
                result += self.loss_func(i - prev);
            }
        }

        result
    }

}

pub struct VectorObjective(pub Vec<Box<dyn Objective>>);

impl Objective for VectorObjective {
    fn loss(&self, solution: &[&str]) -> f32 {
        self.0.iter()
            .map(|obj| obj.loss(solution))
            .sum()
    }
}
