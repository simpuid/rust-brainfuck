mod args;
mod files;
mod token;
use args::*;
use files::*;
use std::process;
use token::*;

fn main() {
    let config = extract(get_config(), "Argument Error");

    let mut files: Vec<File> = Vec::new();
    for name in &config.files {
        files.push(extract(get_file(&name), "IO Error"));
    }

    let mut token_program: Vec<TokenProgram> = Vec::new();
    for f in files {
        token_program.push(extract(parse(&f.data), "Parse Error"));
    }
}

fn extract<T>(data: Result<T, String>, err: &str) -> T {
    match data {
        Ok(d) => return d,
        Err(e) => {
            println!("{}: {}", err, e);
            process::exit(1);
        }
    }
}
