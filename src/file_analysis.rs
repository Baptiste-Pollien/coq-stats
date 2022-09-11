pub mod stats_file;
pub mod stats_coq;
pub mod analyser;


// TODO; to move
use std::fs::metadata;
use std::path::Path;
use std::ffi::OsStr;

pub fn is_folder(path: &String) -> bool {
    let md = metadata(path).unwrap();
    md.is_dir()
}

pub fn is_coq_file(path: &String) -> bool {
    let ext = Path::new(&path)
                .extension()
                .and_then(OsStr::to_str);

    ext == Some("v")
}

pub mod file_analysis {
    use std::fs;
    use crate::lexer;
    use crate::lexer::token::Token;
    use crate::file_analysis::stats_file::StatsFile;
    use crate::file_analysis::analyser::Analyser;

    use super::{is_folder, is_coq_file};

    /// Produce the statistic about the file [file_name]
    pub fn analyse_file(file_name: &String) -> Option<StatsFile> {
        // Test if its a coq file
        if !is_coq_file(&file_name) {
            return None
        }

        let mut stats = StatsFile::new(&file_name);

        // Count the first line
        stats.lines += 1;

        let contenu = fs::read_to_string(&file_name)
            .unwrap_or_else(|error| {
                eprintln!("Error: Cannot read the file {}: {}", file_name,error);
                std::process::exit(1);
            });

        let mut l
            = lexer::Lexer::new(contenu.chars().collect());

        let mut analyser = Analyser::new();

        l.read_char();

        // First loop to analyse lines
        loop {
            match l.next_token() {
                Token::EOL => {
                    stats.blanks += 1;
                },
                Token::EOF => {
                    stats.blanks += 1;
                    break;
                },
                Token::END => {
                    break;
                },
                token => {
                    analyser.analyse_line(&mut l, &mut stats, token);
                },
            }
            if !l.end() {
                stats.lines += 1;
            }
        }

        Some(stats)
    }

    pub fn analyse_folder(path_folder: &String) -> StatsFile {
        let mut stats_folder  = StatsFile::new(&path_folder);

        let paths = fs::read_dir(path_folder)
            .unwrap_or_else(|error| {
                eprintln!("Error: Cannot read the file: {}", error);
                std::process::exit(1);
            });

        for path in paths {
            let path = path.unwrap().path().display().to_string();

            if is_folder(&path) {
                stats_folder += analyse_folder(&path);
            }
            else {
                if let Some(stats) = analyse_file(&path) {
                stats_folder += stats;
                }
            }
        }

        stats_folder
    }
}
