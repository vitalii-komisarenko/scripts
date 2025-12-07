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
/// 3. Remove comments
/// 4. Remove unnecessary keywords
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
        else if let Token::Identifier(ref s) = token
        {
            for keyword in vec!["const", "final"]
            {
                if s == keyword
                {
                    // skip
                    continue;
                }
            }

            res.push(token);
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
    fn dump(&self)
    {
        for (i, token) in self.tokens.iter().enumerate()
        {
            let marker = if self.pos == i {" <--------------------------"} else {""};
            println!("{} {:?} {}", i, token, marker);
        }
    }

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

    fn skip_identifier(&mut self, identifier: &str)
    {
        self.skip_exact_token(&Token::Identifier(identifier.into()));
    }

    /// skip __attribute__(( ...... ))
    fn skip_attribute(&mut self)
    {
        self.skip_identifier("__attribute__");
        self.skip_bracket_pair("(", ")");
    }

    /// Skip content inside `<` and `>` (including the closing `>`)
    fn skip_template_brackets(&mut self)
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
                    "<" => self.skip_template_brackets(),
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

        panic!("skip_template_brackets: EOF");
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
                else
                {
                    self.skip_token();
                }
            }
            else
            {
                self.skip_token();
            }
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

        // 3. Skip __attribute__(( ... )) if needed

        if self.eof()
        {
            panic!("skip_function: ';', '{{' or '__attribute__' expected, EOF found");
        }

        if *self.token() == Token::Identifier("__attribute__".into())
        {
            self.skip_attribute();
        }

        // 4. Check if the next token if ';' or '{'

        if self.eof()
        {
            panic!("skip_function: ';' or '{{' expected, EOF found");
        }

        if let Token::Operator(s) = self.token()
        {
            match s.as_str()
            {
                ";" => {
                    // 5A. It is a function declaration. Skip ";"
                    self.skip_token();
                    return;
                },
                "{" => {
                    // 5B. It is a function definition. Skip curly brackets
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

    fn get_declaration(&mut self) -> Option<String>
    {
        let mut last_identifier: String = "".into();
        let intermediate_operators = ["*", ":", "&"];
        let final_operators = [";", "(", "="];

        'outer: while !self.eof()
        {
            if let Token::Identifier(s) = self.token()
            {
                match s.as_str()
                {
                    "__attribute__" => self.skip_attribute(),
                    "operator" => {
                        // We need to check if `operator` is a variable name in C or an operator in C++
                        if self.pos < self.tokens.len() - 1
                        {
                            if let Token::Operator(s) = &self.tokens[self.pos + 1]
                            {
                                match s.as_str()
                                {
                                    ";" | "(" => {
                                        // It is a declaration of variable `operator` or function `operator( ... )`
                                        last_identifier = "operator".into();
                                        self.skip_token();
                                        continue;
                                    },
                                    "=" => {
                                        if (self.pos < self.tokens.len() - 1) && (self.tokens[self.pos + 2] == Token::Operator("=".into()))
                                        {
                                            // It is `operator ==`
                                            self.skip_token();
                                            last_identifier = "".into();
                                            continue;
                                        }
                                        else {
                                            // It is assignemnt to variable `operator`
                                            last_identifier = "operator".into();
                                            self.skip_token();
                                            continue;
                                        }
                                    },
                                    _ => {
                                        // It is an operator overload in C++
                                        self.skip_token();
                                        last_identifier = "".into();
                                        continue;
                                    }
                                }
                            }
                            else {
                                panic!();
                            }
                        }
                    },
                    _ => {
                        last_identifier = s.clone();
                        self.skip_token();
                    }
                }
                continue;
            }

            for op in intermediate_operators
            {
                if *self.token() == Token::Operator(op.into())
                {
                    self.skip_token();
                    continue 'outer;
                }
            }

            if *self.token() == Token::Operator("<".into())
            {
                self.skip_template_brackets();
                continue;
            }

            for op in final_operators
            {
                if *self.token() == Token::Operator(op.into())
                {
                    if last_identifier == ""
                    {
                        return None;
                    }
                    return Some(last_identifier);
                }
            }

            panic!("get_declaration: Unexpected token: {:?}", self.token());
        }

        panic!("get_declaration: EOF");
    }

    fn process_using(&mut self)
    {
        self.skip_identifier("using");
        if let Token::Identifier(s) = self.token()
        {
            match s.as_str(){
                "namespace" => self.skip_to_operator(";"),
                _ => {
                    self.declarations.push(s.to_string());
                    self.skip_to_operator(";");
                },
            }
        }
        else
        {
            panic!("find_declarations: Identifier expected after `using` but {:?} found", self.token());
        }
    }

    fn process_class_or_struct_without_semicolon(&mut self)
    {
        self.skip_token(); // skip `class`/`struct` keyword
        if self.eof()
        {
            panic!("process_class_or_struct: EOF while reading class/struct name");
        }

        if let Token::Identifier(s) = self.token()
        {
            self.declarations.push(s.to_string());
            self.skip_token(); // class/struct name already processed
        }

        while (*self.token() != Token::Operator(";".to_string())) && (*self.token() != Token::Operator("{".to_string()))
        {
            if self.eof()
            {
                panic!("process_class_or_struct: EOF while parsing class/struct: ';' or '{{' needed");
            }
            self.skip_token();
        }

        if *self.token() == Token::Operator(";".to_string())
        {
            return;
        }
        else if *self.token() == Token::Operator("{".to_string())
        {
            self.skip_bracket_pair("{", "}");
            return;
        }
        else
        {
            panic!("process_class_or_struct: unexpected token after class/struct name: {:?}", self.token());
        }
    }

    fn process_class_or_struct(&mut self)
    {
        self.process_class_or_struct_without_semicolon();
        self.skip_operator(";");
    }

    fn process_typedef(&mut self)
    {
        self.skip_identifier("typedef");
        if let Token::Identifier(identifier) = self.token()
        {
            match identifier.as_str()
            {
                "struct" | "class" => self.process_class_or_struct_without_semicolon(),
                _ => {
                    if let Some(declaration) = self.get_declaration()
                    {
                        self.declarations.push(declaration);
                    }
                    self.skip_operator(";");
                }
            }
        }
        else
        {
            panic!("Process typedef: Identifier expected, {:?} found", self.token());
        }
    }

    fn find_declarations(&mut self, file_content: &str)
    {
        self.declarations = get_preprocessor_definitions(file_content);
        self.tokens = filter_tokens(tokenize(&file_content));

        while !self.eof()
        {
            if let Token::Identifier(s) = self.token()
            {
                match s.as_str()
                {
                    "template" => {
                        self.skip_identifier("template");
                        self.skip_template_brackets();
                    },
                    "using" => self.process_using(),
                    "class" | "struct" => self.process_class_or_struct(),
                    "typedef" => self.process_typedef(),
                    _ => {
                        if let Some(declaration) = self.get_declaration()
                        {
                            self.declarations.push(declaration);
                        }

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
                        else
                        {
                            panic!("find_declarations: Unexpected token {:?}", self.token());
                        }
                    },
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

std::vector<int> vec1 = {1, 2, 3};
std::vector<std::vector<int>> vec2;

int main()
{
    std::cout << function<std::vector<std::vector<std::string>>, 3>() << \"\\n\";
}";
        assert_eq!(find_declarations(input), vec!["function", "vec1", "vec2", "main"]);
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

    #[test]
    fn test_complex_types_c() {
        let input = "\
            const char a;
            const char const b;
            const const char *** const const const c;
            const static char * * * const const d;
            static const const char * * * const const e;
            f; // implicit int
            static inline __attribute__((noreturn)) g() {}
            int	h __attribute__((unused)) = 3;
            extern void exit(int)   __attribute__((noreturn));

            int main() {}
";
        assert_eq!(find_declarations(input), vec!["a", "b", "c", "d", "e", "f", "g", "h", "exit", "main"]);
    }

    #[test]
    fn test_operator_keyword_in_c_1() {
        let input = "operator;";
        assert_eq!(find_declarations(input), vec!["operator"]);
    }

    #[test]
    fn test_operator_keyword_in_c_2() {
        let input = "float operator;";
        assert_eq!(find_declarations(input), vec!["operator"]);
    }

    #[test]
    fn test_operator_keyword_in_c_3() {
        let input = "const const const operator;";
        assert_eq!(find_declarations(input), vec!["operator"]);
    }

    #[test]
    fn test_operator_keyword_in_c_4() {
        let input = "const const int operator = 123;";
        assert_eq!(find_declarations(input), vec!["operator"]);
    }

    #[test]
    fn test_operator_keyword_in_c_5() {
        let input = "operator();";
        assert_eq!(find_declarations(input), vec!["operator"]);
    }

    #[test]
    fn test_operator_keyword_in_c_6() {
        let input = "void operator();";
        assert_eq!(find_declarations(input), vec!["operator"]);
    }

    #[test]
    fn test_operator_keyword_in_c_7() {
        let input = "const const const char * operator();";
        assert_eq!(find_declarations(input), vec!["operator"]);
    }

    #[test]
    fn test_using_namespace() {
        let input = "
        #include <iostream>
        using namespace std;
        using namespace A::B::C;
        int main() {}
        ";
        assert_eq!(find_declarations(input), vec!["main"]);
    }

    #[test]
    fn test_using_type() {
        let input = "
            #include <vector>
            #include <string>

            using myType = std::vector<std::string>;

            myType myVector = {\"aaa\", \"bbb\", \"ccc\"};

            int main() {}
        ";
        assert_eq!(find_declarations(input), vec!["myType", "myVector", "main"]);
    }

    #[test]
    fn test_typedef() {
        let input = "
            #include <stdio.h>

            typedef int t;

            typedef struct a
            {
                int a;
            } b;

            typedef struct {
                int x;
            } c;

            struct a aa;
            b bb;
            c cc;

            void main() {
                t x = 5;
                printf(\"%d\\n\", x);
            }
        ";
        assert_eq!(find_declarations(input), vec!["t", "a", "b", "c", "aa", "bb", "cc", "main"]);
    }
}
