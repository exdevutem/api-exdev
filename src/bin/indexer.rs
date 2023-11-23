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
                match txt {
                    Text(texto) => {
                        let texto = texto
                            .as_bytes()
                            .iter()
                            .map(|&c| c as char)
                            .collect::<String>();

                        acc.push_str(texto.as_str());
                    }
                    _ => (),
                }
                acc
            });

            text
        }
        _ => String::new(),
    }
}

fn main() -> anyhow::Result<()> {
    let input = String::from("assets/Trabajo asalariado y Capital.pdf");

    let pdf = pdf::file::FileOptions::cached().open(input)?;

    for (i, page) in pdf.pages().enumerate() {
        println!(
            r#" 
        -------------------------------------------
        Página número {i}
        -------------------------------------------
        "#
        );

        let page = page?;
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
