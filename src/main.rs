mod ast;
mod backend;
mod cli;
mod lexer;
mod parser;

use backend::*;
use clap::Parser as ClapParser;
use cli::{get_target_language, Args, TargetLanguage};
use lexer::Lexer;
use parser::Parser;
use std::{fs, io, path::Path};

fn get_source_code(path: &str) -> io::Result<Vec<char>> {
    return Ok(fs::read_to_string(path)?.chars().collect::<Vec<char>>());
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.file_name);
    if !path.exists() {
        eprintln!("Error: No such file or directory");
        std::process::exit(1);
    }

    let source_code = get_source_code(&args.file_name)?;

    if source_code.is_empty() {
        std::process::exit(1);
    }

    println!("--STARTING LEXER--");
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize();

    for token in tokens.iter() {
        println!("{:?}", token);
    }
    println!("--ENDING LEXER--\n");

    println!("--STARTING PARSER--");
    let mut parser = Parser::new(tokens);

    let ast = match parser.generate_ast() {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!("--ENDING PARSER--");

    println!("--STARTING CODE GENERATION--");
    let selected_language = get_target_language(&args.target_language);

    let mut code_generator = match selected_language {
        TargetLanguage::C => c::C::new(),
        TargetLanguage::JavaScript => unimplemented!(),
        TargetLanguage::Unknown(unknown_language) => {
            eprintln!("Unknown target language: {}", unknown_language);
            std::process::exit(1);
        }
    };

    code_generator.generate(ast);
    println!("--STARTING CODE GENERATION--");
    Ok(())
}
