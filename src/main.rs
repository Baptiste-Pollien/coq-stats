/// External modules
#[macro_use] extern crate prettytable;
use std::{env, fs};

/// Modules 
mod file_analysis;
pub mod lexer;
mod table_info;

use crate::file_analysis::{
                file_analysis::{analyse_file, analyse_folder},
                is_folder,
                stats_file::StatsFile,
};

fn analyse_files_in_folder(path_folder: &String) {
    let mut table_info = table_info::new_tab_stats_file();
    let mut table_coq_info = table_info::new_tab_coq_information();

    let mut total_stats = StatsFile::new(&String::from("TOTAL"));

    let paths = fs::read_dir(path_folder)
        .unwrap_or_else(|error| {
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
        }
        else {
            if let Some(stats) = analyse_file(&path) {
                stats.table_info(&mut table_info);
                stats.coq_stats.table_info(&path, &mut table_coq_info);
                total_stats += stats;
            }
        }
    }
    total_stats.table_info(&mut table_info);
    total_stats.coq_stats.table_info(&String::from("TOTAL"), &mut table_coq_info);

    table_info.printstd();
    table_coq_info.printstd();
}

fn analyse_one_file(path: String) {
    let stats = analyse_file(&path).unwrap();

    let mut table_info = table_info::new_tab_stats_file();
    stats.table_info(&mut table_info);
    table_info.printstd();

    let mut table_coq_info = table_info::new_tab_coq_information();
    stats.coq_stats.table_info(&path, &mut table_coq_info);
    table_coq_info.printstd();
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = String::from(&args[1]);

    if is_folder(&file_path) {
        analyse_files_in_folder(&file_path);
    } else {
        analyse_one_file(file_path);
    };
}
