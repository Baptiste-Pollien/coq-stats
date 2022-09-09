
// TODO : add Next Obligations, Inductive, Record...
#[derive(Debug)]
pub struct StatsCoq {
    line_code: u64,
    line_prop: u64,
    line_proof: u64,
    nb_lemma: u64,
    nb_theorem: u64,
    nb_proof: u64,
    nb_admitted: u64,
} 

impl StatsCoq {
    pub fn new() -> Self {
        Self {
            line_code: 0,
            line_prop: 0,
            line_proof: 0,
            nb_lemma: 0,
            nb_theorem: 0,
            nb_proof: 0,
            nb_admitted: 0,
        }
    }
}
