#[derive(Debug)]
pub struct Lexer<'a>(pub &'a str);

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0.trim();

        let chars = self.0.chars().collect::<Vec<_>>();

        match chars.first() {
            None => None,
            Some(c) if c.is_alphabetic() => {
                let mut n = 0;
                while chars.get(n).is_some() && chars.get(n).unwrap().is_alphanumeric() {
                    n += 1;
                }

                let res = &self.0[0..n];
                self.0 = &self.0[n..];

                Some(res)
            }
            Some(c) if c.is_numeric() => {
                let mut n = 0;
                while chars.get(n).is_some() && chars.get(n).unwrap().is_numeric() {
                    n += 1;
                }

                let res = &self.0[0..n];
                self.0 = &self.0[n..];

                Some(res)
            }
            Some(_) => {
                let Some(n) = self.0.char_indices().map(|(i, _)| i).nth(1) else {
                    // Este es un edge case. En caso de que el ultimo caracter sea un caracter no
                    // alfanumerico, retorno lo que queda de string nomas.
                    let res = self.0;
                    self.0 = "";
                    return Some(res);
                };

                let res = &self.0[0..n];
                self.0 = &self.0[n..];

                Some(res)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_basic_lex() {
        let string = "Hello, world.";
        let tokens = Lexer(string).collect::<Vec<_>>();

        assert_eq!(tokens, vec!["Hello", ",", "world", "."]);
    }

    #[test]
    fn working_with_numbers() {
        let string = "1234, alfa12 CR7 Ho-18";

        let tokens = Lexer(string).collect::<Vec<_>>();

        assert_eq!(tokens, vec!["1234", ",", "alfa12", "CR7", "Ho", "-", "18"]);
    }
}
