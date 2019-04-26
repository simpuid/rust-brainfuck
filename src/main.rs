mod args;
mod files;
mod memory;
mod program;
mod token;
use std::process;

fn main() {
    let config = extract(args::get_config(), "Argument Error");

    let mut files: Vec<files::File> = Vec::new();
    for name in &config.files {
        files.push(extract(files::get_file(&name), "IO Error"));
    }

    let mut token_program: Vec<token::TokenProgram> = Vec::new();
    for f in files {
        token_program.push(extract(token::parse(&f.data), "Parse Error"));
    }

    for mut p in token_program {
        let program = program::get_program(&mut p);
        let mut l = memory::Memory::new();
        program.run(&mut l);
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
