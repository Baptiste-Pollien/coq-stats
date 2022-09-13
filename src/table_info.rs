use prettytable::{Table, format};

/// Create a table well formated
fn new_tab() -> Table {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(&[
                    format::LinePosition::Bottom],
                    format::LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table
}

/// Create a table for the Stats File
pub fn new_tab_stats_file() -> Table {
    let mut table = new_tab();

    table.set_titles(row!["Path",
                                "Lines",
                                "Code", 
                                "Comments",
                                "Blanks"]);

    table
}

/// Create a table for Coq informations
pub fn new_tab_coq_information() -> Table {
    let mut table = new_tab();

    table.set_titles(row!["Path",
                               "Lines Code",
                               "Lines Lemma/TH",
                               "Lines Proof",
                               "Nb Lemma",
                               "Nb Theorem",
                               "Nb Proof",
                               "Nb Admitted"]);

    table
}