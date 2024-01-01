pub struct Lexer {
    input: String,
}

impl<'a> Lexer {
    pub fn new(input: &'a str) -> Self {
        let input = input.replace("-\n", "").replace("\n", " ");

        Lexer { input }
    }
}

impl<'a> Iterator for Lexer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let (word, new) = self
            .input
            .split_once(" ")
            .unwrap_or((self.input.as_str(), ""));

        if word == "" {
            return None;
        }

        let word = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        self.input = new.trim().to_owned();

        Some(word.to_lowercase())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_basic_lex() {
        let string = "Hello, world.";
        let tokens = Lexer::new(string).filter(|s| s.ne("")).collect::<Vec<_>>();

        assert_eq!(tokens, vec!["Hello", "world"]);
    }

    #[test]
    fn working_with_numbers() {
        let string = "1234, alfa12 CR7 Ho-18";

        let tokens = Lexer::new(string).filter(|s| s.ne("")).collect::<Vec<_>>();

        assert_eq!(tokens, vec!["alfa", "CR", "Ho"]);
    }

    #[test]
    fn multiple_paragraphs() {
        let input = r#"Este es un parrafo.
Este es otro parra-
fo, que es acortado."#;

        let tokens = Lexer::new(input).filter(|s| s.ne("")).collect::<Vec<_>>();

        assert_eq!(
            tokens,
            vec![
                "Este", "es", "un", "parrafo", "Este", "es", "otro", "parrafo", "que", "es",
                "acortado",
            ]
        );
    }
}
