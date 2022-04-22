use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    io::{self, Read},
    path::PathBuf,
};

use comrak::{format_html, parse_document, Arena, ComrakOptions};

const CONTENT_DIR: &str = "./content";
const OUT_DIR: &str = "./dist";

type ProjectName = String;
type ArticlePathBuf = PathBuf;
type Projects = HashMap<ProjectName, Vec<ArticlePathBuf>>;

fn projects(path: &str) -> Result<Projects, io::Error> {
    let mut projects = Projects::new();
    for dir in fs::read_dir(path)? {
        let project = dir?.path();
        if project.is_dir() {
            let project_name = project.to_string_lossy().to_string();
            let mut articles = project.into_os_string();
            articles.push("/**/*.md");
            let mut articles = glob::glob(articles.to_str().unwrap())
                .unwrap()
                .filter_map(Result::ok)
                .collect::<Vec<PathBuf>>();
            alphanumeric_sort::sort_path_slice(&mut articles);
            projects.insert(project_name, articles);
        }
    }
    Ok(projects)
}

///
fn md_to_html(path: PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut f = File::open(path)?;
    let mut md = String::new();
    f.read_to_string(&mut md)?;

    let arena = Arena::new();
    let root = parse_document(&arena, &md, &ComrakOptions::default());
    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html)?;
    Ok(html)
}

fn main() -> io::Result<()> {
    let mut out = File::create("blog.html").unwrap();
    out.write_all(b"<html>\n<body>\n").unwrap();

    let projects = projects(CONTENT_DIR)?;
    for (_project, articles) in projects {
        fs::create_dir_all(OUT_DIR)?;
        for article in articles {
            let article = md_to_html(article)?;

            out.write_all(&article).unwrap();
        }
    }

    out.write_all(b"</body>\n<html>\n").unwrap();

    Ok(())
}
