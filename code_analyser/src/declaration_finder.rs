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
            if s == "("
            {
                *i += 1;
                skip_to_operator(&tokens, i, ")");
            }
            if s == "{"
            {
                *i += 1;
                skip_to_operator(&tokens, i, "}");
            }
            if s == "["
            {
                *i += 1;
                skip_to_operator(&tokens, i, "]");
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
        else if let Token::NewLine(_) = token
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
                i += 1;
                if let Token::Operator(s2) = &tokens[i]
                {
                    i += 1;
                    match s2.as_str()
                    {
                        ";" => (),
                        "=" => skip_to_operator(&tokens, &mut i, ";"),
                        "(" => skip_to_operator(&tokens, &mut i, ")"),
                        _ => panic!("Unexpected operator: {}", s2),
                    }
                }
            }
        }
        else if let Token::Operator(s) = &tokens[i]
        {
            match s.as_str()
            {
                "{" => skip_to_operator(&tokens, &mut i, "}"),
                ";" => { i += 1; },
                _ => panic!("Unexpected operator: {}", s),
            }
        }
        else
        {
            panic!("Unexpected token: {:?}", &tokens[i]);
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

    #[test]
    fn test_single_var_2() {
        let input = "int myVar = 5;";
        assert_eq!(find_declarations(input), vec!["myVar"]);
    }

    #[test]
    fn test_simple_function_declaration() {
        let input = "int myFunc();";
        assert_eq!(find_declarations(input), vec!["myFunc"]);
    }

    #[test]
    fn test_simple_function_definition() {
        let input = "\
int main() {\n
    return 0;\n
}\n
";
        assert_eq!(find_declarations(input), vec!["main"]);
    }
}
