use std::env;

pub struct Config {
    pub files: Vec<String>,
}

pub fn get_config() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let mut files: Vec<String> = Vec::new();
    for s in &args[1..] {
        files.push(s.clone());
    }
    if files.len() < 1 {
        return Err(format!("less number of arguments."));
    } else {
        return Ok(Config { files: files });
    }
}
