use std::{env, fs, path::Path};
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    let dir = Path::new(&path);

    let entries = fs::read_dir(dir).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let from = entry.path();

        if let Some(ext) = from.extension() {
            if ext == "png" || ext == "jpg" {
                let to = dir.join(Uuid::new_v4().to_string() + "." + ext.to_str().unwrap());
                let from_file_name = from.file_name().unwrap().to_str().unwrap();
                let to_file_name = to.file_name().unwrap().to_str().unwrap();
                println!("{} -> {}", from_file_name, to_file_name);
                fs::rename(from, to).unwrap();
            }
        }
    }
}
