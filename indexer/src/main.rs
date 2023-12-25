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

    buf.split(|c: char| c.is_whitespace())
        .map(|w| w.to_lowercase())
        .for_each(|word| {
            if let Some(counter) = terms.get(&word) {
                terms.insert(String::from(word), counter + 1);
            } else {
                terms.insert(String::from(word), 1);
            }
        });

    terms
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let map = process_buf(pdf::read_file(&args.path)?);

    println!("{map:?}");

    Ok(())
}
