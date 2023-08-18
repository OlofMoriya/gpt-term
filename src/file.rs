use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn get_path() -> PathBuf {
    let home_dir: PathBuf = match env::var_os("HOME") {
        Some(home) => home.into(),
        None => {
            println!("Error: could not determine home directory.");
            std::process::exit(2);
        }
    };
    return home_dir.join("chat/");
}

pub fn load_data(context: &str) -> Vec<String> {
    let file_name = format!("{}-abr", context);
    let file_path = get_path().join(file_name);
    let input = fs::read(file_path);
    match input {
        Ok(input) => {
            let text = String::from_utf8_lossy(&input);
            let lines = text.split("\n§!§\n");
            return lines
                .filter_map(|l| {
                    if l == "" {
                        return None;
                    }

                    return Some(l.to_string());
                })
                .collect();
        }
        Err(_) => vec![],
    }
}

pub fn append_log(question: &String, answer: &String, context: &String) {
    let file_name = format!("{}-history.md", context);
    let file_path = get_path().join(file_name);
    if !file_path.as_path().exists() {
        // Create the file if it doesn't exist
        let result = File::create(file_path.clone());
        println!("\n");
        println!("creating file: {}, {:?}", file_path.display(), result);
        println!("\n");
    }

    let data = vec![format!("> {}", question), answer.clone()];
    let to_save: String = data.join("\n\n");

    let fappend = fs::OpenOptions::new().append(true).open(file_path);

    match fappend {
        Ok(mut open_file) => write!(open_file, "{}\n§!§\n", to_save).unwrap(),
        Err(e) => println!("error when appending to file: {}", e),
    };
}

pub fn append_data(message: String, context: String) {
    let file_name = format!("{}-abr", context);
    let file_path = get_path().join(file_name);
    if !file_path.as_path().exists() {
        // Create the file if it doesn't exist
        let result = File::create(file_path.clone());
        println!("{}, {:?}", file_path.display(), result);
    }

    let data = vec![message];
    let to_save: String = data.join("§§§");

    let fappend = fs::OpenOptions::new().append(true).open(file_path);
    //.unwrap();

    match fappend {
        Ok(mut open_file) => write!(open_file, "{}\n§!§\n", to_save).unwrap(),
        Err(e) => println!("error when appending to file: {}", e),
    };
}
