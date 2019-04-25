use std::fs;

pub struct File {
    pub name: String,
    pub data: String,
}

pub fn get_file(name: &String) -> Result<File, String> {
    let data = fs::read_to_string(name);
    match data {
        Ok(s) => {
            return Ok(File {
                name: name.clone(),
                data: s,
            });
        }
        Err(_) => return Err(format!("Cannot read file {}", name)),
    }
}
