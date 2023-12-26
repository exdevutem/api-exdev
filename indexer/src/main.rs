//! Indexador de documentos para plataforma estudiantil, utilizando la estrategia TF-IDF

mod lexer;
mod pdf;
use std::collections::HashMap;

use clap::Parser;
use lexer::Lexer;

#[derive(Parser, Debug)]
struct Args {
    path: std::path::PathBuf,
}

fn process_buf(buf: String) -> HashMap<String, u32> {
    let mut terms = HashMap::<String, u32>::new();

    Lexer(&buf).for_each(|word| {
        let counter = terms.get(&String::from(word)).unwrap_or(&0) + 1;
        terms.insert(word.to_owned(), counter);
    });

    println!("{:?}", Lexer(&buf).collect::<Vec<_>>());

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

    println!("-------- Hottest words -----------");
    top.iter()
        .take(10)
        .for_each(|t| println!("{t:?} - {:?}", t.word.bytes()));

    Ok(())
}
