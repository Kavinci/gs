use std::path::PathBuf;
use std::env;
extern crate num_cpus;

pub fn get_root() -> PathBuf {
    env::current_dir().unwrap()
    //String::from("/")
}

pub fn get_port() -> String {
    let port = "5000";
    let env_port = env::var("GS_PORT");
    String::from(port)
}

pub fn get_threads() -> usize {
    num_cpus::get() * 2
}