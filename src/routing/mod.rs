use std::path::PathBuf;
use std::env;

mod utils;
mod objects;
mod references;

pub struct WebPath {

}

pub async fn route(req: HttpRequest) -> Result<NamedFile> {
    let mut path: PathBuf = req.match_info().query("filepath").parse().unwrap();
    if !path.exists() {
        path = get_theme_route(path);
    }
    //println!("{}", path.display());
    Ok(NamedFile::open(path)?)
}

fn get_theme_route(path: PathBuf) -> PathBuf {
    let mut root_path: PathBuf = env::current_dir().unwrap();
    root_path.push("themes");
    root_path.push(utils::get_theme());
    root_path.push("dist");
    root_path = root_path.join(path.as_path());
        if !root_path.is_file() {
            root_path.push("index.html");
        }
    root_path
}

pub async fn references_route(req: HttpRequest) -> impl Responder {
    let path: String = req.path().to_string();
    //println!("test: {}", path);
    references::get_fs_structure(path)
}

