use crate::configuration;
use crate::server;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn parse_uri(uri: String) -> (PathBuf, bool) {
    let mut path = PathBuf::from("/");
    let reg = Regex::new(r"$[/file/]||$[/reference/]");
    if uri == "/".to_owned() {
        let configuration: configuration::Configuration = configuration::Configuration::new();
        path = PathBuf::from(configuration.theme);
        path.push("index.html");
        return (path, true);
    }
    (path, true)
}

pub fn get_metadata() -> HashMap<String, String> {
    HashMap::default()
}
