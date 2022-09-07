
pub mod file_analysis {
    use std::fs;
    use std::fmt;
    use crate::lexer;

    struct Proofs {
        nb_lemma: u64,
        nb_theorem: u64,
        nb_proof: u64,
        nb_admitted: u64,
        nb_line_prop: u64,
        nb_line_proof: u64,
    }
    
    pub struct StatsFile {
        name:    String,
        nb_lines: u64,
        nb_blanks: u64,
        nb_code: u64,
        nb_props: u64,
        nb_comments: u64,
    }

    impl fmt::Display for StatsFile {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "File : {}\nNB lines: {}\nNB code: {}\nNB blanks: {}\n",
                self.name, self.nb_lines, self.nb_code, self.nb_blanks)
        }
    }

    pub fn init_file(file_name: &String) -> StatsFile {
        StatsFile {
            name:    String::from(file_name),
            nb_lines: 0,
            nb_blanks: 0,
            nb_code: 0,
            nb_props: 0,
            nb_comments: 0,
        }
    }

    pub fn read_file(file_name: String) -> StatsFile {
        let mut stats = init_file(&file_name);

        let contenu = fs::read_to_string(file_name)
            .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

        let mut l = lexer::Lexer::new(contenu.chars().collect());
        l.read_char();
        loop {
            let token = l.next_token();
            if token == lexer::token::Token::EOF {
                break;
            } else {
                println!("{:?}", token);
            }
        }
    println!("{} {} {}", char::from(l.ch), l.position, l.read_position);

        stats
    }
}
