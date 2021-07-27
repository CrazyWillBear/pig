use crate::errors;
use crate::text;
use git2::Repository;
use ::users;

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

pub fn install_package(repo_name: &str) {
    let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
    let path: &str = &format!("/home/{}/.cache/pig/{}", user.name().to_str().unwrap(), repo_name.replace("/", "-"));

    text::title_loading("Checking for Makefile...");
    if std::path::Path::new(&format!("{}/Makefile", path)).exists() {
        text::title("Makefile exists");
        text::subtitle_loading("Running `make install`...");
        std::process::Command::new("make")
            .current_dir(path)
            .args(["install"])
            .stdout(std::process::Stdio::null())
            .spawn();
        text::subtitle("Ran `make install`");
    }
}