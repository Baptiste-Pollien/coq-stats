use clap::Parser;
/// External modules
use std::{path::PathBuf, process::exit};

pub mod file_analysis;
mod lexer;
mod system;
/// Modules
mod table_info;

use crate::file_analysis::file_analysis::{analyse_files_in_folder, analyse_one_file};

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
        let file_path = path.into_os_string().into_string().unwrap_or_else(|e| {
            println!("Error when getting path: {:?}", e);
            exit(0);
        });

        if is_folder(&file_path) {
            analyse_files_in_folder(&file_path);
        } else {
            analyse_one_file(file_path);
        };
    }
}
