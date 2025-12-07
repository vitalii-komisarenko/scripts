//mod bracket_tree;
mod comment_remover;
mod string_remover;
mod statement_tree;
mod tokenizer;
mod preprocessor;
mod declaration_finder;
mod standard_headers;

use std::env;
use std::fs;
use std::process;
use std::io::{self, Read};

enum Task {
    PrintHelp,
    RemoveComments,
    RemoveCommentsAndStrings,
    PrintIncludes,
    PrintStandardIncludes,
    PrintCustomIncludes,
    PrintIncludesWithBrackets,
    PrintDeclarations,
    PrintUnusedStandardHeaders,
}

fn print_help() {
    println!("Usage:");
    println!("    --help");
    println!("        Print this help");
    println!("    --remove-comments <filename>");
    println!("        Remove comments from a single C/C++/header file, prints output to the standard output");
    println!("    --remove-comments-and-strings <filename>");
    println!("        Remove comments, strings and chars from a single C/C++/header file, prints output to the standard output");
    println!("    --print-includes <filename>");
    println!("        Print headers used in #include directives");
    println!("    --print-standard-includes <filename>");
    println!("        Print standard headers used in #include directives (headers inside <>)");
    println!("    --print-custom-includes <filename>");
    println!("        Print custom headers used in #include directives (headers inside \"\")");
    println!("    --print-includes-with-brackets <filename>");
    println!("        Print headers used in #include directives. Preserve <> and \"\"");
    println!("    --find-declarations <filename>");
    println!("        Print all declarations and definitions");
    println!("    --print-unused-standard-headers");
    println!("        Print unused standards headers (headers inside <>)");
}

fn read_file_content(path: &str) -> String
{
    if path == "-" {
        let mut res = String::new();
        io::stdin().read_to_string(&mut res).unwrap();
        return res;
    }

    return fs::read_to_string(path).unwrap();
}

fn read_single_file_content(file_names: Vec::<String>) -> String
{
    if file_names.len() != 1 {
        println!("Only one file name expected");
        process::exit(1);
    }

    read_file_content(file_names[0].as_str())
}

fn main() {
    let mut file_names = Vec::new();
    let mut task = Task::PrintHelp;

    for arg in env::args().skip(1) {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--help" => task = Task::PrintHelp,
                "--remove-comments" => task = Task::RemoveComments,
                "--remove-comments-and-strings" => task = Task::RemoveCommentsAndStrings,
                "--print-includes" => task = Task::PrintIncludes,
                "--print-standard-includes" => task = Task::PrintStandardIncludes,
                "--print-custom-includes" => task = Task::PrintCustomIncludes,
                "--print-includes-with-brackets" => task = Task::PrintIncludesWithBrackets,
                "--find-declarations" => task = Task::PrintDeclarations,
                "--print-unused-standard-headers" => task = Task::PrintUnusedStandardHeaders,
                _ => {
                    print_help();
                    process::exit(1);
                }
            }
        }
        else {
            file_names.push(arg);
        }
    }

    match task {
        Task::PrintHelp => print_help(),
        Task::RemoveComments => {
            let file_content = read_single_file_content(file_names);
            let without_comments = comment_remover::remove_comments(file_content.as_str());
            print!("{}", without_comments);
        },
        Task::RemoveCommentsAndStrings => {
            let file_content = read_single_file_content(file_names);
            let without_comments = comment_remover::remove_comments(file_content.as_str());
            let without_comments_and_strings = string_remover::remove_strings(&without_comments);
            print!("{}", without_comments_and_strings);
        },
        Task::PrintIncludes => {
            let file_content = read_single_file_content(file_names);
            for header in preprocessor::get_includes(&file_content).into_iter() {
                println!("{}", header);
            }
        },
        Task::PrintStandardIncludes => {
            let file_content = read_single_file_content(file_names);
            for header in preprocessor::get_standard_includes(&file_content).into_iter() {
                println!("{}", header);
            }
        },
        Task::PrintCustomIncludes => {
            let file_content = read_single_file_content(file_names);
            for header in preprocessor::get_custom_includes(&file_content).into_iter() {
                println!("{}", header);
            }
        },
        Task::PrintIncludesWithBrackets => {
            let file_content = read_single_file_content(file_names);
            for header in preprocessor::get_includes_with_brackets(&file_content).into_iter() {
                println!("{}", header);
            }
        },
        Task::PrintDeclarations => {
            let file_content = read_single_file_content(file_names);
            for header in declaration_finder::find_declarations(&file_content).into_iter() {
                println!("{}", header);
            }
        },
        Task::PrintUnusedStandardHeaders => {
            for file in file_names
            {
                for header in standard_headers::get_unused_headers(&read_file_content(&file))
                {
                    println!("{} : {}", file, header);
                }
            }
        }
    }
}
