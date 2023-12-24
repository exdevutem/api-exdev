//! Indexador de documentos para plataforma estudiantil, utilizando la estrategia TF-IDF

use clap::Parser;

use pdf::content::Op;

fn op_to_string(op: &Op) -> String {
    match op {
        Op::TextDraw { text } => {
            format!("{datos} ", datos = text.to_string_lossy())
        }
        Op::TextDrawAdjusted { array } => {
            let text = array.iter().fold(String::new(), |mut acc, txt| {
                use pdf::content::TextDrawAdjusted::Text;

                if let Text(texto) = txt {
                    let texto = texto.to_string_lossy();

                    acc.push_str(texto.as_str());
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

    let file = pdf::file::FileOptions::cached().open(args.path)?;
    let resolver = file.resolver();

    let text = file
        .pages()
        .filter_map(Result::ok)
        .filter_map(|page| page.contents.clone())
        .map(|content| {
            content
                .operations(&resolver)
                .unwrap()
                .iter()
                .map(|e| -> String { op_to_string(e) })
                .fold(String::new(), |mut acc, s| {
                    acc.push_str(s.as_str());
                    acc
                })
        })
        .collect::<String>();

    println!("{text}");

    Ok(())
}
