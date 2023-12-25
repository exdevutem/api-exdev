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
        // TODO: Reemplazar el map de abajo por un Lexer
        .map(|w| w.chars().filter(|c| c.is_alphanumeric()).collect())
        .for_each(|word: String| {
            let counter = terms.get(&word).unwrap_or(&0) + 1;
            terms.insert(String::from(word), counter);
        });

    terms
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Term {
    frecuency: u32,
    word: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let map = process_buf(pdf::read_file(&args.path)?);

    let mut top = map
        .iter()
        .map(|(k, v)| Term {
            word: k.clone(),
            frecuency: *v,
        })
        .collect::<Vec<Term>>();

    top.sort_unstable();
    top.reverse();

    top.iter().take(10).for_each(|t| println!("{t:?}"));

    Ok(())
}
