

#[derive(Debug)]
pub struct StatsCoq {
    props: u64,
    lemmas: u64,
    theorems: u64,
    proofs: u64,
    admitteds: u64,
}

impl StatsCoq {
    pub fn new() -> Self {
        Self {
            props: 0,
            lemmas: 0,
            theorems: 0,
            proofs: 0,
            admitteds: 0,
        }
    }
}
