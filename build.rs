use std::process::{Command, exit, Stdio};
use std::env;
use std::io::Write;
use reqwest::blocking::get;

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
        .args(&["-m", "venv", &venv, "--without-pip"])
        .status().unwrap();
    let get_pip = get("https://bootstrap.pypa.io/get-pip.py").unwrap().text().unwrap();
    let mut run_get_pip = Command::new(format!("{venv}/bin/python3"))
        .stdin(Stdio::piped())
        .spawn().unwrap();
    run_get_pip.stdin.take().unwrap().write_all(&get_pip.bytes().collect::<Vec<_>>()).unwrap();
    run_get_pip.wait().unwrap();
    Command::new(format!("{venv}/bin/python3"))
        .args(&["-m", "pip", "install", "pyinstaller", "-r", &format!("{clone_dir}/requirements.txt")])
        .status().unwrap();
    Command::new(format!("{venv}/bin/python3"))
        .args(&["pyinst.py", "--name", "yt-dlp"])
        .current_dir(clone_dir)
        .status().unwrap();
}

