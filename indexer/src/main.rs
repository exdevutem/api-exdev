//! Indexador de documentos para plataforma estudiantil, utilizando la estrategia TF-IDF

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = std::fs::read(args.path).unwrap();

    let file = poppler::Document::from_data(file.as_slice(), None).unwrap();

    for index in 0..file.n_pages() {
        println!(
            "{out}",
            out = file.page(index).unwrap().text().unwrap().to_string()
        );
    }

    Ok(())
}
