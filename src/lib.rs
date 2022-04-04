use log::info;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::theme;
use std::fs;
use std::str;

pub mod pagetoc_lib {
    use super::*;

    pub struct PagetocPreprocessor;

    impl PagetocPreprocessor {
        pub fn new() -> PagetocPreprocessor {
            PagetocPreprocessor
        }
    }

    impl Preprocessor for PagetocPreprocessor {
        fn name(&self) -> &str {
            "mdbook-pagetoc"
        }

        fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
            env_logger::init_from_env(
                env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
            );
            let html_config = ctx.config.html_config().unwrap_or_default();

            let pagetoc_js = include_str!("pagetoc.js");
            let pagetoc_css = include_str!("pagetoc.css");
            let theme_dir = match html_config.theme {
                Some(ref theme) => ctx.root.join(theme),
                None => ctx.root.join("theme"),
            };

            let theme_index =
                String::from_utf8(theme::Theme::new(theme_dir.as_path()).index).unwrap();
            let anchor_tag = "<main>";
            let pagetoc_snippet = "<div class=\"sidetoc\"><nav class=\"pagetoc\"></nav></div>";
            let index_hbs = if !theme_index.contains(&pagetoc_snippet) {
                theme_index.replace(anchor_tag, &(format!("{}{}", anchor_tag, pagetoc_snippet)))
            } else {
                theme_index
            };
            fs::create_dir_all(theme_dir.as_path()).expect("Unable to create directory");
            for (file_name, contents) in [
                ("index.hbs", index_hbs.as_str()),
                ("pagetoc.js", pagetoc_js),
                ("pagetoc.css", pagetoc_css),
            ] {
                let file_path = theme_dir.join(file_name);
                if !file_path.exists() {
                    info!("{}: Writing {}", self.name(), file_path.display());
                    fs::write(file_path, contents).expect("Unable to write file");
                }
            }
            Ok(book)
        }

        fn supports_renderer(&self, renderer: &str) -> bool {
            renderer == "html"
        }
    }
}
