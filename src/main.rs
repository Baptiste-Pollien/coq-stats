/// External modules
use std::{path::PathBuf};
use clap::Parser;

/// Modules 
mod table_info;
mod system;
mod lexer;
pub mod file_analysis;

use crate::file_analysis::{
    file_analysis::{analyse_files_in_folder, analyse_one_file},
};

use crate::system::system::is_folder;


/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Output file
    /// #[clap(short, long, parse(from_os_str))]
    /// output: Option<PathBuf>,

    /// Files to process
    #[clap(name = "FILE", parse(from_os_str), required = true)]
    files: Vec<PathBuf>,
}

fn main() { 
    let args = Args::parse();

    for path in args.files {
        let file_path  = path.into_os_string()
                                          .into_string()
                                          .unwrap();

        if is_folder(&file_path) {
            analyse_files_in_folder(&file_path);
        } else {
            analyse_one_file(file_path);
        };
    }
}
