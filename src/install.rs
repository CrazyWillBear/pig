use crate::errors;
use crate::text;
use git2::Repository;
use std::io::{self, Write};
use users;
use std::process::{Command, Stdio};

pub fn clone_package(repo_name: &str) {
    let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
    let path: &str = &format!("/home/{}/.cache/pig/{}", user.name().to_str().unwrap(), repo_name.replace("/", "-"));

    text::title_loading("Checking if repository is already cloned...");
    if std::path::Path::new(path).exists() {
        text::title("Repository is already cloned");
        text::subtitle_loading("Repository exists, deleting...");
        std::fs::remove_dir_all(path);
        text::subtitle("Repository deleted");
    }

    text::title_loading("Cloning Git repo...");
    let url: &str = &format!("https://github.com/{}", repo_name);
    let _repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => {
            errors::err(2);
            panic!("failed to clone: {}", e);
        }
    };
    text::title("Git repo successfully cloned");
}

pub fn install_make_package(repo_name: &str) {
    let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
    let path: &str = &format!("/home/{}/.cache/pig/{}", user.name().to_str().unwrap(), repo_name.replace("/", "-"));

    text::subtitle_loading("Running Makefile...");
    println!("-=-=-=-=-");
    let output_install = Command::new("make")
        .current_dir(path)
        .args(["install", "--debug=v"])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output_install.stdout).unwrap();
    let output_str = std::str::from_utf8(&output_install.stdout);
    if output_str.unwrap().contains(&"File 'install' does not exist.") {
        let output = Command::new("make")
            .current_dir(path)
            .arg("--debug=v")
            .output()
            .expect("failed to execute process");
        io::stdout().write_all(&output.stdout).unwrap();
    }
    text::subtitle("Ran Makefile");
}

pub fn install_cmake_package (repo_name: &str) {
    let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
    let path: &str = &format!("/home/{}/.cache/pig/{}", user.name().to_str().unwrap(), repo_name.replace("/", "-"));

    text::subtitle_loading("Running CMake...\n");
    println!("-=-=-=-=-");

    let output_c_init = Command::new("cmake")
        .current_dir(format!("{}", path))
        .arg(".")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output_c_init.stdout).unwrap();

    let output_c = Command::new("cmake")
        .current_dir(format!("{}", path))
        .args(["--build", "."])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output_c.stdout).unwrap();


    let mut need_root = false;
    let output = Command::new("make")
        .current_dir(format!("{}", path))
        .arg("install")
        .output()
        .expect("failed to execute process");

    if String::from_utf8_lossy(&output.stderr).contains("Permission denied.") {
        need_root = true;
    }

    io::stdout().write_all(&output.stdout).unwrap();

    if need_root {
        let sudo_cmd = Command::new("sudo")
            .current_dir(format!("{}", path))
            .args(["make", "install"])
            .output()
            .expect("failed to execute process");
        io::stdout().write_all(&sudo_cmd.stdout).unwrap();
    }

    text::subtitle("Ran CMake");
}