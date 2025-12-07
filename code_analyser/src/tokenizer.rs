use std::string::String;


#[derive(Debug)]
#[derive(PartialEq)]
enum Token
{
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


pub fn tokenize(s: &str) -> Vec<Token>
{
    let mut res = Vec::<Token>::new();

    res
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::tokenizer;

    #[test]
    fn test_empty() {
        let input = "";
        assert_eq!(tokenize(input), Vec::<tokenizer::Token>::new());
    }
}
