mod file_analysis;
pub mod lexer;

use crate::file_analysis::file_analysis::analyse_file;
use crate::file_analysis::file_analysis::analyse_folder;
use crate::file_analysis::is_folder;
use std::env;

//TODO
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use prettytable::format;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = String::from(&args[1]);

    let stats = if is_folder(&file_path) {
        Some(analyse_folder(file_path))
    } else {
        analyse_file(file_path)
    };
    let stats = stats.unwrap();

    // Test pretty table

    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(&[format::LinePosition::Top,
                    format::LinePosition::Bottom],
                    format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Path", "Lines", "Code", "Comments", "Blanks"]);
    table.add_row(row![stats.name, stats.lines, stats.code(), stats.comments, stats.comments]);

    table.printstd();

    // Coq infos

    let stats = stats.coq_stats;

    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(&[format::LinePosition::Top,
                    format::LinePosition::Bottom],
                    format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Code", "Line Proof", "Lemma", "Theorem", "Proof", "Admitted"]);
    table.add_row(row![stats.line_code, stats.line_proof, stats.nb_lemma, stats.nb_theorem, stats.nb_proof, stats.nb_admitted]);

    table.printstd();

    println!("{:?}", stats);

}
