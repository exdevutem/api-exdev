//! Indexador de documentos para plataforma estudiantil, utilizando la estrategia TF-IDF

mod pdf;
use std::collections::HashMap;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    path: std::path::PathBuf,
}

fn process_buf(buf: String) -> HashMap<String, u32> {
    let mut terms = HashMap::<String, u32>::new();

    terms
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("{}", pdf::read_file(&args.path)?);

    Ok(())
}
