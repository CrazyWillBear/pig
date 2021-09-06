# Pig

## Summary
Pig (Package Installer for Git) is a Git repo installer. Essentially, you run `pig install CrazyWillBear/pig` and it will install it (not just clone, but check for how to compile and install and do so accordinly). It is in the VERY early stages of development and as of now can only install packages containing a Makefile with an install function included.

## Prerequisites
- A Linux machine
- A method of compiling Rust code

## How to install
- Compile the code. In the directory containing the Cargo.toml folder, run `cargo build --release`
- Move the resulting executable into /bin. In the directory containing the executable (/target/release) run `sudo mv pig /bin/pig`

Or run this: `git clone https://github.com/CrazyWillBear/pig; cd pig; cargo build --release; mv target/release/pig /bin/pig`

## How to use
Simply run `pig install <insert GitHub repo author/name>`
(ONLY WORKS ON GITHUB REPOS WITH MAKEFILES OR CMAKE)
