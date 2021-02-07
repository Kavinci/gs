extern crate num_cpus;
use std::path::PathBuf;
use std::env;

pub struct Configuration {
    pub root: PathBuf,
    pub port: String,
    pub threads: usize,
    pub theme: PathBuf
}

impl Configuration {
    //#![feature(const_fn)]
    pub fn new() -> Configuration {
        Configuration {
            root: get_root(),
            port: get_port(),
            threads: get_threads(),
            theme: get_theme()
        }
    }
}

fn get_root() -> PathBuf {
    let mut root_path = PathBuf::from(env::current_dir().unwrap());
    let root = get_environment_variable("GS_ROOT");
    if root.0 {
        root_path = PathBuf::from(root.1);
    }
    println!("Root Path: {}", root_path.to_string_lossy());
    root_path
}

fn get_port() -> String {
    let mut port = get_environment_variable("GS_PORT");
    if !port.0 {
        port.1 = String::from("5000");
    }
    println!("Port: {}", port.1);
    port.1
}

fn get_threads() -> usize {
    let mut threads = get_environment_variable("GS_THREADS");
    if !threads.0 {
        let max = num_cpus::get() * 2;
        threads.1 = max.to_string();
    }
    println!("Threads: {}", threads.1.parse::<usize>().unwrap());
    threads.1.parse::<usize>().unwrap()
}

fn get_theme() -> PathBuf {
    let mut theme_path = PathBuf::from(env::current_dir().unwrap());
    theme_path.push("themes");
    let mut theme = get_environment_variable("GS_THEME");
    if !theme.0 {
        theme.1 = "default".to_string();
    }
    theme_path.push(theme.1);
    println!("Theme: {}", theme_path.to_string_lossy());
    theme_path
}

fn get_environment_variable(match_key: &str) -> (bool, String) {
    let mut found = false;
    let mut value: String = String::default(); 
    for (_key, _value) in env::vars(){
        //println!("{}={}", _key, _value);
        if _key == match_key {
            found = true;
            value = _value;
        }
    }
    (found, value)
}
