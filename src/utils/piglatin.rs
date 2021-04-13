pub fn piglatin(string: &str) -> String {
    let mut chars = string.chars();
    let mut new_string = String::new();
    let char = chars.next();
    if char.is_some() {
        let suffix = match char.unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                new_string.push(char.unwrap());
                String::from("-hay")
            }
            'a' ..= 'z' | 'A' ..= 'Z' => {
                format!("-{}ay", char.unwrap())
            }
            _ => {
                new_string.push(char.unwrap());
                String::from("")
            }
        };

        while let Some(c) = chars.next() {
            new_string.push(c)
        }
        new_string += &suffix;
    }
    new_string
}

#[cfg(test)]
mod piglatin_test {
    use super::*;

    #[test]
    fn an_empty_string_should_not_be_a_problem() {
        let piglatin = piglatin("");
        assert_eq!(piglatin, "");
    }

    #[test]
    fn consonant_string_should_be_converted_to_pig_latin() {
        let piglatin = piglatin("filana");
        assert_eq!(piglatin, "ilana-fay");
    }

    #[test]
    fn vocal_starting_string_should_be_converted_to_pig_latin() {
        let piglatin1 = piglatin("edinburgh");
        assert_eq!(piglatin1, "edinburgh-hay");
        let piglatin2 = piglatin("eनdinburgh");
        assert_eq!(piglatin2, "eनdinburgh-hay");
    }

    #[test]
    fn non_letter_starting_string_should_not_be_converted_to_pig_latin() {
        let piglatin = piglatin("नdinburgh");
        assert_eq!(piglatin, "नdinburgh");
    }
}