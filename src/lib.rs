extern crate prettytable;
pub mod file_analysis;
mod lexer;
mod system;
mod table_info;

use crate::file_analysis::{file_analysis::analyse_file, stats_file::StatsFile};

/// Function to test the file analysis
pub fn run_test(res: &StatsFile) {
    let file_name = String::from(&res.name);

    if let Some(stats) = analyse_file(&file_name) {
        assert_eq!(stats, *res);
        assert_eq!(stats.lines, stats.blanks + stats.code() + stats.comments);
    } else {
        assert!(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_analysis::stats_coq::StatsCoq;

    #[test]
    fn test0() {
        let cstats = StatsCoq::new();
        let res = StatsFile::new_tests("tests/test0.v", 1, 1, 0, cstats);
        run_test(&res);
    }

    #[test]
    fn test1() {
        let cstats = StatsCoq::new_test(1, 0, 0, 0, 0, 0, 0);
        let res = StatsFile::new_tests("tests/test1.v", 1, 0, 0, cstats);
        run_test(&res);
    }

    #[test]
    fn test2() {
        let cstats = StatsCoq::new_test(5, 2, 3, 1, 0, 1, 0);
        let res = StatsFile::new_tests("tests/test2.v", 20, 5, 5, cstats);
        run_test(&res);
    }

    #[test]
    fn test3() {
        let cstats = StatsCoq::new_test(5, 2, 3, 1, 0, 1, 0);
        let res = StatsFile::new_tests("tests/test3.v", 19, 4, 5, cstats);
        run_test(&res);
    }

    #[test]
    fn test4() {
        let cstats = StatsCoq::new();
        let res = StatsFile::new_tests("tests/test4.v", 2, 2, 0, cstats);
        run_test(&res);
    }

    #[test]
    fn test5() {
        let cstats = StatsCoq::new_test(2, 0, 0, 0, 0, 0, 0);
        let res = StatsFile::new_tests("tests/test5.v", 2, 0, 0, cstats);
        run_test(&res);
    }

    #[test]
    fn test6() {
        let cstats = StatsCoq::new_test(5, 2, 3, 1, 0, 1, 0);
        let res = StatsFile::new_tests("tests/test6.v", 19, 4, 5, cstats);
        run_test(&res);
    }
}
