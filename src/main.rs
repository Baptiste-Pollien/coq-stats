mod file_analysis;
pub mod lexer;

use crate::file_analysis::file_analysis::analyse_file;


fn main() {
    let nom_fichier = String::from("tests/test4.v");
    println!("Dans le fichier : {}", nom_fichier);

    let stats = analyse_file(nom_fichier);

    println!("{:?}", stats);

}
