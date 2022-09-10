use std::ops::Add;


#[derive(Debug, PartialEq)]
pub struct StatsCoq {
    pub line_code: u64,
    pub line_proof: u64,
    pub nb_lemma: u64,
    pub nb_theorem: u64,
    pub nb_proof: u64,
    pub nb_admitted: u64,
} 

impl StatsCoq {
    pub fn new() -> Self {
        Self {
            line_code: 0,
            line_proof: 0,
            nb_lemma: 0,
            nb_theorem: 0,
            nb_proof: 0,
            nb_admitted: 0,
        }
    }

    pub fn new_test(line_code: u64, 
                          line_proof: u64,
                          nb_lemma: u64,
                          nb_theorem: u64,
                          nb_proof: u64,
                          nb_admitted: u64,) -> Self {
        Self {
            line_code: line_code,
            line_proof: line_proof,
            nb_lemma: nb_lemma,
            nb_theorem: nb_theorem,
            nb_proof: nb_proof,
            nb_admitted: nb_admitted,
        }
    }
}

impl<'a, 'b> Add<&'b StatsCoq> for &'a StatsCoq {
    type Output = StatsCoq;

    fn add(self, other: &'b StatsCoq) -> StatsCoq {
        StatsCoq {
            line_code: self.line_code + other.line_code,
            line_proof: self.line_proof + other.line_proof,
            nb_lemma: self.nb_lemma + other.nb_lemma,
            nb_theorem: self.nb_theorem + other.nb_theorem,
            nb_proof: self.nb_proof + other.nb_proof,
            nb_admitted: self.nb_admitted + other.nb_admitted,
        }
    }
}
