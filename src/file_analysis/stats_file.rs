use crate::file_analysis::stats_coq::StatsCoq;
use std::ops::Add;
use std::ops::AddAssign;

use prettytable::{Table, row};

#[derive(Debug, PartialEq)]
pub struct StatsFile {
    pub name: String,
    pub lines: u64,
    pub blanks: u64,
    pub comments: u64,
    pub coq_stats: StatsCoq,
}

impl StatsFile {
    pub fn new(file_name: &String) -> Self {
        Self {
            name: String::from(file_name),
            lines: 0,
            blanks: 0,
            comments: 0,
            coq_stats: StatsCoq::new(),
        }
    }

    pub fn code(&self) -> u64 {
        self.coq_stats.line_code
            + self.coq_stats.line_proposition
            + self.coq_stats.line_proof
    }

    pub fn new_tests(file_name: &str, 
                           lines: u64,
                           blanks: u64,
                           comments: u64,
                           coq_stats: StatsCoq) -> Self{
        Self {
            name: String::from(file_name),
            lines: lines,
            blanks: blanks,
            comments: comments,
            coq_stats: coq_stats,
        }
    }

    pub fn table_info(&self, table: &mut Table) {
        table.add_row(row![self.name,
                                  self.lines,
                                 self.code(),
                                 self.comments,
                                 self.comments]);
    }
}

impl<'a, 'b> Add<&'b StatsFile> for &'a StatsFile {
    type Output = StatsFile;

    fn add(self, other: &'b StatsFile) -> StatsFile {
        StatsFile {
            name: String::from(&self.name),
            lines: self.lines + other.lines,
            blanks: self.blanks + other.blanks,
            comments: self.comments + other.comments,
            coq_stats: &self.coq_stats + &other.coq_stats,
        }
    }
}

impl AddAssign for StatsFile {
    fn add_assign(&mut self, other: StatsFile) {
        *self = StatsFile {
            name: String::from(&self.name),
            lines: self.lines + other.lines,
            blanks: self.blanks + other.blanks,
            comments: self.comments + other.comments,
            coq_stats: &self.coq_stats + &other.coq_stats,
        }
    }
}
