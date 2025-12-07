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

fn skip_function(tokens: &Vec<Token>, i: &mut usize)
{
    // To handle function declarations and definitions, e.g.:
    //
    // int myFunc ( ... );
    // int myFunc ( ... ) { ... }
    //
    // The pointer is at the "("

    // 1. Check that we point at "("

    if *i >= tokens.len()
    {
        panic!("skip_function: '(' expected, EOF found");
    }

    if let Token::Operator(s) = &tokens[*i]
    {
        if s != "("
        {
            panic!("skip_function: '(' expected, {:?} found", &tokens[*i]);
        }
    }
    else
    {
        panic!("skip_function: '(' expected, {:?} found", &tokens[*i]);
    }

    // 2. Skip parameter list (round brackets)

    *i += 1;
    skip_to_operator(&tokens, i, ")");

    // 3. Check if the next token if ';' or '{'

    if *i >= tokens.len()
    {
        panic!("skip_function: ';' or '{{' expected, EOF found");
    }

    if let Token::Operator(s) = &tokens[*i]
    {
        match s.as_str()
        {
            ";" => {
                // 4A. It is a function declaration. Skip ";"
                *i += 1;
                return;
            },
            "{" => {
                // 4B. It is a function definition. Skip curly brackets
                *i += 1;
                skip_to_operator(&tokens, i, "}");
                return;
            }
            _ => panic!("skip_function: ';' or '{{' expected, {:?} found", &tokens[*i]),
        }
    }
    else
    {
        panic!("skip_function: ';' or '{{' expected, {:?} found", &tokens[*i]);
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
                    match s2.as_str()
                    {
                        ";" => {i += 1},
                        "=" => skip_to_operator(&tokens, &mut i, ";"),
                        "(" => skip_function(&tokens, &mut i),
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
