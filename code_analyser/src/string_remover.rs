enum ParserState {
    Normal,
    String,
    StringBackslash,
    Char,
    CharBackslash,
}

/// Remove string and char content and replace them with "" and '' respectively.
///
/// It greatly simplifies the future parsing since strings can contain special characters.
///
/// It is assumed that the user already called `remove_comments` function.

fn remove_strings(file_content: &str) -> String {
    let mut result: String = Default::default();
    let mut state: ParserState = ParserState::Normal;
    for ch in file_content.bytes() {
        match state {
            ParserState::Normal => {
                result.push(ch as char);
                match ch {
                    b'"' => state = ParserState::String,
                    b'\'' => state = ParserState::Char,
                    _ => (),
                }
            },
            ParserState::String => {
                match ch {
                    b'"' => {
                        result.push(ch as char);
                        state = ParserState::Normal;
                    },
                    b'\\' => state = ParserState::StringBackslash,
                    _ => (),
                }
            },
            ParserState::StringBackslash => {
                state = ParserState::String;
            },
            ParserState::Char => {
                match ch {
                    b'\'' => {
                        result.push(ch as char);
                        state = ParserState::Normal;
                    },
                    b'\\' => state = ParserState::CharBackslash,
                    _ => (),
                }
            },
            ParserState::CharBackslash => {
                state = ParserState::Char;
            },
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_string() {
        assert_eq!(remove_strings("x = \"hello\";"), "x = \"\";");
    }

    #[test]
    fn test_string_with_quotes() {
        assert_eq!(remove_strings("x = \"\\\"\";"), "x = \"\";");
    }

    #[test]
    fn test_simple_char() {
        assert_eq!(remove_strings("char ch = 'a';"), "char ch = '';");
    }

    #[test]
    fn test_char_quote() {
        assert_eq!(remove_strings("char ch = '\\'';"), "char ch = '';");
    }
}
