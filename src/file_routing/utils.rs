use std::env;

pub fn get_theme() -> String {
    let mut theme = get_environment_variable("THEME");
    if !theme.0 {
        theme.1 = "default".to_string();
    }
    theme.1
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