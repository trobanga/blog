use std::{fs::File, io::Read, io::Write};

use comrak::{format_html, parse_document, Arena, ComrakOptions};

#[no_mangle]
pub fn template_file() {
    println!("Content-Type: text/html\n");

    let data = get_parameters();

    let mut reg = Handlebars::new();
    let template_path = format!("{}/template.hbs", std::env::var("TEMPLATE_PATH").unwrap());
    reg.register_template_file("template", template_path)
        .unwrap();
    println!("{}", reg.render("template", &json!(data)).unwrap());
}

fn main() {
    let mut f = File::open("README.md").unwrap();
    let mut md = String::new();
    f.read_to_string(&mut md).unwrap();

    let arena = Arena::new();
    let root = parse_document(&arena, &md, &ComrakOptions::default());
    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();

    let mut out = File::create("README.html").unwrap();

    out.write_all(b"<html>\n<body>\n").unwrap();
    out.write_all(&html).unwrap();
    out.write_all(b"</body>\n<html>\n").unwrap();
}
