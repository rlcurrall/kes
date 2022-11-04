use std::{collections::HashMap, env::current_dir, ffi::OsStr, path::PathBuf};

use glob::glob;
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;

use crate::templates::Templates;

#[derive(Clone, Serialize)]
pub struct PostItem {
    pub key: String,
    pub title: String,
}

#[derive(Clone)]
pub struct PostManager {
    post_list: Vec<PostItem>,
    render_cache: HashMap<String, String>,
}

impl PostManager {
    pub fn new(post_dir: String, templates: &Templates) -> Self {
        let mut render_cache = HashMap::new();
        let mut post_list = vec![];

        for post in collect_post_paths(&post_dir) {
            let html = get_post_html(&post, templates);
            render_cache.insert(get_post_key(&post), html);
            post_list.push(PostItem {
                key: get_post_key(&post),
                title: get_post_title(&post),
            });
        }

        Self {
            post_list,
            render_cache,
        }
    }

    pub fn get(&self, post: &String) -> Option<String> {
        self.render_cache.get(post).map(|s| s.to_string())
    }

    pub fn get_post_list(&self) -> Vec<PostItem> {
        self.post_list.clone()
    }
}

fn collect_post_paths(post_dir: &str) -> Vec<PathBuf> {
    let root_path = PathBuf::from("/");
    let cur_dir = current_dir().unwrap_or(root_path);
    let abs_dir = cur_dir.to_str().unwrap_or("/");
    let post_path = format!("{abs_dir}/{post_dir}");

    if !PathBuf::from(&post_path).is_dir() {
        tracing::warn!("Invalid posts directory provided: {}", post_path);
        return vec![];
    }

    let paths_res = glob(format!("{post_path}/*.md").as_str());

    match paths_res {
        Err(err) => {
            tracing::error!("error getting post file paths: {err}");
            vec![]
        }
        Ok(paths) => paths
            .into_iter()
            .filter_map(|f| f.ok())
            .filter_map(|f| match f.exists() {
                true => Some(f),
                false => None,
            })
            .collect(),
    }
}

fn get_post_title(path: &PathBuf) -> String {
    get_post_key(path)
        .replace("-", " ")
        .replace("_", " ")
        .split(" ")
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn get_post_key(path: &PathBuf) -> String {
    let os_str = OsStr::new("");
    path.file_name()
        .unwrap_or(&os_str)
        .to_str()
        .unwrap_or("")
        .trim_end_matches(".md")
        .to_string()
}

fn get_post_html(post: &PathBuf, tmpl: &Templates) -> String {
    match std::fs::read_to_string(post) {
        Err(_) => "".to_string(),
        Ok(markdown) => {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&markdown, options);
            let mut html_output: String = String::with_capacity(markdown.len() * 3 / 2);
            html::push_html(&mut html_output, parser);

            let post_title = get_post_title(post);
            tmpl.render_post(post_title, html_output)
        }
    }
}
