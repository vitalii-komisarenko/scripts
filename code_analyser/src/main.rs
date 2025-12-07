//mod bracket_tree;
mod comment_remover;
mod string_remover;
mod statement_tree;

use std::env;
use std::fs;
use std::process;

enum Task {
    PrintHelp,
    RemoveComments,
}

fn print_help() {
    println!("Usage:");
    println!("    --help");
    println!("        Print this help");
    println!("    --remove-comments <filename>");
    println!("        Remove comments from a single C/C++/header file, prints output to the standard output");
}

fn main() {
    let mut file_names = Vec::new();
    let mut task = Task::PrintHelp;

    for arg in env::args().skip(1) {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--help" => task = Task::PrintHelp,
                "--remove-comments" => task = Task::RemoveComments,
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
            if file_names.len() != 1 {
                println!("Only one file name expected");
                process::exit(1);
            }
            else {
                let file_content = fs::read_to_string(file_names[0].as_str()).unwrap();
                let without_comments = comment_remover::remove_comments(file_content.as_str());
                print!("{}", without_comments);
            }
        }
    }
}
