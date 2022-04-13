use std::env::VarError;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn init_from_env() {
    let home_dir = match env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => get_current_dir(),
    };
    create_config_dir(home_dir);
}

fn get_current_dir() -> String {
    let current_dir = env::current_dir().unwrap();
    current_dir.into_os_string().into_string().unwrap()
}

fn create_config_dir(home_dir: String) {
    let config_dir = &String::from(".wave");
    let config_path = Path::new(&home_dir).join(Path::new(config_dir));
    match config_path.exists() {
        true => {}
        false => create_dir(config_path),
    }
}

fn create_dir(path: PathBuf) {
    fs::create_dir_all(path).expect("create config dir error");
}
