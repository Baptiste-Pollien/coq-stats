# Coq'Stats

Coq'Stats is a program to get statistics about Coq projects.

```[bash]
+------------------+-------+------+----------+--------+
| Path             | Lines | Code | Comments | Blanks |
+------------------+-------+------+----------+--------+
| ./semantics      | 1787  | 1229 | 306      | 252    |
| ./verification   | 3284  | 2207 | 566      | 511    |
| ./common         | 702   | 610  | 14       | 78     |
| ./extraction.v   | 81    | 26   | 37       | 18     |
| ./generator      | 2994  | 2300 | 315      | 379    |
| ./syntax         | 1020  | 680  | 122      | 218    |
| TOTAL            | 9868  | 7052 | 1360     | 1456   |
+------------------+-------+------+----------+--------+
+------------------+------------+----------------+-------------+----------+------------+----------+-------------+
| Path             | Lines Code | Lines Lemma/TH | Lines Proof | Nb Lemma | Nb Theorem | Nb Proof | Nb Admitted |
+------------------+------------+----------------+-------------+----------+------------+----------+-------------+
| ./semantics      | 1008       | 59             | 162         | 11       | 1          | 11       | 3           |
| ./verification   | 273        | 747            | 1397        | 66       | 4          | 64       | 6           |
| ./common         | 80         | 218            | 312         | 45       | 0          | 45       | 0           |
| ./extraction.v   | 26         | 0              | 0           | 0        | 0          | 0        | 0           |
| ./generator      | 1201       | 260            | 839         | 56       | 2          | 71       | 1           |
| ./syntax         | 645        | 14             | 21          | 3        | 0          | 3        | 0           |
| TOTAL            | 3233       | 1298           | 2731        | 181      | 7          | 194      | 10          |
+------------------+------------+----------------+-------------+----------+------------+----------+-------------+

```

## Usage

To run the program:

```[bash]
cargo run [FILE]
```

Where `FILE` is the path to a file or a folder.

**Note**: This program analyse only Coq file. To analyse other type of file
you can use [tokei](https://github.com/XAMPPRocky/tokei).

## Analysis

The Coq'Stats realize an analysis of the project given in parameters and
then display the results of the analysis in the form of 2 tables.

The first table displays generic information:

- The number of lines,
- The number of code lines,
- The number of comment lines,
- The number of blank lines.

Coq'Stats support and detect the nested comments.

The second table displays some information specific to Coq projects:

- The number of code lines (import, definitions, records...),
- The number of lines for the definition of lemmas or theorems,
- The number of proof lines (section starting by `Proof` or `Next Obligation`),
- The number of lemmas,
- The number of theorems,
- The number of proofs (ending by `Qed`),
- The number of admitted (ending by `Admitted`).

The number of lines in the second table does take into account the blank
lines or the commented lines.
