use std::string::String;


#[derive(Debug)]
#[derive(PartialEq)]
enum Token
{
    // A token that is not properly parsed
    Unknown(String),
    // Backshash followed by newline markers, such as \n, \r, \n\r, \r\n
    LineContinuation(String),
    // \n, \r, \n\r or \r\n
    NewLine(String),
    // Whitespace, but not Token::NewLine
    WhiteSpace(String),
    // Single-line or multiline comment
    // Markers of the begin and end (//, newline, /* and */) are included
    Comment(String),
    // String
    // Quotes are included
    String(String),
    // Char
    // Quotes are included
    Char(String),
    // Integer or floating number, but without leading sign
    // E.g. 123456 is a number, 123e-456 is a number, but -123 is a combination of an operator "-" and number 123.
    Number(String),
    // One-character operator, such as +, <, =, ~, &
    //
    // If the operator is made of two characters, such as +=, it is parsed as two operators (+ and = in this case).
    // It is done this way to not confuse operator >> and two closing templates, e.g.:
    // * std::vector<std::vector<int>>
    // * std::vector<std::vector<int> >
    Operator(String),
}


fn read_whitespace(mut s: &str) -> String
{
    let mut res = String::new();

    while s.len() > 0
    {
        let ch = s.bytes().nth(0).unwrap();

        if ch != b' ' && ch != b'\t'
        {
            break;
        }

        res.push(ch as char);
        s = &s[1..];
    }

    res
}


fn read_single_line_comment(mut s: &str) -> String
{
    let mut res = String::new();

    while s.len() > 0
    {
        if s.starts_with("\\\n") || s.starts_with("\\\r")
        {
            res.push_str(&s[..2]);
            s = &s[2..];
            continue;
        }

        if s.starts_with("\n") || s.starts_with("\r")
        {
            res.push_str(&s[..1]);
            s = &s[1..];
            break;
        }

        res.push_str(&s[..1]);
        s = &s[1..];
        continue;
    }

    res
}


fn read_multi_line_comment(mut s: &str) -> String
{
    let mut res = String::new();

    while s.len() > 0
    {
        if s.starts_with("*/")
        {
            res.push_str(&s[..2]);
            s = &s[2..];
            break;
        }

        res.push_str(&s[..1]);
        s = &s[1..];
        continue;
    }

    res
}


fn read_string(mut s: &str) -> String
{
    let mut res = String::new();

    res.push_str(&s[..1]);
    s = &s[1..];

    while s.len() > 0
    {
        if s.starts_with("\\\"")
        {
            res.push_str(&s[..2]);
            s = &s[2..];
            continue;
        }

        if s.starts_with("\"")
        {
            res.push_str(&s[..1]);
            s = &s[1..];
            break;
        }

        res.push_str(&s[..1]);
        s = &s[1..];
        continue;
    }

    res
}


fn read_char(mut s: &str) -> String
{
    let mut res = String::new();

    res.push_str(&s[..1]);
    s = &s[1..];

    while s.len() > 0
    {
        if s.starts_with("\\'")
        {
            res.push_str(&s[..2]);
            s = &s[2..];
            continue;
        }

        if s.starts_with("'")
        {
            res.push_str(&s[..1]);
            s = &s[1..];
            break;
        }

        res.push_str(&s[..1]);
        s = &s[1..];
        continue;
    }

    res
}


fn read_unsigned_int(mut s: &str) -> String
{
    let mut res = String::new();

    while s.len() > 0
    {
        let ch = s.bytes().nth(0).unwrap();
        if b'0' <= ch && ch <= b'9'
        {
            res.push(ch as char);
            s = &s[1..];
        }
        else
        {
            break;
        }
    }

    res
}


fn read_number(mut s: &str) -> String
{
    let mut res = String::new();

    // Integer part
    let mut val = read_unsigned_int(s);
    res.push_str(&s[..val.len()]);
    s = &s[val.len()..];

    // Read dot
    if s.starts_with(".")
    {
        res.push_str(&s[..1]);
        s = &s[1..];
    }

    // Fractional part
    val = read_unsigned_int(s);
    res.push_str(&s[..val.len()]);
    s = &s[val.len()..];

    // Exponent symbol
    if s.starts_with("e") || s.starts_with("E")
    {
        res.push_str(&s[..1]);
        s = &s[1..];
    }
    else
    {
        return res;
    }

    // Exponent sign
    if s.starts_with("-") || s.starts_with("+")
    {
        res.push_str(&s[..1]);
        s = &s[1..];
    }

    // Exponent
    val = read_unsigned_int(s);
    res.push_str(&s[..val.len()]);
    s = &s[val.len()..];

    res
}


pub fn tokenize(file_content: &str) -> Vec<Token>
{
    let mut s = file_content;
    let mut res = Vec::<Token>::new();

    'outer: while s.len() > 0
    {
        let ch = s.bytes().nth(0).unwrap();

        if ch == b' ' || ch == b'\t'
        {
            let val = read_whitespace(s);
            s = &s[val.len()..];
            res.push(Token::WhiteSpace(val));
            continue 'outer;
        }

        for val in ["\\\n\r", "\\\r\n", "\\\n", "\\\r"].into_iter()
        {
            if s.starts_with(val)
            {
                s = &s[val.len()..];
                res.push(Token::LineContinuation(val.to_string()));
                continue 'outer;
            }
        }

        for val in ["\n\r", "\r\n", "\n", "\r"].into_iter()
        {
            if s.starts_with(val)
            {
                s = &s[val.len()..];
                res.push(Token::NewLine(val.to_string()));   
                continue 'outer;
            }
        }

        if s.starts_with("//")
        {
            let val = read_single_line_comment(s);
            s = &s[val.len()..];
            res.push(Token::Comment(val.to_string()));
            continue 'outer;
        }

        if s.starts_with("/*")
        {
            let val = read_multi_line_comment(s);
            s = &s[val.len()..];
            res.push(Token::Comment(val.to_string()));
            continue 'outer;
        }

        if s.starts_with("\"")
        {
            let val = read_string(s);
            s = &s[val.len()..];
            res.push(Token::String(val.to_string()));
            continue 'outer;
        }

        if s.starts_with("'")
        {
            let val = read_char(s);
            s = &s[val.len()..];
            res.push(Token::Char(val.to_string()));
            continue 'outer;
        }

        if (b'0' <= ch && ch <= b'9') ||
           s.starts_with(".0") || s.starts_with(".1") || s.starts_with(".2") || s.starts_with(".3") || s.starts_with(".4") ||
           s.starts_with(".5") || s.starts_with(".6") || s.starts_with(".7") || s.starts_with(".8") || s.starts_with(".9")
        {
            let val = read_number(s);
            s = &s[val.len()..];
            res.push(Token::Number(val.to_string()));
            continue 'outer;
        }

        for val in ["#", "!", "~", "%", "^", "&", "*", "(", ")", "[", "]", "{", "}",
                    "+", "-", "=", "/", ":", ";", "<", ">", "?", ",", ".", "|"].into_iter()
        {
            if s.starts_with(val)
            {
                s = &s[val.len()..];
                res.push(Token::Operator(val.to_string()));   
                continue 'outer;
            }
        }

        let val = &s[..1];
        s = &s[1..];
        res.push(Token::Unknown(val.to_string()));
    }

    res
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::tokenizer::Token;

    #[test]
    fn test_empty() {
        let input = "";
        assert_eq!(tokenize(input), Vec::<Token>::new());
    }

    #[test]
    fn test_whitespace_space() {
        let input = " ";
        assert_eq!(tokenize(input), vec![Token::WhiteSpace(" ".to_string())]);
    }

    #[test]
    fn test_whitespace_tab() {
        let input = "\t";
        assert_eq!(tokenize(input), vec![Token::WhiteSpace("\t".to_string())]);
    }

    #[test]
    fn test_whitespace_mixed() {
        let input = " \t  \t  \t\t\t";
        assert_eq!(tokenize(input), vec![Token::WhiteSpace(" \t  \t  \t\t\t".to_string())]);
    }

    #[test]
    fn test_line_continuation() {
        let input = " \t  \t \\\n \t\t\t";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace(" \t  \t ".to_string()),
            Token::LineContinuation("\\\n".to_string()),
            Token::WhiteSpace(" \t\t\t".to_string()),
        ]);
    }

    #[test]
    fn test_newline() {
        let input = " \t  \t \n \t\t\t";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace(" \t  \t ".to_string()),
            Token::NewLine("\n".to_string()),
            Token::WhiteSpace(" \t\t\t".to_string()),
        ]);
    }

    #[test]
    fn test_single_line_comment() {
        let input = "// \t  \t \n \t\t\t";
        assert_eq!(tokenize(input), vec![
            Token::Comment("// \t  \t \n".to_string()),
            Token::WhiteSpace(" \t\t\t".to_string()),
        ]);
    }

    #[test]
    fn test_multi_line_comment() {
        let input = "/* \t  \t */\n \t\t\t";
        assert_eq!(tokenize(input), vec![
            Token::Comment("/* \t  \t */".to_string()),
            Token::NewLine("\n".to_string()),
            Token::WhiteSpace(" \t\t\t".to_string()),
        ]);
    }


    #[test]
    fn test_empty_string() {
        let input = " \"\" ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace(" ".to_string()),
            Token::String("\"\"".to_string()),
            Token::WhiteSpace(" ".to_string()),
        ]);
    }

    #[test]
    fn test_string() {
        let input = " \"abcd\" ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace(" ".to_string()),
            Token::String("\"abcd\"".to_string()),
            Token::WhiteSpace(" ".to_string()),
        ]);
    }

    #[test]
    fn test_string_with_espaced_double_quotes_1() {
        let input = " \"\\\"abcd\" ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace(" ".to_string()),
            Token::String("\"\\\"abcd\"".to_string()),
            Token::WhiteSpace(" ".to_string()),
        ]);
    }

    #[test]
    fn test_string_with_espaced_double_quotes_2() {
        let input = " \"ab\\\"cd\" ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace(" ".to_string()),
            Token::String("\"ab\\\"cd\"".to_string()),
            Token::WhiteSpace(" ".to_string()),
        ]);
    }

    #[test]
    fn test_string_with_espaced_double_quotes_3() {
        let input = " \"abcd\\\"\" ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace(" ".to_string()),
            Token::String("\"abcd\\\"\"".to_string()),
            Token::WhiteSpace(" ".to_string()),
        ]);
    }

    #[test]
    fn test_char() {
        let input = "  'a'    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Char("'a'".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_char_single_quote() {
        let input = "  '\\''    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Char("'\\''".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_simple_1() {
        let input = "  1    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("1".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_simple_2() {
        let input = "  12345    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("12345".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_float_1() {
        let input = "  1.2    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("1.2".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_float_2() {
        let input = "  1.    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("1.".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_float_3() {
        let input = "  .2    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number(".2".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_float_4() {
        let input = "  123.456    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("123.456".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_exponent_1() {
        let input = "  123.456e789    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("123.456e789".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_exponent_2() {
        let input = "  123.456e-789    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("123.456e-789".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_number_exponent_3() {
        let input = "  123.456E+789    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Number("123.456E+789".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }

    #[test]
    fn test_operator() {
        let input = "  -123.456E+789    ";
        assert_eq!(tokenize(input), vec![
            Token::WhiteSpace("  ".to_string()),
            Token::Operator("-".to_string()),
            Token::Number("123.456E+789".to_string()),
            Token::WhiteSpace("    ".to_string()),
        ]);
    }
}
