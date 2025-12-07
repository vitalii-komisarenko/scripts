use crate::tokenizer::tokenize;
use crate::tokenizer::Token;

#[derive(PartialEq)]
enum State
{
    LookingForOctothorp,
    LookingForDefineWord,
    LookingForIdentifier,
    LookingForNewLine,
}

pub fn get_preprocessor_definitions(file_content: &str) -> Vec<String>
{
    let mut res = Vec::<String>::new();
    let tokens = tokenize(file_content);
    let mut state = State::LookingForOctothorp;

    for token in tokens.into_iter()
    {
        if state == State::LookingForOctothorp
        {
            if let Token::Operator(s) = token
            {
                if s == "#".to_string()
                {
                    state = State::LookingForDefineWord;
                }
            }

            continue;
        }

        if state == State::LookingForDefineWord
        {
            if let Token::Identifier(s) = token
            {
                if s == "define".to_string()
                {
                    state = State::LookingForIdentifier;
                    continue;
                }
            }
            else if let Token::WhiteSpace(_) = token
            {
                continue;
            }

            state = State::LookingForNewLine;
            continue;
        }

        if state == State::LookingForIdentifier
        {
            if let Token::Identifier(s) = token
            {
                res.push(s);
                state = State::LookingForNewLine;
            }

            continue;
        }

        if state == State::LookingForNewLine
        {
            if let Token::NewLine(s) = token
            {
                state = State::LookingForOctothorp;
            }
            continue;
        }
    }

    res
}

pub fn get_includes(file_content: &str) -> Vec<String>
{
    let mut res = Vec::<String>::new();
    let tokens = tokenize(file_content);
    let mut state = State::LookingForOctothorp;
    let mut new_include = String::new();

    for token in tokens.into_iter()
    {
        if state == State::LookingForOctothorp
        {
            if let Token::Operator(s) = token
            {
                if s == "#".to_string()
                {
                    state = State::LookingForDefineWord;
                }
            }

            continue;
        }

        if state == State::LookingForDefineWord
        {
            if let Token::Identifier(s) = token
            {
                if s == "include".to_string()
                {
                    state = State::LookingForNewLine;
                    continue;
                }
            }
            else if let Token::WhiteSpace(_) = token
            {
                continue;
            }

            state = State::LookingForOctothorp;
            continue;
        }

        if state == State::LookingForNewLine
        {
            if let Token::NewLine(s) = token
            {
                res.push(new_include);
                new_include = String::new();
                state = State::LookingForOctothorp;
            }
            else if let Token::String(s) = token
            {
                new_include.push_str(&s)
            }
            else if let Token::Operator(s) = token
            {
                new_include.push_str(&s)
            }
            else if let Token::Identifier(s) = token
            {
                new_include.push_str(&s)
            }
            else if let Token::Number(s) = token
            {
                new_include.push_str(&s)
            }
            continue;
        }
    }

    res
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_empty_file()
    {
        let input = "";
        assert_eq!(get_preprocessor_definitions(input), Vec::<String>::new());
    }

    #[test]
    fn test_no_defines()
    {
        let input = "#include <iostream>\n
                    \n
                    int main()\n
                    {\n
                        std::cout << \"Hello, World!\\n\";\n
                    }\n";
        assert_eq!(get_preprocessor_definitions(input), Vec::<String>::new());
    }

    #[test]
    fn test_simple_define()
    {
        let input = "#include <iostream>\n
                    \n
                    #define A\n
                    #define B A\n
                    #define D 123\n
                    #     define    C  \n
                    #define define\n
                    #ifdef A\n
                    #define X\\\n
                        123\n
                    #endif\n
                    int main()\n
                    {\n
                        std::cout << \"Hello, World!\\n\";\n
                    }\n";
        assert_eq!(get_preprocessor_definitions(input), vec![
            "A".to_string(),
            "B".to_string(),
            "D".to_string(),
            "C".to_string(),
            "define".to_string(),
            "X".to_string(),
        ]);
    }

    #[test]
    fn test_includes_1()
    {
        let input = "\
#include <iostream>

#include \"file.h\"
#include <string>
#include <string.h>
#include <2.h>
#include \"a/b/c.hpp\"
#include  \t  \"abc.h\"  \t  
#include  \t  <vector>   \t  
#       include    <deque>   \t  
int main()
{
    return 0;
}
";
        assert_eq!(get_includes(input), vec![
            "<iostream>".to_string(),
            "\"file.h\"".to_string(),
            "<string>".to_string(),
            "<string.h>".to_string(),
            "<2.h>".to_string(),
            "\"a/b/c.hpp\"".to_string(),
            "\"abc.h\"".to_string(),
            "<vector>".to_string(),
            "<deque>".to_string(),
        ]);
    }

    #[test]
    fn test_includes_2()
    {
        let input = "#include <iostream>\n
                    \n
                    #include \"file.h\"\n
                    #include <string>\n
                    #include <string.h>\n
                    #include <2.h>\n
                    #include \"a/b/c.hpp\"\n
                    #include  \t  \"abc.h\"  \t  \n
                    #include  \t  <vector>   \t  \n
                    #       include    <deque>   \t  \n
                    int main()\n
                    {\n
                        return 0;\n
                    }\n";
        assert_eq!(get_includes(input), vec![
            "<iostream>".to_string(),
            "\"file.h\"".to_string(),
            "<string>".to_string(),
            "<string.h>".to_string(),
            "<2.h>".to_string(),
            "\"a/b/c.hpp\"".to_string(),
            "\"abc.h\"".to_string(),
            "<vector>".to_string(),
            "<deque>".to_string(),
        ]);
    }
}

