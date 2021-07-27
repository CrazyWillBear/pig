use spinners;

use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

pub fn title_loading(text: &str) {
    let sp = Spinner::new(&Spinners::Line, format!("\n>> {}", text));
}

pub fn title(text: &str) {
    println!(">> {}", text)
}


pub fn subtitle_loading(text: &str) {
    let sp = Spinner::new(&Spinners::Line, format!("\n     > {}", text));
}

pub fn subtitle(text: &str) {
    println!("     > {}", text)
}