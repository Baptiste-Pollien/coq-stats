pub mod stats_file;
pub mod stats_coq;
pub mod analyser;

pub mod file_analysis {
    use std::fs;
    use crate::lexer;
    use crate::lexer::token::Token;
    use crate::file_analysis::stats_file::StatsFile;
    use crate::file_analysis::analyser::Analyser;

    /// Produce the statistic about the file [file_name]
    pub fn analyse_file(file_name: String) -> StatsFile {
        let mut stats = StatsFile::new(&file_name);

        // Count the first line
        stats.lines += 1;

        let contenu = fs::read_to_string(file_name)
            .expect("Cannot read the file");

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

        stats
    }

    pub fn analyse_folder(path_folder: String) -> StatsFile {
        let mut stats_folder  = StatsFile::new(&path_folder);

        let paths = fs::read_dir(path_folder).unwrap();

        for path in paths {
            let path = path.unwrap().path().display().to_string();
            stats_folder += analyse_file(path.to_string());
        }

        stats_folder
    }
}
