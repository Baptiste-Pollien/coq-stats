mod file_analysis;
pub mod lexer;

use crate::file_analysis::file_analysis::analyse_file;
use crate::file_analysis::stats_file::StatsFile;

pub fn run_test (res: &StatsFile) {
    let file_name = String::from(&res.name);

    let stats = analyse_file(file_name);

    assert_eq!(stats.lines, stats.blanks + stats.code + stats.comments);
    assert_eq!(stats.lines, res.lines);
    assert_eq!(stats.blanks, res.blanks);
    assert_eq!(stats.code, res.code);
    assert_eq!(stats.comments, res.comments);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_analysis::stats_coq::StatsCoq;

    #[test]
    fn test0 () {
        let cstats =  StatsCoq::new();
        let res 
            = StatsFile::new_tests("tests/test0.v", 1, 1, 0, 0, cstats);
        run_test(&res);
    }

    #[test]
    fn test1 () {
        let cstats =  StatsCoq::new();
        let res 
            = StatsFile::new_tests("tests/test1.v", 1, 0, 1, 0, cstats);
        run_test(&res);
    }


    #[test]
    fn test2 () {
        let cstats =  StatsCoq::new();
        let res 
            = StatsFile::new_tests("tests/test2.v", 20, 5, 10, 5, cstats);
        run_test(&res);
    }

    #[test]
    fn test3 () {
        let cstats =  StatsCoq::new();
        let res 
            = StatsFile::new_tests("tests/test3.v", 19, 4, 10, 5, cstats);
        run_test(&res);
    }

    #[test]
    fn test4 () {
        let cstats =  StatsCoq::new();
        let res 
            = StatsFile::new_tests("tests/test4.v", 2, 2, 0, 0, cstats);
        run_test(&res);
    }

    #[test]
    fn test5 () {
        let cstats =  StatsCoq::new();
        let res 
            = StatsFile::new_tests("tests/test5.v", 2, 0, 2, 0, cstats);
        run_test(&res);
    }

}
