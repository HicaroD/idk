use clap::Parser as ClapParser;

pub enum TargetLanguage<'a> {
    C,
    JavaScript,
    Unknown(&'a str),
}

pub fn get_target_language(selected_language: &str) -> TargetLanguage {
    match selected_language {
        "C" => TargetLanguage::C,
        "JavaScript" => TargetLanguage::JavaScript,
        unknown_language => TargetLanguage::Unknown(unknown_language),
    }
}

#[derive(ClapParser, Debug)]
#[clap(author="Hícaro Dânrlley", version="0.1", about="A general purpose and open-source programming language", long_about = None)]
pub struct Args {
    /// File name
    #[clap(short = 'n', long = "name", value_parser)]
    pub file_name: String,

    /// Target language (C, JavaScript)
    #[clap(short = 't', long = "target", value_parser)]
    pub target_language: String,
}
