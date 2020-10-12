mod io;
mod helper;
mod parser;
mod kubernetes;

pub fn build(path: &str) {
    let content = io::read::templates::read_templates(path).unwrap();
    let templates = parser::template::parse_template(content.get(0).unwrap().to_owned());

    println!("{:?}", templates);
}