use crate::tokenizer::Token;
use crate::tokenizer::tokenize;
use crate::preprocessor::get_preprocessor_definitions;

struct DeclarationFinder
{
    tokens: Vec<Token>,
    pos: usize,
    declarations: Vec<String>,
}

/// Filter out unneeded tokens to simplify processing
///
/// 1. Remove preprocessor directives
/// 2. Remove whitespace (Token::WhiteSpace and Token::NewLine)
fn filter_tokens(input_tokens: Vec::<Token>) -> Vec::<Token>
{
    let mut res = Vec::<Token>::new();
    let mut is_in_preprocessor = false;

    for token in input_tokens.into_iter()
    {
        if is_in_preprocessor
        {
            if let Token::NewLine(_) = token
            {
                is_in_preprocessor = false;
            }
            continue;
        }

        if matches!(token, Token::WhiteSpace(_) | Token::NewLine(_) | Token::Comment(_))
        {
            // skip
        }
        else if let Token::Operator(s) = token
        {
            if s == "#"
            {
                is_in_preprocessor = true;
            }
            else
            {
                res.push(Token::Operator(s));
            }
        }
        else
        {
            res.push(token);
        }
    }

    res
}

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

fn skip_bracket_pair(tokens: &Vec<Token>, i: &mut usize, opening_bracket: &str, closing_bracket: &str)
{
    if *i >= tokens.len()
    {
        panic!("skip_bracket_pair: {} expected, EOF found", opening_bracket);
    }
    *i += 1;

    while *i < tokens.len()
    {
        let token = &tokens[*i];

        if *token == Token::Operator(closing_bracket.into())
        {
            *i += 1;
            return
        }
        else if *token == Token::Operator("(".into())
        {
            skip_bracket_pair(&tokens, i, "(", ")");
        }
        else if *token == Token::Operator("{".into())
        {
            skip_bracket_pair(&tokens, i, "{", "}");
        }
        else if *token == Token::Operator("[".into())
        {
            skip_bracket_pair(&tokens, i, "[", "]");
        }
        else
        {
            *i += 1;
        }
    }

    panic!("skip_bracket_pair: EOF");
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

    skip_bracket_pair(&tokens, i, "(", ")");

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

impl DeclarationFinder
{
    fn find_declarations(&mut self, file_content: &str)
    {
        self.declarations = get_preprocessor_definitions(file_content);
        self.tokens = filter_tokens(tokenize(&file_content));

        while self.pos < self.tokens.len()
        {
            if let Token::Identifier(s) = &self.tokens[self.pos]
            {
                self.pos += 1;
                if let Token::Identifier(s1) = &self.tokens[self.pos]
                {
                    self.declarations.push(s1.to_string());
                    self.pos += 1;
                    if let Token::Operator(s2) = &self.tokens[self.pos]
                    {
                        match s2.as_str()
                        {
                            ";" => {self.pos += 1},
                            "=" => skip_to_operator(&self.tokens, &mut self.pos, ";"),
                            "(" => skip_function(&self.tokens, &mut self.pos),
                            _ => panic!("Unexpected operator: {}", s2),
                        }
                    }
                }
            }
            else if let Token::Operator(s) = &self.tokens[self.pos]
            {
                match s.as_str()
                {
                    "{" => skip_bracket_pair(&self.tokens, &mut self.pos, "{", "}"),
                    ";" => { self.pos += 1; },
                    _ => panic!("Unexpected operator: {}", s),
                }
            }
            else
            {
                panic!("Unexpected token: {:?}", &self.tokens[self.pos]);
            }
        }
    }
}

pub fn find_declarations(file_content: &str) -> Vec<String>
{
    let mut d = DeclarationFinder {
        tokens: Vec::<Token>::new(),
        pos: 0,
        declarations: Vec::<String>::new(),
    };
    d.find_declarations(file_content);
    d.declarations
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

    #[test]
    fn test_ignoring_preprocessor_directives() {
        let input = "\
#pragma once

#ifdef A
#     define B
#endif

/*
 * @brief a function returning -123
 */
int myFunc123() {
    return -123;
}
";
        assert_eq!(find_declarations(input), vec!["B", "myFunc123"]);
    }
}
