use crate::tokenizer::Token;
use crate::tokenizer::tokenize;
use crate::preprocessor::get_preprocessor_definitions;

struct DeclarationFinder
{
    tokens: Vec<Token>,
    pos: usize,
    declarations: Vec<String>,
}

impl DeclarationFinder
{
    fn eof(&self) -> bool
    {
        self.pos >= self.tokens.len()
    }

    fn token(&self) -> &Token
    {
        &self.tokens[self.pos]
    }

    fn skip_token(&mut self)
    {
        self.pos += 1;
    }
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

impl DeclarationFinder
{
    fn skip_exact_token(&mut self, token_to_skip: &Token)
    {
        if self.eof()
        {
            panic!("skip_exact_token: {:?} expected, EOF found", token_to_skip);
        }
        if self.token() != token_to_skip
        {
            panic!("skip_exact_token: {:?} expected, {:?} found", token_to_skip, self.token());
        }
        self.skip_token();
    }

    fn skip_operator(&mut self, operator: &str)
    {
        self.skip_exact_token(&Token::Operator(operator.into()));
    }

    /// Skip content inside `<` and `>` (including the closing `>`)
    fn skip_template(&mut self)
    {
        self.skip_operator("<");

        while !self.eof()
        {
            if let Token::Operator(s) = self.token()
            {
                match s.as_str()
                {
                    "(" => self.skip_bracket_pair("(", ")"),
                    "{" => self.skip_bracket_pair("{", "}"),
                    "[" => self.skip_bracket_pair("[", "]"),
                    "<" => self.skip_template(),
                    ">" => {
                        self.skip_token();
                        return;
                    },
                    _ => self.skip_token(),
                }
            }
            else
            {
                self.skip_token();
            }
        }

        panic!("skip_template: EOF");
    }

    fn skip_to_operator(&mut self, operator: &str)
    {
        while !self.eof()
        {
            if let Token::Operator(s) = &self.token()
            {
                if s == operator
                {
                    self.skip_token();
                    return
                }
                else if s == "("
                {
                    self.skip_token();
                    self.skip_to_operator(")");
                }
                else if s == "{"
                {
                    self.skip_token();
                    self.skip_to_operator("}");
                }
                else if s == "["
                {
                    self.skip_token();
                    self.skip_to_operator("]");
                }
            }
            self.skip_token();
        }
    }

    fn skip_bracket_pair(&mut self, opening_bracket: &str, closing_bracket: &str)
    {
        if self.eof()
        {
            panic!("skip_bracket_pair: {} expected, EOF found", opening_bracket);
        }

        self.skip_token();

        while !self.eof()
        {
            let token = self.token();

            if *token == Token::Operator(closing_bracket.into())
            {
                self.skip_token();
                return
            }
            else if *token == Token::Operator("(".into())
            {
                self.skip_bracket_pair("(", ")");
            }
            else if *token == Token::Operator("{".into())
            {
                self.skip_bracket_pair("{", "}");
            }
            else if *token == Token::Operator("[".into())
            {
                self.skip_bracket_pair("[", "]");
            }
            else
            {
                self.skip_token();
            }
        }

        panic!("skip_bracket_pair: EOF");
    }

    fn skip_function(&mut self)
    {
        // To handle function declarations and definitions, e.g.:
        //
        // int myFunc ( ... );
        // int myFunc ( ... ) { ... }
        //
        // The pointer is at the "("

        // 1. Check that we point at "("

        if self.eof()
        {
            panic!("skip_function: '(' expected, EOF found");
        }

        if *self.token() != Token::Operator("(".into())
        {
            panic!("skip_function: '(' expected, {:?} found", self.token());
        }

        // 2. Skip parameter list (round brackets)

        self.skip_bracket_pair("(", ")");

        // 3. Check if the next token if ';' or '{'

        if self.eof()
        {
            panic!("skip_function: ';' or '{{' expected, EOF found");
        }

        if let Token::Operator(s) = self.token()
        {
            match s.as_str()
            {
                ";" => {
                    // 4A. It is a function declaration. Skip ";"
                    self.skip_token();
                    return;
                },
                "{" => {
                    // 4B. It is a function definition. Skip curly brackets
                    self.skip_token();
                    self.skip_to_operator("}");
                    return;
                }
                _ => panic!("skip_function: ';' or '{{' expected, {:?} found", self.token()),
            }
        }
        else
        {
            panic!("skip_function: ';' or '{{' expected, {:?} found", self.token());
        }
    }

    fn find_declarations(&mut self, file_content: &str)
    {
        self.declarations = get_preprocessor_definitions(file_content);
        self.tokens = filter_tokens(tokenize(&file_content));

        while !self.eof()
        {
            if *self.token() == Token::Identifier("template".into())
            {
                self.skip_token(); // skip `template` keyword
                self.skip_template();
            }
            else if (*self.token() == Token::Identifier("class".into()) || *self.token() == Token::Identifier("struct".into()))
            {
                self.skip_token(); // skip `class`/`struct` keyword
                if self.eof()
                {
                    panic!("find_declarations: EOF while reading class/struct name");
                }
                if let Token::Identifier(s) = self.token()
                {
                    self.declarations.push(s.to_string());
                    self.skip_token(); // class/struct name already processed

                    while (*self.token() != Token::Operator(";".to_string())) && (*self.token() != Token::Operator("{".to_string()))
                    {
                        if self.eof()
                        {
                            panic!("find_declarations: EOF while parsing class/struct: ';' or '{{' needed");
                        }
                        self.skip_token();
                    }

                    if *self.token() == Token::Operator(";".to_string())
                    {
                        self.skip_token();
                        continue;
                    }
                    else if *self.token() == Token::Operator("{".to_string())
                    {
                        self.skip_bracket_pair("{", "}");
                        self.skip_token(); // Skip ';'
                        continue;
                    }
                    else
                    {
                        panic!("find_declarations: unexpected token after class/struct name: {:?}", self.token());
                    }
                }
                else
                {
                    panic!("find_declarations: Unexpected token while reading class/struct name: {:?}", self.token())
                }
            }
            else if let Token::Identifier(s) = &self.tokens[self.pos]
            {
                self.pos += 1;
                if let Token::Identifier(s1) = &self.tokens[self.pos]
                {
                    if s1 == "operator"
                    {
                        self.skip_token(); // skip `operator` keyword
                        loop
                        {
                            if self.eof()
                            {
                                panic!("find_declarations: EOF while skipping operator");
                            }
                            if *self.token() == Token::Operator("(".into())
                            {
                                break;
                            }
                            self.skip_token();
                        }
                        self.skip_function();
                    }
                    else
                    {
                        self.declarations.push(s1.to_string());
                        self.pos += 1;
                        if let Token::Operator(s2) = &self.tokens[self.pos]
                        {
                            match s2.as_str()
                            {
                                ";" => {self.pos += 1},
                                "=" => self.skip_to_operator(";"),
                                "(" => self.skip_function(),
                                _ => panic!("Unexpected operator: {}", s2),
                            }
                        }
                    }
                }
            }
            else if let Token::Operator(s) = &self.tokens[self.pos]
            {
                match s.as_str()
                {
                    "{" => self.skip_bracket_pair("{", "}"),
                    ";" => self.skip_token(),
                    _ => panic!("Unexpected operator: {}", s),
                }
            }
            else
            {
                panic!("Unexpected token: {:?}", self.token());
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

    #[test]
    fn test_simple_template() {
        let input = "template <typename T> T my_func(T a, const T& b);";
        assert_eq!(find_declarations(input), vec!["my_func"]);
    }

    #[test]
    fn test_complex_template() {
        let input = "\
#include <iostream>
#include <string>
#include <vector>

template <typename T, int X, int Y = (X > 5) ? 3 : 4>
int function()
{
    return Y;
}

int main()
{
    std::cout << function<std::vector<std::vector<std::string>>, 3>() << \"\\n\";
}";
        assert_eq!(find_declarations(input), vec!["function", "main"]);
    }

    #[test]
    fn test_class_forward_declaration() {
        let input = "\
struct X;
class Y;
";
        assert_eq!(find_declarations(input), vec!["X", "Y"]);
    }

    #[test]
    fn test_class_simple_definitions() {
        let input = "\
struct X
{
};
class Y
{
    int a;
    void b();
};
";
        assert_eq!(find_declarations(input), vec!["X", "Y"]);
    }

    #[test]
    fn test_class_inheritance_1() {
        let input = "\
class X
{
};
class Y: public X
{
    int a;
    void b();
};
";
        assert_eq!(find_declarations(input), vec!["X", "Y"]);
    }

    #[test]
    fn test_class_inheritance_2() {
        let input = "\
class X
{
};
class Y final: public X
{
    int a;
    void b();
};
";
        assert_eq!(find_declarations(input), vec!["X", "Y"]);
    }


    #[test]
    fn test_template_with_template_parameter() {
        let input = "\
#include <functional>
#include <string>
#include <vector>

template <class T=std::hash<std::vector<std::string>>>
class myClass
{
public:
    T a;
    T b() {
        return T();
    }
};

int main()
{
};
";
        assert_eq!(find_declarations(input), vec!["myClass", "main"]);
    }

    #[test]
    fn test_operator() {
        let input = "\
struct S
{
    std::string name;
};

bool operator==(const S& lhs, const S& rhs)
{
    return lhs.name == rhs.name;
}";
        assert_eq!(find_declarations(input), vec!["S"]);
    }
}
