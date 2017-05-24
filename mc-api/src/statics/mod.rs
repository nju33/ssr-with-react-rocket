use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open(Path::new("statics/index.html"))
    // let mut context = HashMap::<String, String>::new();
    // context.insert("title", "ssr-with-react-rocket");
    // Template::render("index", &context)
}

#[get("/statics/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("statics/").join(file)).ok()
    // NamedFile::open(Path::new(&file)).ok()
    // NamedFile::open(Path::new("templates/bundle.js"))
}
