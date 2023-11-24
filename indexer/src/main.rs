//! Indexador de documentos para plataforma estudiantil, utilizando la estrategia TF-IDF

use clap::Parser;
use std::ops::Deref;

use pdf::content::Op;

fn op_to_string(op: &Op) -> String {
    match op {
        Op::TextDraw { text } => {
            let datos = text
                .as_bytes()
                .iter()
                .map(|&c| c as char)
                .collect::<String>();

            datos
        }
        Op::TextDrawAdjusted { array } => {
            let text = array.iter().fold(String::new(), |mut acc, txt| {
                use pdf::content::TextDrawAdjusted::Text;
                if let Text(texto) = txt {
                    let texto = texto
                        .as_bytes()
                        .iter()
                        .map(|&c| c as char)
                        .collect::<String>();

                    acc.push_str(texto.as_str());
                } else {
                    acc.push_str(" ")
                }

                acc
            });

            text
        }
        _ => String::new(),
    }
}

#[derive(Parser, Debug)]
struct Args {
    path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let pdf = pdf::file::FileOptions::cached().open(args.path)?;

    for page in pdf.pages().filter_map(Result::ok) {
        let page = page.deref();

        let content = match &page.contents {
            Some(c) => c,
            None => continue,
        };

        let parrafo = content
            .operations(&pdf)?
            .iter()
            .map(|e| -> String { op_to_string(e) })
            .fold(String::new(), |mut acc, s| {
                acc.push_str(s.as_str());
                acc
            });

        println!("{parrafo}");
    }

    Ok(())
}
