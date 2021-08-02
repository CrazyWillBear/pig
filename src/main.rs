mod install;
mod errors;
mod text;
mod config;

use whoami;
use std::process::Command;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user = whoami::username();

    if !Path::new(&format!("/home/{}/.config/pig", user)).exists() {
        config::make_config();
    }

    if args.len() < 3 {
        errors::err(1);
    }

    let command: &str = args[1].as_str();

    match command {
        "install" => {
            install::clone_package(args[2].as_str());

            // check for Makefile
            text::title_loading("Checking for Makefile...");
            if Path::new(&format!("/home/{}/.cache/pig/{}/Makefile", user, args[2].replace("/", "-"))).exists() {
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

            if Path::new(&format!("/home/{}/.cache/pig/{}/CMakeLists.txt", user, args[2].replace("/", "-"))).exists() {
                text::title("CMakeLists exists");
                install::install_cmake_package(args[2].as_str())
            }

            text::title_loading("Checking for Install.sh script...");
            if Path::new(&format!("/home/{}/.cache/pig/{}/Install.sh", user, args[2].replace("/", "-"))).exists() {
                text::title("Install.sh exists");
                install::install_sh_package(args[2].as_str(), "Install.sh")
            }

            if Path::new(&format!("/home/{}/.cache/pig/{}/install.sh", user, args[2].replace("/", "-"))).exists() {
                text::title("Install.sh exists");
                install::install_sh_package(args[2].as_str(), "install.sh")
            }
        }
        _ => {

        }
    }
}