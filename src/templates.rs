use std::{fs, path::PathBuf};

use tera::{Context, Tera};

use crate::posts::PostItem;

static DEFAULT_POST: &str = include_str!("../templates/post.html");
static DEFAULT_HOME: &str = include_str!("../templates/home.html");
static DEFAULT_404: &str = include_str!("../templates/404.html");

pub struct Templates {
    tera: Tera,
}

impl Templates {
    pub fn new(
        t_home_path: Option<PathBuf>,
        t_post_path: Option<PathBuf>,
        t_404_path: Option<PathBuf>,
    ) -> Self {
        let mut tera = Tera::default();

        let home_template = match t_home_path {
            None => DEFAULT_HOME.to_string(),
            Some(path) => match fs::read_to_string(path) {
                Ok(template) => template,
                Err(e) => {
                    tracing::error!("could not load template: {}", e);
                    DEFAULT_HOME.to_string()
                }
            },
        };
        tera.add_raw_template("home", &home_template).unwrap();

        let post_template = match t_post_path {
            None => DEFAULT_POST.to_string(),
            Some(path) => match fs::read_to_string(path) {
                Ok(template) => template,
                Err(e) => {
                    tracing::error!("could not load template: {}", e);
                    DEFAULT_POST.to_string()
                }
            },
        };
        tera.add_raw_template("post", &post_template).unwrap();

        let not_found_template = match t_404_path {
            None => DEFAULT_404.to_string(),
            Some(path) => match fs::read_to_string(path) {
                Ok(template) => template,
                Err(e) => {
                    tracing::error!("could not load template: {}", e);
                    DEFAULT_404.to_string()
                }
            },
        };
        tera.add_raw_template("404", &not_found_template).unwrap();

        Self { tera }
    }

    pub fn render_post(&self, title: String, content: String) -> String {
        let mut ctx = Context::new();
        ctx.insert("title", &title);
        ctx.insert("body", &content);
        self.tera.render("post", &ctx).unwrap()
    }

    pub fn render_home(&self, post_list: Vec<PostItem>) -> String {
        let mut ctx = Context::new();
        ctx.insert("posts", &post_list);
        self.tera.render("home", &ctx).unwrap()
    }

    pub fn render_404(&self) -> String {
        let ctx = Context::new();
        self.tera.render("404", &ctx).unwrap()
    }
}
