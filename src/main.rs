use clap::Parser;

use polars::prelude::*;

#[derive(Parser)]
struct Cli {
    /// The input parquet file that will be converted (.parquet)
    input_file: std::path::PathBuf,

    /// An output path for the file (.csv)
    output_file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut in_file = std::fs::File::open(args.input_file).unwrap();

    let mut df = ParquetReader::new(&mut in_file).finish().unwrap();
    println!("Data Schema & Types");
    println!("{:#?}", df.schema());
    let mut out_file = std::fs::File::create(&args.output_file).unwrap();
    CsvWriter::new(&mut out_file).finish(&mut df).unwrap();
    println!("Successfully written to {:#?}", &args.output_file);
}
