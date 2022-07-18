mod ast;
mod backend;
mod lexer;
mod parser;

use backend::*;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs, io, path::Path};

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author="Hícaro Dânrlley", version="0.1", about="A general purpose and open-source programming language", long_about = None)]
struct Args {
    /// File name
    #[clap(short = 'f', long = "name", value_parser)]
    file_name: String,

    /// Language target to compile (C, JavaScript)
    #[clap(short = 't', long = "target", value_parser)]
    language_target: String,
}

fn get_source_code(path: String) -> io::Result<Vec<char>> {
    return Ok(fs::read_to_string(path)?.chars().collect::<Vec<char>>());
}

enum TargetLanguage<'a> {
    C,
    JavaScript,
    Unknown(&'a str),
}

fn get_target_language(selected_language: &str) -> TargetLanguage {
    match selected_language {
        "C" => TargetLanguage::C,
        "JavaScript" => TargetLanguage::JavaScript,
        unknown_language => TargetLanguage::Unknown(unknown_language),
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = Path::new(&args.file_name);
    if !path.exists() {
        eprintln!("Error: No such file or directory");
        std::process::exit(1);
    }

    let source_code = get_source_code(args.file_name.clone())?;

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
    let selected_language = get_target_language(&args.language_target);

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
