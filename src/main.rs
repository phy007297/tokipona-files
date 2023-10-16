use serde_json::Value;
use std::fs;

fn check_json(path: &str, content: &str) -> bool {
	match serde_json::from_str::<Value>(&content) {
		Ok(_) => {
			println!("[OK] {}", path);
			true
		},
		Err(err) => {
			println!("[ERR] {}\n{:?}", &path, err);
			false
		}
	}
}

fn check_file(path: &str) -> bool {
	match fs::read_to_string(&path) {
		Ok(content) => {
			if path.ends_with(".json") {
				check_json(&path, &content)
			}

			else if path.ends_with(".md") {
				true
			}

			else {
				true
			}
		}

		Err(err) => {
			println!("[ERR] {}\n {:?}", &path, err);
			false
		}
	}
}

fn check_folder(directory: &str) {
	let paths = fs::read_dir(&directory).unwrap();
	for path in paths {
		let name = path.unwrap().path().display().to_string();
		let meta = fs::metadata(&name).unwrap();

		if meta.is_file() {
			match check_file(&name) {
				true => continue,
				false => break,
			}
		}

		if meta.is_dir() {
			check_folder(&name);
		}
	}
}


fn main() {
	check_folder("files/");
}
