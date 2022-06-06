mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::{env, fs, io, path::Path};

fn get_source_code(path: String) -> io::Result<Vec<char>> {
    return Ok(fs::read_to_string(path)?.chars().collect::<Vec<char>>());
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No input files");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        eprintln!("Error: No such file or directory");
        std::process::exit(1);
    }

    let source_code = get_source_code(args[1].clone())?;

    if source_code.len() == 0 {
        std::process::exit(1);
    }

    println!("--STARTING LEXER--");
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize();

    for token in tokens.iter() {
        println!("{:?}", token);
    }
    println!("--ENDING LEXER--");

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
    Ok(())
}
