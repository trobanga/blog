use std::{fs::File, io::Read, io::Write};

use comrak::{format_html, parse_document, Arena, ComrakOptions};

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
