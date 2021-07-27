pub fn err(code: i32) {
    let msg: &str;
    match code {
        1 => msg = "Invalid arguments provided",
        2 => msg = "Could not clone Git repository (it may not exist)",
        _ => msg = "Unknown error"
    };
    eprintln!("ERROR:\n{} (Error code: {})", msg, code);
    std::process::exit(code);
}