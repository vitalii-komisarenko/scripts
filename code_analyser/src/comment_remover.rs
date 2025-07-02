enum ParserState {
    Normal,
    String,
    StringBackslash,
    Char,
    CharBackslash,
    Slash,
    SingleLineComment,
    SingleLineCommentNewLine,
    MultiLineComment,
    MultiLineCommentAsterisk,
}

fn remove_comments(file_content: &str) -> String {
    let mut result: String = Default::default();
    let mut state: ParserState = ParserState::Normal;
    for ch in file_content.bytes() {
        match state {
            ParserState::Normal => {
                if ch == b'/' {
                    state = ParserState::Slash;
                }
                else {
                    result.push(ch as char);
                    match ch {
                        b'"' => state = ParserState::String,
                        b'\'' => state = ParserState::Char,
                        _ => (),
                    }
                }
            },
            ParserState::String => {
                result.push(ch as char);
                match ch {
                    b'"' => state = ParserState::Normal,
                    b'\\' => state = ParserState::StringBackslash,
                    _ => (),
                }
            },
            ParserState::StringBackslash => {
                result.push(ch as char);
                state = ParserState::String;
            },
            ParserState::Char => {
                result.push(ch as char);
                match ch {
                    b'\'' => state = ParserState::Normal,
                    b'\\' => state = ParserState::CharBackslash,
                    _ => (),
                }
            },
            ParserState::CharBackslash => {
                result.push(ch as char);
                state = ParserState::Char;
            },
            ParserState::Slash => {
                match ch {
                    b'/' => state = ParserState::SingleLineComment,
                    b'*' => state = ParserState::MultiLineComment,
                    _ => {
                        result.push('/');
                        result.push(ch as char);
                        state = ParserState::Normal;
                    }
                }
            },
            ParserState::SingleLineComment => {
                match ch {
                    b'\n' | b'\r' => state = ParserState::SingleLineCommentNewLine,
                    _ => (),
                }
            },
            ParserState::SingleLineCommentNewLine => {
                match ch {
                    b'\n' | b'\r' => (),
                    _ => {
                        result.push(ch as char);
                        state = ParserState::Normal;
                    }
                }
            },
            ParserState::MultiLineComment => {
                match ch {
                    b'*' => state = ParserState::MultiLineCommentAsterisk,
                    _ => (),
                }
            },
            ParserState::MultiLineCommentAsterisk => {
                match ch {
                    b'/' => state = ParserState::Normal,
                    _ => (),
                }
            }
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_comments() {
        let input = "int main() {}";
        assert_eq!(remove_comments(input), input);
    }

    #[test]
    fn test_no_comments_multiline() {
        let input = "int main()\n{\n}\n";
        assert_eq!(remove_comments(input), input);
    }

    #[test]
    fn test_no_comments_multiline_windows_newlines() {
        let input = "int main()\r\n{\r\n}\r\n";
        assert_eq!(remove_comments(input), input);
    }

    #[test]
    fn test_no_comments_multiline_old_mac_newlines() {
        let input = "int main()\r{\r}\r";
        assert_eq!(remove_comments(input), input);
    }

    #[test]
    fn test_single_line_comment() {
        let input = "\
// single-line comment
int main() {
}";
        let output = "\
int main() {
}";
        assert_eq!(remove_comments(input), output);
    }

    #[test]
    fn test_multi_line_comment() {
        let input = "\
/* multi-line comment */
int main() {
}";
        let output = "\n\
int main() {
}";
        assert_eq!(remove_comments(input), output);
    }

    #[test]
    fn test_multi_line_comment_with_asterisk() {
        let input = "\
/* multi-line * comment */
int main() {
}";
        let output = "\n\
int main() {
}";
        assert_eq!(remove_comments(input), output);
    }

    #[test]
    fn test_simple_string() {
        let input = "\
int main() {
    printf(\"hello, world!\");
}";
        let output = "\
int main() {
    printf(\"hello, world!\");
}";
        assert_eq!(remove_comments(input), output);
    }

    #[test]
    fn test_string_with_double_quotes() {
        let input = "\
int main() {
    printf(\"hello \\\" world!\");
}";
        let output = "\
int main() {
    printf(\"hello \\\" world!\");
}";
        assert_eq!(remove_comments(input), output);
    }

    #[test]
    fn test_string_with_two_slashes() {
        let input = "\
int main() {
    printf(\"hello // world!\");
}";
        let output = "\
int main() {
    printf(\"hello // world!\");
}";
        assert_eq!(remove_comments(input), output);
    }

    #[test]
    fn test_string_with_slashe_and_asterisk() {
        let input = "\
int main() {
    printf(\"hello /* world!\");
}";
        let output = "\
int main() {
    printf(\"hello /* world!\");
}";
        assert_eq!(remove_comments(input), output);
    }

    #[test]
    fn test_normal_char() {
        let input = "int main() {char ch = '1';}";
        assert_eq!(remove_comments(input), input);
    }

    #[test]
    fn test_char_with_backslash() {
        let input = "int main() {char ch = '\\n';}";
        assert_eq!(remove_comments(input), input);
    }

}
