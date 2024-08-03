use std::env;
use std::fs::write;
use reqwest::blocking::get;

const YTDLP_VERSION: &str = "2024.08.01";

fn main() {
    if env::var("CARGO_CFG_UNIX").is_err() {
        println!("cargo:warning=Only unix targets are supported");
    }
    println!("cargo:rerun-if-changed=build.rs");
    write(
        format!("{}/yt-dlp", env::var("OUT_DIR").unwrap()),
        get(format!("https://github.com/yt-dlp/yt-dlp/releases/download/{YTDLP_VERSION}/yt-dlp")).unwrap()
            .error_for_status().unwrap()
            .bytes().unwrap(),
    ).unwrap();
}
