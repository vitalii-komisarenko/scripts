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
}

