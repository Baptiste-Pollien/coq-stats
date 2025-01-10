pub mod analyser;
pub mod stats_coq;
pub mod stats_file;

pub mod file_analysis {
    use crate::{
        file_analysis::{analyser::Analyser, stats_file::StatsFile},
        lexer,
        lexer::token::Token,
        system::system::{is_coq_file, is_folder},
        table_info::table_info,
    };
    use std::fs;

    /// Produce the statistic about the file [file_name]
    pub fn analyse_file(file_name: &String) -> Option<StatsFile> {
        // Test if its a coq file
        if !is_coq_file(&file_name) {
            return None;
        }

        let mut stats = StatsFile::new(&file_name);

        // Count the first line
        stats.lines += 1;

        let contenu = fs::read_to_string(&file_name).unwrap_or_else(|error| {
            eprintln!("Error: Cannot read the file {}: {}", file_name, error);
            std::process::exit(1);
        });

        let mut l = lexer::Lexer::new(contenu.chars().collect());

        let mut analyser = Analyser::new();

        l.read_char();

        // First loop to analyse lines
        loop {
            match l.next_token() {
                Token::EOL => {
                    stats.blanks += 1;
                }
                Token::EOF => {
                    stats.blanks += 1;
                    break;
                }
                Token::END => {
                    break;
                }
                token => {
                    analyser.analyse_line(&mut l, &mut stats, token);
                }
            }
            if !l.end() {
                stats.lines += 1;
            }
        }

        Some(stats)
    }

    /// Function to analyse recursively all the files in a folder
    fn analyse_folder(path_folder: &String) -> StatsFile {
        let mut stats_folder = StatsFile::new(&path_folder);

        let paths = fs::read_dir(path_folder).unwrap_or_else(|error| {
            eprintln!("Error: Cannot read the file: {}", error);
            std::process::exit(1);
        });

        for path in paths {
            let path = path.unwrap().path().display().to_string();

            if is_folder(&path) {
                stats_folder += analyse_folder(&path);
            } else {
                if let Some(stats) = analyse_file(&path) {
                    stats_folder += stats;
                }
            }
        }

        stats_folder
    }

    /// Analyse a folder and display the info of each files on a line
    pub fn analyse_files_in_folder(path_folder: &String) {
        let mut table_info = table_info::new_tab_stats_file();
        let mut table_coq_info = table_info::new_tab_coq_information();

        let mut total_stats = StatsFile::new(&String::from("TOTAL"));

        let paths = fs::read_dir(path_folder).unwrap_or_else(|error| {
            eprintln!("Error: Cannot read the file: {}", error);
            std::process::exit(1);
        });

        for path in paths {
            let path = path.unwrap().path().display().to_string();

            if is_folder(&path) {
                let stats = analyse_folder(&path);
                stats.table_info(&mut table_info);
                stats.coq_stats.table_info(&path, &mut table_coq_info);
                total_stats += stats;
            } else {
                if let Some(stats) = analyse_file(&path) {
                    stats.table_info(&mut table_info);
                    stats.coq_stats.table_info(&path, &mut table_coq_info);
                    total_stats += stats;
                }
            }
        }
        total_stats.table_info(&mut table_info);
        total_stats
            .coq_stats
            .table_info(&String::from("TOTAL"), &mut table_coq_info);

        table_info.printstd();
        table_coq_info.printstd();
    }

    /// Analysis of a file
    pub fn analyse_one_file(path: String) {
        let stats = analyse_file(&path).unwrap();

        let mut table_info = table_info::new_tab_stats_file();
        stats.table_info(&mut table_info);
        table_info.printstd();

        let mut table_coq_info = table_info::new_tab_coq_information();
        stats.coq_stats.table_info(&path, &mut table_coq_info);
        table_coq_info.printstd();
    }
}
