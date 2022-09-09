mod file_analysis;
pub mod lexer;

use crate::file_analysis::file_analysis::analyse_file;
use std::env;

fn main() {
    // TODO: UPDATE and clean and manage errors, manage project
    // TODO manage display
    let args: Vec<String> = env::args().collect();

    let file_path = String::from(&args[1]);

    let stats = analyse_file(file_path);

    println!("{:?}", stats);

}
