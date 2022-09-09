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

        let contenu = fs::read_to_string(file_name)
            .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

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
            stats.lines += 1;
        }

        stats
    }
}
