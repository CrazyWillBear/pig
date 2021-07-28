mod install;
mod errors;
mod text;

use users;
use std::process::Command;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command: &str = args[1].as_str();
    let user = users::get_user_by_uid(users::get_current_uid()).unwrap();

    if args.len() < 3 {
        errors::err(1);
    }

    match command {
        "install" => {
            install::clone_package(args[2].as_str());

            // check for Makefile
            text::title_loading("Checking for Makefile...");
            if Path::new(&format!("/home/{}/.cache/pig/{}/Makefile", user.name().to_str().unwrap(), args[2].replace("/", "-"))).exists() {
                text::title("Makefile exists");
                install::install_make_package(args[2].as_str())
            }

            // check for CMake
            text::title_loading("Checking for CMake...");
            let output = &Command::new("make")
                .arg("--version")
                .output()
                .expect("failed to execute process");
            if output.status.to_string() != "exit status: 0" {
                errors::err(3);
            }

            if Path::new(&format!("/home/{}/.cache/pig/{}/CMakeLists.txt", user.name().to_str().unwrap(), args[2].replace("/", "-"))).exists() {
                text::title("CMakeLists exists");
                install::install_cmake_package(args[2].as_str())
            }
        }
        _ => {

        }
    }
}
