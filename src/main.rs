mod file_analysis;
pub mod lexer;

use crate::file_analysis::file_analysis::read_file;


fn main() {
    let nom_fichier = String::from("test.v");
    println!("Dans le fichier : {}", nom_fichier);

    let stats = read_file(nom_fichier);

    println!("{}", stats);

}
