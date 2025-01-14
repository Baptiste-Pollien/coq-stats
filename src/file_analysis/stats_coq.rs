use std::ops::Add;

use prettytable::{row, Table};

#[derive(Debug, PartialEq)]
pub struct StatsCoq {
    pub line_code: u64,
    pub line_proposition: u64,
    pub line_proof: u64,
    pub nb_lemma: u64,
    pub nb_theorem: u64,
    pub nb_proof: u64,
    pub nb_admitted: u64,
}

impl Default for StatsCoq {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsCoq {
    pub fn new() -> Self {
        Self {
            line_code: 0,
            line_proposition: 0,
            line_proof: 0,
            nb_lemma: 0,
            nb_theorem: 0,
            nb_proof: 0,
            nb_admitted: 0,
        }
    }

    pub fn new_test(
        line_code: u64,
        line_proposition: u64,
        line_proof: u64,
        nb_lemma: u64,
        nb_theorem: u64,
        nb_proof: u64,
        nb_admitted: u64,
    ) -> Self {
        Self {
            line_code,
            line_proposition,
            line_proof,
            nb_lemma,
            nb_theorem,
            nb_proof,
            nb_admitted,
        }
    }

    // Add the information in the table
    pub fn table_info(&self, path: &String, table: &mut Table) {
        table.add_row(row![
            path,
            self.line_code,
            self.line_proposition,
            self.line_proof,
            self.nb_lemma,
            self.nb_theorem,
            self.nb_proof,
            self.nb_admitted
        ]);
    }
}

impl<'b> Add<&'b StatsCoq> for &StatsCoq {
    type Output = StatsCoq;

    fn add(self, other: &'b StatsCoq) -> StatsCoq {
        StatsCoq {
            line_code: self.line_code + other.line_code,
            line_proposition: self.line_proposition + other.line_proposition,
            line_proof: self.line_proof + other.line_proof,
            nb_lemma: self.nb_lemma + other.nb_lemma,
            nb_theorem: self.nb_theorem + other.nb_theorem,
            nb_proof: self.nb_proof + other.nb_proof,
            nb_admitted: self.nb_admitted + other.nb_admitted,
        }
    }
}
