//! Indexador de documentos para plataforma estudiantil, utilizando la estrategia TF-IDF

mod lexer;
mod pdf;
use std::{collections::HashMap, path::PathBuf};

use anyhow::anyhow;
use clap::Parser;
use lexer::Lexer;

#[derive(Parser, Debug)]
struct Args {
    path: std::path::PathBuf,
}

fn process_buf(buf: String) -> Index {
    let mut terms = Index::new();

    Lexer::new(&buf).for_each(|word| {
        let counter = terms.get(&String::from(&word)).unwrap_or(&0) + 1;
        terms.insert(word.to_owned(), counter);
    });

    terms
}

fn read_file(p: &PathBuf) -> anyhow::Result<String> {
    if let Some(ext) = p.extension() {
        match ext.to_str() {
            Some("pdf") => pdf::read_file(p),
            _ => unimplemented!(),
        }
    } else {
        Err(anyhow!("Couldn't get file extension of {p:?}"))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Term {
    frecuency: usize,
    word: String,
}

type Index = HashMap<String, usize>;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut indexes = HashMap::new();

    match args.path {
        p if p.is_dir() => {
            std::fs::read_dir(p)?
                .filter_map(Result::ok)
                .for_each(|file| {
                    if let Ok(content) = read_file(&file.path()) {
                        let map = process_buf(content);

                        indexes.insert(file.path(), map);
                    }
                });
        }
        p if p.is_file() => {
            if let Ok(content) = read_file(&p) {
                let map = process_buf(content);

                indexes.insert(p, map);
            }
        }
        _ => unreachable!(),
    }

    let mut idf = indexes.iter().fold(HashMap::new(), |mut acc, (_, e)| {
        for (term, _) in e.iter() {
            let found = acc.get(term).unwrap_or(&0.0) + 1.0;
            acc.insert(term.clone(), found);
        }
        acc
    });

    let docs = indexes.len();

    println!("{idf:?}");
    println!("'Linux' term: {linux:?}", linux = idf.get("linux"));

    for (_, i) in idf.iter_mut() {
        *i = (docs as f64 / *i as f64).ln();
    }

    println!("{idf:?}");
    println!("'Linux' term: {linux:?}", linux = idf.get("linux"));

    Ok(())
}
