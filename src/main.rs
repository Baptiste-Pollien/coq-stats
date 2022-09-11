/// External modules
#[macro_use] extern crate prettytable;
use std::env;

/// Modules 
mod file_analysis;
pub mod lexer;
mod table_info;

use crate::file_analysis::{
                file_analysis::{analyse_file, analyse_folder},
                is_folder
};


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = String::from(&args[1]);

    let stats = if is_folder(&file_path) {
        Some(analyse_folder(file_path))
    } else {
        analyse_file(file_path)
    };
    let stats = stats.unwrap();

    // Display table 
    let mut table_info = table_info::new_tab_stats_file();
    stats.table_info(&mut table_info);
    table_info.printstd();

    let mut table_coq_info = table_info::new_tab_coq_information();
    stats.coq_stats.table_info(&mut table_coq_info);
    table_coq_info.printstd();

}
