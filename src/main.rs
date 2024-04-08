use clap::Parser;
use bio::io::fasta;
use regex::Regex;
use std::{fs::File, io::{self, Write, BufReader, BufWriter}};
use flate2::read::MultiGzDecoder;

/// Finds specified motifs in a FASTA file and outputs them in BED format with additional details.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input FASTA file (can be uncompressed or gzipped)
    #[clap(short, long)]
    fasta: String,

    /// Motif to search for
    #[clap(short, long)]
    motif: String,

    /// Output BED file. Writes to stdout if not specified
    #[clap(short, long)]
    bed: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let fasta_path = &args.fasta;
    let motif = &args.motif;
    let reverse_complement = reverse_complement(motif);

    let is_gzipped = fasta_path.ends_with(".gz");
    let reader_box: Box<dyn std::io::Read> = if is_gzipped {
        Box::new(MultiGzDecoder::new(File::open(fasta_path)?))
    } else {
        Box::new(File::open(fasta_path)?)
    };

    let reader = fasta::Reader::new(BufReader::new(reader_box));
    let mut writer: Box<dyn Write> = match args.bed {
        Some(ref path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(io::stdout())),
    };

    let motif_regex = Regex::new(&regex_pattern(motif)).unwrap();
    let reverse_regex = Regex::new(&regex_pattern(&reverse_complement)).unwrap();

    for result in reader.records() {
        let record = result.expect("Error reading a record");
        let seq = String::from_utf8(record.seq().to_vec()).unwrap();

        // Search for and write matches of the original motif
        for mat in motif_regex.find_iter(&seq) {
            let matched_sequence = &seq[mat.start()..mat.end()];
            writeln!(writer, "{}\t{}\t{}\t{}\t.\t+\t{}", record.id(), mat.start(), mat.end(), motif, matched_sequence)?;
        }

        // Search for and write matches of the reverse complement
        for mat in reverse_regex.find_iter(&seq) {
            let matched_sequence = &seq[mat.start()..mat.end()];
            writeln!(writer, "{}\t{}\t{}\t{}\t.\t-\t{}", record.id(), mat.start(), mat.end(), &reverse_complement, matched_sequence)?;
        }
    }

    Ok(())
}

fn reverse_complement(seq: &str) -> String {
    seq.chars().rev().map(|c| match c {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        'N' => 'N',
        _ => panic!("Invalid character in sequence"),
    }).collect()
}

fn regex_pattern(motif: &str) -> String {
    motif.replace("N", ".")
}
