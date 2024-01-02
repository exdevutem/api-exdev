//! Lexeador de palabras naturales.
//!
//! A partir de un string, extrae todas las palabras naturales de este. Se toma como palabra
//! natural cualquier conjunto de caracteres alfabeticos. Debido a esto, nos interesa
//! principalmente filtrar cualquier numero de estos tokens.
//!
//! Se recomienda mirar las pruebas para entender el funcionamiento de este lexer.

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
        // Separo por el siguiente espacio.
        let (word, new) = self
            .input
            .split_once(" ")
            .unwrap_or((self.input.as_str(), ""));

        // El input estaba vacio, no quedan palabras.
        if word == "" {
            return None;
        }

        // Filtro y obtengo las letras.
        let word = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        // Quito espacios que existieran al inicio, por si acaso.
        self.input = new.trim().to_owned();

        // Tokens que solo contienen numeros quedan vacios con el filtro anterior,
        // por lo que retorno la siguiente palabra.
        if word == "" {
            return self.next();
        }

        // Si salio todo bien, retorno el token.
        Some(word.to_lowercase())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_basic_lex() {
        let string = "Hello, world.";
        let tokens = Lexer::new(string).collect::<Vec<_>>();

        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn working_with_numbers() {
        let string = "1234, alfa12 CR7 Ho-18";

        let tokens = Lexer::new(string).collect::<Vec<_>>();

        assert_eq!(tokens, vec!["alfa", "cr", "ho"]);
    }

    #[test]
    fn multiple_paragraphs() {
        let input = r#"Este es un parrafo.
Este es otro parra-
fo, que es acortado."#;

        let tokens = Lexer::new(input).collect::<Vec<_>>();

        assert_eq!(
            tokens,
            vec![
                "este", "es", "un", "parrafo", "este", "es", "otro", "parrafo", "que", "es",
                "acortado",
            ]
        );
    }

    #[test]
    fn none_on_numbers() {
        let input = "11 13 69 420";

        let tokens = Lexer::new(input).collect::<Vec<_>>();

        assert_eq!(tokens, Vec::<String>::new());
    }

    #[test]
    fn splitted_words() {
        let input = "hol-12492835a";

        let tokens = Lexer::new(input).collect::<Vec<_>>();

        assert_eq!(tokens, vec!["hola"]);
    }
}
