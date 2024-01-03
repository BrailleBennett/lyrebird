use tokio::sync::OnceCell;
use tempfile::tempdir;
use std::fs::{File, set_permissions, Permissions};
use std::io::Write;
use std::mem;
use std::os::unix::fs::PermissionsExt;

const YTDLP: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/yt-dlp"));

static FILE: OnceCell<String> = OnceCell::const_new();

pub async fn ytdlp() -> &'static str {
    &FILE.get_or_init(|| async {
        let dir = tempdir().unwrap();
        let path = dir.path().join("yt-dlp");
        let mut file = File::create(&path).unwrap();
        file.write_all(YTDLP).unwrap();
        set_permissions(&path, Permissions::from_mode(0o500)).unwrap();
        mem::forget(dir);
        path.display().to_string()
    }).await
}
