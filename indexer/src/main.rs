//! Indexador de documentos para plataforma estudiantil, utilizando la estrategia TF-IDF

mod lexer;
mod pdf;
use std::{collections::HashMap, path::PathBuf};

use anyhow::anyhow;
use clap::Parser;
use lexer::Lexer;

#[derive(Parser, Debug)]
struct Args {
    #[arg(help = "Archivo o directorio por el cual buscar.")]
    path: std::path::PathBuf,

    #[arg(short, long, help = "Termino por el que hacer la busqueda.")]
    term: Option<String>,
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
            _ => Err(anyhow!("Not implemented or invalid file")),
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

    let docs = indexes.len();
    let mut idf = indexes.iter().fold(HashMap::new(), |mut acc, (_, e)| {
        for (term, _) in e.iter() {
            let found = acc.get(term).unwrap_or(&0.0) + 1.0;
            acc.insert(term.clone(), found);
        }
        acc
    });

    for (_, i) in idf.iter_mut() {
        *i = (docs as f64 / *i as f64).ln();
    }

    // Si se introdujo un termino de busca, imprimo
    // cuales son los documentos mas relevantes.
    if let Some(search) = args.term {
        let search = search.to_lowercase();
        indexes.iter().for_each(|(path, index)| {
            println!(
                "{path:?} - {}",
                *index.get(&search).unwrap_or(&0) as f64 * idf.get(&search).unwrap_or(&0.0)
            );
        });
    }

    Ok(())
}
