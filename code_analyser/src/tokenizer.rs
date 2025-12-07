use std::string::String;


#[derive(Debug)]
#[derive(PartialEq)]
enum Token
{
    None,
    PreprocessorDirective(String),
    LineContinuation(String),
    NewLine(String),
    WhiteSpace(String),
    Comment(String),
    String(String),
    Char(String),
    Number(String),
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

pub fn tokenize(file_content: &str) -> Vec<Token>
{
    let mut s = file_content;
    let mut res = Vec::<Token>::new();

    while s.len() > 0
    {
        let ch = s.bytes().nth(0).unwrap();

        if ch == b' ' || ch == b'\t'
        {
            let val = read_whitespace(s);
            s = &s[val.len()..];
            res.push(Token::WhiteSpace(val));
        }

        for val in ["\\\n\r", "\\\r\n", "\\\n", "\\\r"].into_iter()
        {
            if s.starts_with(val)
            {
                s = &s[val.len()..];
                res.push(Token::LineContinuation(val.to_string()));   
            }
        }

        for val in ["\n\r", "\r\n", "\n", "\r"].into_iter()
        {
            if s.starts_with(val)
            {
                s = &s[val.len()..];
                res.push(Token::NewLine(val.to_string()));   
            }
        }

        if s.starts_with("//")
        {
            let val = read_single_line_comment(s);
            s = &s[val.len()..];
            res.push(Token::Comment(val.to_string()));
        }
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
}
