use std::io::prelude::*;
use std::fs::File;
use std::fs;
use whoami;

pub fn make_config() -> std::io::Result<()> {
    let user = whoami::username();
    let mut config = File::create(format!("/home/{}/.config/pig", user))?;

    config.write_all(b"https://github.com")?;
    Ok(())
}

pub fn get_repo_site() -> String {
    let user = whoami::username();
    fs::read_to_string(&format!("/home/{}/.config/pig", user)).unwrap()
}