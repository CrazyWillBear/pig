mod install;
mod errors;
mod text;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command: &str = args[1].as_str();

    if args.len() < 3 {
        errors::err(1);
    }

    match command {
        "install" => {
            install::clone_package(args[2].as_str());
            install::install_package(args[2].as_str())
        }
        _ => {

        }
    }
}
