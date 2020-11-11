mod io;
mod helper;
mod parser;
mod kubernetes;

pub fn build(path: &str) {
    let content = io::read::templates::read_templates(path).unwrap();
    parser::worker::prepare(content);
}