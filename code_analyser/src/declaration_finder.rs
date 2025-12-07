use crate::comment_remover::remove_comments;
use crate::string_remover::remove_strings;
use crate::tokenizer::Token;
use crate::tokenizer::tokenize;

fn skip_to_operator(tokens: &Vec<Token>, i: &mut usize, operator: &str)
{
    while *i < tokens.len()
    {
        if let Token::Operator(s) = &tokens[*i]
        {
            if s == operator
            {
                *i += 1;
                return
            }
        }
        *i += 1;
    }
}

pub fn find_declarations(file_content: &str) -> Vec<String>
{
    let mut content = remove_comments(file_content);
    content = remove_strings(content.as_str());

    let mut res = Vec::<String>::new();
    let tokens_with_whitespace = tokenize(&content);

    let mut tokens = Vec::<Token>::new();
    for token in tokens_with_whitespace.into_iter()
    {
        if let Token::WhiteSpace(_) = token
        {
            // skip
        }
        else
        {
            tokens.push(token);
        }
    }

    let mut i = 0;
    while i < tokens.len()
    {
        if let Token::Identifier(s) = &tokens[i]
        {
            i += 1;
            if let Token::Identifier(s1) = &tokens[i]
            {
                res.push(s1.to_string());
                skip_to_operator(&tokens, &mut i, ";");
            }
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_var() {
        let input = "int myVar;";
        assert_eq!(find_declarations(input), vec!["myVar"]);
    }
}
