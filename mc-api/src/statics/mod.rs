use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/statics/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/").join(file)).ok()
    // NamedFile::open(Path::new(&file)).ok()
    // NamedFile::open(Path::new("templates/bundle.js"))
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open(Path::new("statics/index.html"))
}

#[get("/<dir..>")]
pub fn dirs(dir: PathBuf) -> io::Result<NamedFile> {
    // let ext = dir.extension().unwrap()
    NamedFile::open(Path::new("statics/index.html"))
}
