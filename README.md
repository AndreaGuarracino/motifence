# Motifence
`motifence` is a command-line tool designed to find specified motifs in a FASTA file and output them in BED format with additional details.

## Features
- Supports both uncompressed and gzipped FASTA files.
- Allows specifying motifs via command-line arguments or a file.
- Outputs results in BED format, including the matched sequence.

## Installation
To install `motifence`, you need to have Rust and Cargo installed.

Clone the repository:

```shell
git clone https://github.com/yourusername/motifence.git
cd motifence
```
Build the project:

```shell
cargo build --release
```

The executable will be located in `target/release/`. You can add this directory to your `PATH` or move the executable to a directory that is already in your `PATH`.

## Usage
Search for a single motif and write results to stdout:

```shell
motifence --fasta sequences.fasta --motif ATGC
```

Search for motifs listed in a file and write results to a BED file:

```shell
motifence --fasta sequences.fasta --motif_file motifs.txt --bed results.bed
```

Search for a single motif in a gzipped FASTA file and write results to stdout:

```shell
motifence --fasta sequences.fasta.gz --motif ATGC
```

## Output Format
The output is in BED format with the following columns:

- Chromosome/Sequence ID
- Start position (0-based)
- End position (1-based)
- Motif
- Dot (.)
- Strand (+ or -)
 Matched sequence

Example output:

```shell
chr1    100     104     ATGC    .       +       ATGC
chr2    200     204     ATGC    .       -       GCAT
```
