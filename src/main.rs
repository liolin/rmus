use dotenv::dotenv;
use std::env;
use std::{fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let music_dir = env::var("MUSIC_DIR").unwrap();
    let all_files = find_files(&music_dir)?
        .into_iter()
        .filter(|f| f.extension().unwrap() == "flac")
        .collect::<Vec<_>>();

    println!("{:?}", all_files);
    Ok(())
}

fn find_files(path: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for e in fs::read_dir(path)? {
        let path = e?.path();

        if path.is_dir() {
            files.append(&mut find_files(path.to_str().unwrap())?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}
