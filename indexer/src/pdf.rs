use std::path::PathBuf;

use anyhow::{anyhow, Result};

pub fn read_file(path: &PathBuf) -> Result<String> {
    let file = std::fs::read(path)?;

    let file = poppler::Document::from_data(file.as_slice(), None).unwrap();

    let mut out = Vec::<String>::new();

    for index in 0..file.n_pages() {
        out.push(
            file.page(index)
                .ok_or(anyhow!("Couldn't retrieve page {index}"))?
                .text()
                .ok_or(anyhow!("Couldn't retrieve text from page {index}"))?
                .to_string(),
        )
    }

    Ok(out.join(" "))
}
