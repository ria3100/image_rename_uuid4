use std::{env, fs, path::Path};
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    let dir = Path::new(&path);

    let entries = fs::read_dir(dir).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "png" || ext == "jpg" {
                let new_name = dir.join(Uuid::new_v4().to_string() + "." + ext.to_str().unwrap());
                fs::rename(path, new_name).unwrap();
            }
        }
    }
}
