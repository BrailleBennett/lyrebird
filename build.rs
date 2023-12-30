use std::process::{Command, exit};
use std::env;

const YTDLP_VERSION: &str = "2023.12.30";

fn main() {
    if env::var("TARGET").unwrap() != env::var("HOST").unwrap() {
        println!("cargo:warning=Cross compiling is not supported");
        exit(1);
    }
    println!("cargo:rerun-if-changed=build.rs");
    let clone_dir = format!("{}/yt-dlp", env::var("OUT_DIR").unwrap());
    let venv = format!("{}/venv", env::var("OUT_DIR").unwrap());
    Command::new("git")
        .args(&["clone", "https://github.com/yt-dlp/yt-dlp", &clone_dir, "-b", &format!("{YTDLP_VERSION}"), "--depth", "1"])
        .status().unwrap();
    Command::new("python3")
        .args(&["-m", "venv", &venv, "--upgrade-deps"])
        .status().unwrap();
    Command::new(format!("{venv}/bin/python3"))
        .args(&["-m", "pip", "install", "pyinstaller", "-r", &format!("{clone_dir}/requirements.txt")])
        .status().unwrap();
    Command::new(format!("{venv}/bin/python3"))
        .args(&["pyinst.py", "--name", "yt-dlp"])
        .current_dir(clone_dir)
        .status().unwrap();
}

